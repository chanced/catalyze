use ahash::{HashMapExt, HashSet};
use itertools::Itertools;
use protobuf::{
    descriptor::{
        field_descriptor_proto, DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto,
        FieldDescriptorProto, FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto,
        ServiceDescriptorProto, SourceCodeInfo,
    },
    MessageField,
};
use snafu::{Backtrace, ResultExt};
use std::{path::PathBuf, str::FromStr};

use crate::{
    error::{self, Error, GroupNotSupported, HydrationCtx, HydrationFailed, TypeNotFound},
    HashMap,
};

use super::{
    container, dependency, enum_, enum_value, extension, extension_decl,
    field::{self, ValueInner},
    file, location, message,
    method::{self, IoInner},
    node::{self, NodeMap},
    oneof, package, reference, service, Ast, FullyQualifiedName, Name,
};

pub(super) fn run(descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Ast, Error> {
    Hydrator::run(descriptors, targets)
}

struct Hydrator {
    ast: Ast,
}

impl Hydrator {
    fn run(descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Ast, Error> {
        let mut hydrator = Hydrator {
            ast: Ast::new(descriptors.len()),
        };
        for descriptor in descriptors {
            let file_path = PathBuf::from(descriptor.name());
            let file = hydrator
                .hydrate_file(descriptor, targets)
                .with_context(|_| HydrationCtx {
                    file_path: file_path.clone(),
                })?;
            hydrator.ast.files_by_path.insert(file_path, file.key);
            hydrator.ast.files_by_name.insert(file.name, file.key);
        }
        Ok(hydrator.ast)
    }

    fn hydrate_package(
        &mut self,
        package: Option<String>,
    ) -> (Option<package::Key>, Option<FullyQualifiedName>) {
        let Some(name) = package else {
            return (None, None);
        };
        let fqn = FullyQualifiedName::for_package(name);
        let key = self.ast.packages.get_or_insert_key(fqn.clone());
        (Some(key), Some(fqn))
    }

    fn hydrate_file(
        &mut self,
        descriptor: FileDescriptorProto,
        targets: &[String],
    ) -> Result<file::Ident, HydrationFailed> {
        let FileDescriptorProto {
            name,
            package,
            dependency,
            public_dependency: public_dependencies,
            weak_dependency: weak_dependencies,
            message_type,
            enum_type,
            service,
            extension,
            options,
            source_code_info,
            syntax,
            special_fields,
        } = descriptor;
        let (package, package_fqn) = self.hydrate_package(package);
        let name: Name = name.unwrap().into();

        let fqn = FullyQualifiedName::new(&name, package_fqn);
        let key = self.ast.files.get_or_insert_key(fqn.clone());
        let location = file_location(source_code_info)?;
        let mut nodes = HashMap::with_capacity(location.node_count);
        self.ast.reserve(location.node_count);

        let mut all_references = Vec::new();

        let messages = self.hydrate_messages(
            message_type,
            location.messages,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
            &mut all_references,
        )?;

        let enums = self.hydrate_enums(
            enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let services = self.hydrate_services(
            service,
            location.services,
            fqn.clone(),
            key,
            package,
            &mut nodes,
            &mut all_references,
        )?;

        let HydratedExtensions {
            extension_decls,
            extensions,
            ext_refs: ext_references,
        } = self.hydrate_extensions(
            extension,
            location.extensions,
            key.into(),
            fqn,
            key,
            package,
            &mut nodes,
        )?;

        let dependencies = self.hydrate_dependencies(key, dependency)?;
        let is_build_target = targets
            .iter()
            .any(|target| target.as_str() == name.as_str());

        let file = self.ast.files[key].hydrate(file::Hydrate {
            name,
            package,
            messages,
            enums,
            services,
            all_references,
            ext_references,
            extensions,
            extension_decls,
            dependencies,
            is_build_target,
            syntax,
            options,
            nodes,
            package_comments: location.package,
            comments: location.syntax,
            public_dependencies,
            weak_dependencies,
            special_fields,
        })?;
        Ok(file)
    }

    fn hydrate_messages(
        &mut self,
        descriptors: Vec<DescriptorProto>,
        locations: Vec<location::Message>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
        transitive_refs: &mut Vec<reference::Inner>,
    ) -> Result<Vec<message::Ident>, HydrationFailed> {
        assert_locations("message", &locations, &descriptors)?;

        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
                self.hydrate_message(
                    Message {
                        descriptor,
                        fqn,
                        location,
                        container,
                        file,
                        package,
                    },
                    nodes,
                    transitive_refs,
                )
            })
            .collect()
    }

    fn hydrate_message(
        &mut self,
        message: Message,
        nodes: &mut NodeMap,
        transitive_refs: &mut Vec<reference::Inner>,
    ) -> Result<message::Ident, HydrationFailed> {
        let parent_refs = transitive_refs;
        let mut all_refs = Vec::new();

        let Message {
            descriptor,
            fqn,
            location,
            container,
            file,
            package,
        } = message;

        let name: Name = descriptor.name.unwrap_or_default().into();
        let key = self.ast.messages.get_or_insert_key(fqn.clone());

        let enums = self.hydrate_enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        )?;

        let messages = self.hydrate_messages(
            descriptor.nested_type,
            location.messages,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
            &mut all_refs,
        )?;
        let oneofs = self.hydrate_oneofs(
            descriptor.oneof_decl,
            location.oneofs,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
        )?;
        let HydratedFields { fields, field_refs } = self.hydrate_fields(
            descriptor.field,
            location.fields,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
            &oneofs,
        )?;

        let HydratedExtensions {
            extension_decls,
            extensions,
            mut ext_refs,
        } = self.hydrate_extensions(
            descriptor.extension,
            location.extensions,
            key.into(),
            fqn,
            file,
            package,
            nodes,
        )?;

        let well_known = if self.is_well_known(package) {
            message::WellKnownMessage::from_str(&name).ok()
        } else {
            None
        };
        parent_refs.extend(all_refs.iter().copied());

        let mut references = field_refs;

        references.append(&mut ext_refs);

        let msg = self.ast.messages[key].hydrate(message::Hydrate {
            name,
            location: location.detail,
            all_refs,
            references,
            container,
            package,
            enums,
            messages,
            fields,
            oneofs,
            extensions,
            extension_decls,
            well_known,
            options: descriptor.options,
            reserved_ranges: descriptor.reserved_range,
            reserved_names: descriptor.reserved_name,
            extension_range: descriptor.extension_range,
            special_fields: descriptor.special_fields,
        })?;
        nodes.insert(msg.fqn(), msg.node_key());
        self.ast.nodes.insert(msg.fqn(), msg.node_key());
        Ok(msg)
    }

    fn hydrate_fields(
        &mut self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::Field>,
        message: message::Key,
        message_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
        oneofs: &[oneof::Ident],
    ) -> Result<HydratedFields, HydrationFailed> {
        assert_locations("field", &locations, &descriptors)?;
        let mut fields = Vec::with_capacity(descriptors.len());
        let mut references = Vec::new();
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(message_fqn.clone()));
            let oneof =
                oneof_for_field(&descriptor, oneofs).with_context(|_| error::OneofIndexCtx {
                    field_fqn: fqn.clone(),
                })?;

            let (field, reference) = self.hydrate_field(
                Field {
                    descriptor,
                    fqn,
                    location,
                    message,
                    file,
                    package,
                    oneof,
                },
                &mut references,
                nodes,
            )?;
            fields.push(field);
            if let Some(reference) = reference {
                references.push(reference);
            }
        }
        Ok(HydratedFields {
            fields,
            field_refs: references,
        })
    }
    fn hydrate_field_enum_value(
        &mut self,
        type_name: String,
        field_fqn: &FullyQualifiedName,
    ) -> Result<field::ValueInner, error::EmptyTypeName> {
        todo!()
    }
    fn hydrate_field_message_value(
        &mut self,
        type_name: String,
        field_fqn: &FullyQualifiedName,
    ) -> Result<field::ValueInner, error::EmptyTypeName> {
        todo!()
    }
    fn hydrate_field_value(
        &mut self,
        proto_type: field_descriptor_proto::Type,
        type_name: String,
        field_fqn: &FullyQualifiedName,
    ) -> Result<field::ValueInner, HydrationFailed> {
        use field_descriptor_proto::Type as ProtoType;
        match proto_type {
            ProtoType::TYPE_ENUM => self
                .hydrate_field_enum_value(type_name, field_fqn)
                .with_context(|_| error::EmptyTypeNameCtx {
                    field_fqn: field_fqn.clone(),
                    type_not_found: error::TypeNotFound::Enum,
                }),
            ProtoType::TYPE_MESSAGE => self
                .hydrate_field_message_value(type_name, field_fqn)
                .with_context(|_| error::EmptyTypeNameCtx {
                    field_fqn: field_fqn.clone(),
                    type_not_found: error::TypeNotFound::Message,
                }),
            ProtoType::TYPE_GROUP => Err(GroupNotSupported {
                backtrace: Backtrace::capture(),
            })
            .with_context(|_| error::GroupNotSupportedCtx {
                field_fqn: field_fqn.clone(),
            }),
            _ => Ok(field::ValueInner::new(proto_type, None, None)),
        }
    }

    #[allow(clippy::unused_self)]
    fn hydrate_reference(&self, field: field::Key, value: ValueInner) -> Option<reference::Inner> {
        match value {
            ValueInner::Enum(key) => Some(reference::Inner {
                referent: reference::ReferentKey::Enum(key),
                referrer: reference::ReferrerKey::Field(field),
            }),
            ValueInner::Message(key) => Some(reference::Inner {
                referent: reference::ReferentKey::Message(key),
                referrer: reference::ReferrerKey::Field(field),
            }),
            _ => None,
        }
    }
    fn hydrate_field(
        &mut self,
        field: Field,
        references: &mut Vec<reference::Inner>,
        nodes: &mut NodeMap,
    ) -> Result<(field::Ident, Option<reference::Inner>), HydrationFailed> {
        let Field {
            descriptor,
            fqn,
            location,
            message,
            file,
            package,
            oneof,
        } = field;

        let key = self.ast.fields.get_or_insert_key(fqn.clone());
        let FieldDescriptorProto {
            name,
            number,
            label,
            type_,
            type_name,
            extendee: _, // not needed - used for extensions
            default_value,
            json_name,
            oneof_index,
            options,
            proto3_optional,
            special_fields,
        } = descriptor;

        let name: Name = name.unwrap_or_default().into();

        let proto_type = type_
            .unwrap()
            .enum_value()
            .map_err(|type_| error::UnknownFieldType {
                backtrace: Backtrace::capture(),
                type_,
            })
            .with_context(|_| error::UnknownFieldTypeCtx {
                field_fqn: fqn.clone(),
            })?;

        let value =
            self.hydrate_field_value(proto_type, type_name.clone().unwrap_or_default(), &fqn)?;
        let reference = self.hydrate_reference(key, value);

        if let Some(reference) = reference {
            references.push(reference);
        }

        let field = self.ast.fields[key].hydrate(field::Hydrate {
            name,
            location: location.detail,
            number,
            value,
            label,
            type_,
            proto_type,
            type_name,
            default_value,
            json_name,
            oneof_index,
            options,
            proto3_optional,
            special_fields,
            message,
            file,
            package,
            reference,
        })?;
        if let Some(oneof) = oneof {
            self.ast.oneofs[oneof].add_field(field.key);
        }

        let field = self.insert_node(nodes, field)?;
        Ok((field, reference))
    }

    fn hydrate_dependencies(
        &mut self,
        dependent: file::Key,
        dependencies: Vec<String>,
    ) -> Result<Vec<dependency::Inner>, HydrationFailed> {
        let direct_dependencies = dependencies
            .into_iter()
            .map(|dependency| {
                let fqn = FullyQualifiedName(dependency.into());
                let dependency_file = self.ast.files.get_or_insert_key(fqn.clone());
                dependency::Inner {
                    dependent,
                    dependency: dependency_file,
                }
            })
            .collect_vec();
        Ok(direct_dependencies)
    }

    fn hydrate_enums(
        &mut self,
        descriptors: Vec<EnumDescriptorProto>,
        locations: Vec<location::Enum>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<enum_::Ident>, HydrationFailed> {
        assert_locations("enum", &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.hydrate_enum(
                    Enum {
                        fqn: FullyQualifiedName::new(
                            descriptor.name(),
                            Some(container_fqn.clone()),
                        ),
                        descriptor,
                        location,
                        file,
                        package,
                        container,
                    },
                    nodes,
                )
            })
            .collect()
    }

    fn hydrate_enum(
        &mut self,
        enum_: Enum,
        nodes: &mut NodeMap,
    ) -> Result<enum_::Ident, HydrationFailed> {
        let Enum {
            descriptor,
            fqn,
            location,
            container,
            file,
            package,
        } = enum_;
        let key = self.ast.enums.get_or_insert_key(fqn.clone());
        let name: Name = descriptor.name.unwrap_or_default().into();
        let values = self.hydrate_enum_values(
            descriptor.value,
            location.values,
            key,
            fqn,
            file,
            package,
            nodes,
        )?;
        let well_known = if self.is_well_known(package) {
            enum_::WellKnownEnum::from_str(&name).ok()
        } else {
            None
        };
        let enum_ = self.ast.enums[key].hydrate(enum_::Hydrate {
            name,
            well_known,
            file,
            location: location.detail,
            container,
            package,
            values,
            options: descriptor.options,
            reserved_ranges: descriptor.reserved_range,
            reserved_names: descriptor.reserved_name,
            special_fields: descriptor.special_fields,
        })?;

        nodes.insert(enum_.fqn(), enum_.node_key());
        self.ast.nodes.insert(enum_.fqn(), enum_.node_key());
        Ok(enum_)
    }

    fn hydrate_oneofs(
        &mut self,
        descriptors: Vec<OneofDescriptorProto>,
        locations: Vec<location::Oneof>,
        message: message::Key,
        message_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<oneof::Ident>, HydrationFailed> {
        assert_locations("oneof", &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(message_fqn.clone()));
                self.hydrate_oneof(
                    Oneof {
                        descriptor,
                        fqn,
                        location,
                        message,
                        file,
                        package,
                    },
                    nodes,
                )
            })
            .collect()
    }

    fn hydrate_oneof(
        &mut self,
        oneof: Oneof,
        nodes: &mut NodeMap,
    ) -> Result<super::node::Ident<oneof::Key>, HydrationFailed> {
        let Oneof {
            descriptor,
            fqn,
            location,
            message,
            file,
            package,
        } = oneof;

        let OneofDescriptorProto {
            name,
            options,
            special_fields,
        } = descriptor;

        let key = self.ast.oneofs.get_or_insert_key(fqn.clone());
        let name: Name = name.unwrap_or_default().into();
        let oneof = self.ast.oneofs[key].hydrate(oneof::Hydrate {
            fields: Vec::default(),
            name,
            location: location.detail,
            options,
            special_fields,
            message,
            file,
            package,
        })?;
        nodes.insert(oneof.fqn(), oneof.node_key());
        self.ast.nodes.insert(oneof.fqn(), oneof.node_key());
        todo!()
    }

    fn hydrate_extensions(
        &self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::ExtensionDecl>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<HydratedExtensions, HydrationFailed> {
        todo!()
    }

    fn is_well_known(&self, package: Option<package::Key>) -> bool {
        let Some(package) = package else { return false };
        self.ast.well_known == package
    }

    fn hydrate_services(
        &mut self,
        descriptors: Vec<ServiceDescriptorProto>,
        locations: Vec<location::Service>,
        file_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
        references: &mut Vec<reference::Inner>,
    ) -> Result<Vec<service::Ident>, HydrationFailed> {
        assert_locations("service", &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(file_fqn.clone()));
                self.hydrate_service(
                    Service {
                        descriptor,
                        fqn,
                        location,
                        file,
                        package,
                    },
                    nodes,
                    references,
                )
            })
            .collect()
    }

    fn hydrate_service(
        &mut self,
        service: Service,
        nodes: &mut NodeMap,
        all_references: &mut Vec<reference::Inner>,
    ) -> Result<service::Ident, HydrationFailed> {
        let Service {
            descriptor,
            fqn,
            location,
            file,
            package,
        } = service;
        let ServiceDescriptorProto {
            name,
            options,
            method,
            special_fields,
        } = descriptor;
        let mut references = Vec::with_capacity(method.len() * 2);
        let name = name.unwrap_or_default().into();
        let key = self.ast.services.get_or_insert_key(fqn.clone());
        let methods = self.hydrate_methods(
            method,
            location.methods,
            key,
            fqn,
            file,
            package,
            nodes,
            &mut references,
        )?;

        all_references.extend(references.iter().copied());

        let service = self.ast.services[key].hydrate(service::Hydrate {
            name,
            location: location.detail,
            methods,
            special_fields,
            file,
            package,
            options,
            references,
        })?;
        nodes.insert(service.fqn(), service.node_key());
        self.ast.nodes.insert(service.fqn(), service.node_key());
        Ok(service)
    }

    fn hydrate_methods(
        &mut self,
        descriptors: Vec<MethodDescriptorProto>,
        locations: Vec<location::Method>,
        service: service::Key,
        service_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
        references: &mut Vec<reference::Inner>,
    ) -> Result<Vec<method::Ident>, HydrationFailed> {
        assert_locations("method", &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.hydrate_method(
                    Method {
                        fqn: FullyQualifiedName::new(descriptor.name(), Some(service_fqn.clone())),
                        descriptor,
                        location,
                        file,
                        package,
                        service,
                    },
                    nodes,
                    references,
                )
            })
            .collect()
    }

    fn hydrate_method(
        &mut self,
        method: Method,
        nodes: &mut NodeMap,
        references: &mut Vec<reference::Inner>,
    ) -> Result<method::Ident, HydrationFailed> {
        let Method {
            fqn,
            descriptor,
            location,
            service,
            file,
            package,
        } = method;
        let MethodDescriptorProto {
            input_type,
            output_type,
            options,
            name,
            client_streaming,
            server_streaming,
            special_fields,
        } = descriptor;
        let name: Name = name.unwrap_or_default().into();
        let key = self.ast.methods.get_or_insert_key(fqn.clone());
        let input = input_type.ok_or_else(|| error::HydrationFailed::MethodMissingMessage {
            method_fqn: fqn.clone(),
            direction: method::Direction::Input,
        })?;
        let input = self
            .ast
            .messages
            .get_or_insert_key(FullyQualifiedName(input.into()));
        let output = output_type.ok_or_else(|| error::HydrationFailed::MethodMissingMessage {
            method_fqn: fqn.clone(),
            direction: method::Direction::Output,
        })?;

        let output = self
            .ast
            .messages
            .get_or_insert_key(FullyQualifiedName(output.into()));

        references.push(reference::Inner {
            referent: input.into(),
            referrer: key.into(),
        });
        references.push(reference::Inner {
            referent: output.into(),
            referrer: key.into(),
        });

        let io = method::IoInner::new(
            input,
            client_streaming.unwrap_or_default(),
            output,
            server_streaming.unwrap_or_default(),
        );

        let method = self.ast.methods[key].hydrate(method::Hydrate {
            name,
            service,
            file,
            package,
            location: location.detail,
            io,
            options,
            special_fields,
        })?;

        self.insert_node(nodes, method)
    }

    fn hydrate_enum_values(
        &mut self,
        descriptors: Vec<EnumValueDescriptorProto>,
        locations: Vec<location::EnumValue>,
        enum_: enum_::Key,
        enum_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<enum_value::Ident>, HydrationFailed> {
        assert_locations("enum values", &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.hydrate_enum_value(
                    EnumValue {
                        fqn: FullyQualifiedName::new(descriptor.name(), Some(enum_fqn.clone())),
                        descriptor,
                        location,
                        file,
                        package,
                        enum_,
                    },
                    nodes,
                )
            })
            .collect()
    }

    fn hydrate_enum_value(
        &mut self,
        enum_value: EnumValue,
        nodes: &mut NodeMap,
    ) -> Result<enum_value::Ident, HydrationFailed> {
        let EnumValue {
            descriptor,
            fqn,
            location,
            enum_,
            file,
            package,
        } = enum_value;
        let EnumValueDescriptorProto {
            name,
            number,
            options,
            special_fields,
        } = descriptor;

        let key = self.ast.enum_values.get_or_insert_key(fqn);
        let name: Name = name.unwrap_or_default().into();
        let number = number.unwrap();
        let enum_value = self.ast.enum_values[key].hydrate(enum_value::Hydrate {
            name,
            location: location.detail,
            number,
            options,
            special_fields,
            enum_,
            file,
            package,
        })?;

        self.insert_node(nodes, enum_value)
    }

    fn insert_node<K>(
        &mut self,
        nodes: &mut NodeMap,
        node: node::Ident<K>,
    ) -> Result<node::Ident<K>, HydrationFailed>
    where
        K: Copy + Into<node::Key>,
    {
        let key = node.node_key();
        self.ast.nodes.insert(node.fqn(), key);
        nodes.insert(node.fqn(), key);
        Ok(node)
    }
}

struct Message {
    descriptor: DescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Message,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Enum {
    descriptor: EnumDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Enum,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct EnumValue {
    descriptor: EnumValueDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::EnumValue,
    enum_: enum_::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Service {
    descriptor: ServiceDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Service,
    file: file::Key,
    package: Option<package::Key>,
}

struct Method {
    fqn: FullyQualifiedName,
    location: location::Method,
    descriptor: MethodDescriptorProto,
    service: service::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Field {
    descriptor: FieldDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Field,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
    oneof: Option<oneof::Key>,
}

struct Oneof {
    descriptor: OneofDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Oneof,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Extension {
    descriptor: FieldDescriptorProto,
    fqn: FullyQualifiedName,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct ExtensionDecl {
    location: location::ExtensionDecl,
    extensions: Vec<Extension>,
}

struct HydratedExtensions {
    extension_decls: Vec<extension_decl::Key>,
    extensions: Vec<extension::Ident>,
    ext_refs: Vec<reference::Inner>,
}
struct HydratedFields {
    fields: Vec<field::Ident>,
    field_refs: Vec<reference::Inner>,
}

fn assert_locations<T, L>(
    kind: &'static str,
    locations: &[L],
    descriptors: &[T],
) -> Result<(), error::LocationsMisaligned> {
    if locations.len() == descriptors.len() {
        Ok(())
    } else {
        Err(error::LocationsMisaligned {
            kind,
            expected: descriptors.len(),
            found: locations.len(),
            backtrace: Backtrace::capture(),
        })
    }
}

fn file_location(info: MessageField<SourceCodeInfo>) -> Result<location::File, HydrationFailed> {
    let info = info
        .0
        .ok_or_else(|| error::HydrationFailed::MissingSourceCodeInfo)?;
    location::File::new(*info)
}

fn oneof_for_field(
    descriptor: &FieldDescriptorProto,
    oneofs: &[oneof::Ident],
) -> Result<Option<oneof::Key>, error::InvalidIndex> {
    let Some(index) = descriptor.oneof_index else {
        return Ok(None);
    };
    let idx: usize = index.try_into().map_err(|_| error::InvalidIndex {
        backtrace: Backtrace::capture(),
        index,
    })?;

    let oneof_key = oneofs
        .get(idx)
        .ok_or_else(|| error::InvalidIndex {
            backtrace: Backtrace::capture(),
            index,
        })?
        .key;

    Ok(Some(oneof_key))
}
