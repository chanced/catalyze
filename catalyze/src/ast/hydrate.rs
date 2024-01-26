use ahash::HashMapExt;
use itertools::Itertools;
use protobuf::{
    descriptor::{
        DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
        FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
        SourceCodeInfo,
    },
    MessageField,
};
use std::{path::PathBuf, str::FromStr};

use crate::{as_i32, error::HydrateError, HashMap};

use super::{
    container, enum_, enum_value, extension, extension_decl, field,
    file::{self},
    location, message, method,
    node::NodeMap,
    oneof, package, service, Ast, FullyQualifiedName, Name,
};

pub(super) fn run(
    descriptors: Vec<FileDescriptorProto>,
    targets: &[String],
) -> Result<Ast, HydrateError> {
    Hydrator::run(descriptors, targets)
}

struct Hydrator {
    ast: Ast,
}

impl Hydrator {
    fn run(descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Ast, HydrateError> {
        let mut hydrator = Hydrator {
            ast: Ast::new(descriptors.len()),
        };

        for descriptor in descriptors {
            hydrator.file(descriptor, targets)?;
        }

        Ok(hydrator.ast)
    }

    fn file_location(
        &self,
        fqn: &Name,
        info: MessageField<SourceCodeInfo>,
    ) -> Result<location::File, HydrateError> {
        location::File::new(
            info.unwrap_or_else(|| panic!("File \"{fqn}\" is missing SourceCodeInfo")),
        )
    }

    fn messages(
        &mut self,
        descriptors: Vec<DescriptorProto>,
        locations: Vec<location::Message>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<message::Ident>, HydrateError> {
        assert_locations("message", &locations, &descriptors);
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
                self.message(
                    Message {
                        fqn,
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
    fn message(
        &mut self,
        message: Message,
        nodes: &mut NodeMap,
    ) -> Result<message::Ident, HydrateError> {
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
        let enums = self.enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        )?;
        let messages = self.messages(
            descriptor.nested_type,
            location.messages,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        )?;

        let fields = self.fields(
            descriptor.field,
            location.fields,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
        )?;
        let oneofs = self.oneofs(
            descriptor.oneof_decl,
            location.oneofs,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
        )?;
        let (extension_decls, extensions) = self.extensions(
            descriptor.extension,
            location.extensions,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        )?;

        let well_known = if self.is_well_known(package) {
            message::WellKnownMessage::from_str(&name).ok()
        } else {
            None
        };

        let msg = self.ast.messages[key].hydrate(message::Hydrate {
            name,
            location: location.detail,
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
    fn file(
        &mut self,
        descriptor: FileDescriptorProto,
        targets: &[String],
    ) -> Result<file::Ident, HydrateError> {
        let (package, package_fqn) = self.package(descriptor.package);
        let name: Name = descriptor.name.unwrap().into();
        let fqn = FullyQualifiedName::new(&name, package_fqn);
        let key = self.ast.files.get_or_insert_key(fqn.clone());
        let location = self.file_location(&name, descriptor.source_code_info)?;
        let mut nodes = HashMap::with_capacity(location.node_count);
        self.ast.reserve(location.node_count);

        let messages = self.messages(
            descriptor.message_type,
            location.messages,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let enums = self.enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let services = self.services(
            descriptor.service,
            location.services,
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let (extension_decls, extensions) = self.extensions(
            descriptor.extension,
            location.extensions,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let dependencies = self.dependencies(
            key,
            descriptor.dependency,
            descriptor.public_dependency,
            descriptor.weak_dependency,
        );

        let is_build_target = targets
            .iter()
            .any(|target| target.as_str() == name.as_str());

        let file = self.ast.files[key].hydrate(file::Hydrate {
            name: name.clone(),
            package,
            messages,
            enums,
            services,
            extensions,
            extension_decls,
            dependencies,
            is_build_target,
            syntax: descriptor.syntax,
            options: descriptor.options,
            nodes,
            package_comments: location.package,
            comments: location.syntax,
        })?;
        self.ast
            .files_by_path
            .insert(PathBuf::from(name.as_str()), key);
        self.ast.files_by_name.insert(name, key);
        Ok(file)
    }

    fn package(
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

    fn dependencies(
        &mut self,
        dependent: file::Key,
        dependencies: Vec<String>,
        public_dependencies: Vec<i32>,
        weak_dependencies: Vec<i32>,
    ) -> Result<file::DependenciesInner, HydrateError> {
        let mut direct = dependencies
            .into_iter()
            .map(|dependency| {
                let fqn = FullyQualifiedName(dependency.into());
                let dependency_file = self.ast.files.get_or_insert_key(fqn.clone());
                file::DependencyInner {
                    dependent,
                    dependency: dependency_file,
                    ..Default::default()
                }
            })
            .collect_vec();

        let mut public = Vec::with_capacity(public_dependencies.len());
        for idx in public_dependencies {
            let i: usize = idx.try_into().map_err(|_| {
                HydrateError::invalid_weak_dependency(
                    idx,
                    FullyQualifiedName::from(self.ast.files[dependent].path.as_path()),
                )
            })?;
            let dep = &mut direct.get_mut(i).ok_or_else(|| {
                HydrateError::invalid_weak_dependency(
                    idx,
                    FullyQualifiedName::from(self.ast.files[dependent].path.as_path()),
                )
            })?;

            dep.is_public = true;
            public.push(dep.clone());
        }
        Ok(file::DependenciesInner {
            transitive: Vec::default(),
            direct,
            public,
            weak,
            unusued: Vec::default(),
        })
    }
    fn enums(
        &mut self,
        descriptors: Vec<EnumDescriptorProto>,
        locations: Vec<location::Enum>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<enum_::Ident>, HydrateError> {
        assert_locations("enum", &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.enum_(
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

    fn enum_(&mut self, enum_: Enum, nodes: &mut NodeMap) -> Result<enum_::Ident, HydrateError> {
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
        let values = self.enum_values(
            descriptor.value,
            location.values,
            key,
            fqn.clone(),
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

    fn fields(
        &mut self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::Field>,
        message: message::Key,
        message_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<field::Ident>, HydrateError> {
        assert_locations("field", &locations, &descriptors);
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(message_fqn.clone()));
                self.field(
                    Field {
                        fqn,
                        message,
                        descriptor,
                        location,
                        file,
                        package,
                    },
                    nodes,
                )
            })
            .collect()
    }

    fn oneofs(
        &mut self,
        descriptors: Vec<OneofDescriptorProto>,
        locations: Vec<location::Oneof>,
        message: message::Key,
        message_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<oneof::Ident>, HydrateError> {
        assert_locations("oneof", &locations, &descriptors);
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(message_fqn.clone()));
                self.oneof(
                    Oneof {
                        message,
                        fqn,
                        descriptor,
                        location,
                        file,
                        package,
                    },
                    nodes,
                )
            })
            .collect()
    }

    fn extensions(
        &self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::ExtensionDecl>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<(Vec<extension_decl::Key>, Vec<extension::Ident>), HydrateError> {
        todo!()
    }

    fn is_well_known(&self, package: Option<package::Key>) -> bool {
        let Some(package) = package else { return false };
        self.ast.well_known == package
    }

    fn services(
        &mut self,
        descriptors: Vec<ServiceDescriptorProto>,
        locations: Vec<location::Service>,
        file_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<service::Ident>, HydrateError> {
        assert_locations("service", &locations, &descriptors);
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(file_fqn.clone()));
                self.service(
                    Service {
                        fqn,
                        descriptor,
                        location,
                        file,
                        package,
                    },
                    nodes,
                )
            })
            .collect()
    }

    fn service(
        &mut self,
        service: Service,
        nodes: &mut NodeMap,
    ) -> Result<service::Ident, HydrateError> {
        let Service {
            descriptor,
            fqn,
            location,
            file,
            package,
        } = service;
        let name = descriptor.name.unwrap_or_default().into();
        let key = self.ast.services.get_or_insert_key(fqn.clone());
        let methods = self.methods(
            descriptor.method,
            location.methods,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
        )?;
        let service = self.ast.services[key].hydrate(service::Hydrate {
            name,
            location: location.detail,
            methods,
            special_fields: descriptor.special_fields,
            file,
            package,
            options: descriptor.options,
        })?;
        nodes.insert(service.fqn(), service.node_key());
        self.ast.nodes.insert(service.fqn(), service.node_key());
        Ok(service)
    }

    fn methods(
        &mut self,
        descriptors: Vec<MethodDescriptorProto>,
        locations: Vec<location::Method>,
        service: service::Key,
        service_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<method::Ident>, HydrateError> {
        assert_locations("method", &locations, &descriptors);
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.method(
                    Method {
                        fqn: FullyQualifiedName::new(descriptor.name(), Some(service_fqn.clone())),
                        descriptor,
                        location,
                        file,
                        package,
                        service,
                    },
                    nodes,
                )
            })
            .collect()
    }
    fn field(&mut self, field: Field, nodes: &mut NodeMap) -> Result<field::Ident, HydrateError> {
        let Field {
            descriptor,
            fqn,
            location,
            message,
            file,
            package,
        } = field;

        let key = self.ast.fields.get_or_insert_key(fqn.clone());

        let FieldDescriptorProto {
            name,
            number,
            label,
            type_,
            type_name,
            extendee: _,
            default_value,
            json_name,
            oneof_index,
            options,
            proto3_optional,
            special_fields,
        } = descriptor;
        let name: Name = name.unwrap_or_default().into();
        let field = self.ast.fields[key].hydrate(field::Hydrate {
            name,
            location: location.detail,
            number,
            label,
            type_,
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
        })?;
        nodes.insert(field.fqn(), field.node_key());
        self.ast.nodes.insert(field.fqn(), field.node_key());
        Ok(field)
    }

    fn method(
        &mut self,
        method: Method,
        nodes: &mut NodeMap,
    ) -> Result<method::Ident, HydrateError> {
        let Method {
            fqn,
            descriptor,
            location,
            service,
            file,
            package,
        } = method;
        let name = descriptor.name.unwrap_or_default().into();
        let key = self.ast.methods.get_or_insert_key(fqn.clone());
        let method = self.ast.methods[key].hydrate(method::Hydrate {
            name,
            service,
            file,
            package,
            location: location.detail,
            input_type: descriptor.input_type,
            output_type: descriptor.output_type,
            client_streaming: descriptor.client_streaming,
            server_streaming: descriptor.server_streaming,
            options: descriptor.options,
        })?;
        nodes.insert(method.fqn(), method.node_key());
        self.ast.nodes.insert(method.fqn(), method.node_key());
        Ok(method)
    }

    fn enum_values(
        &mut self,
        descriptors: Vec<EnumValueDescriptorProto>,
        locations: Vec<location::EnumValue>,
        enum_: enum_::Key,
        enum_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut NodeMap,
    ) -> Result<Vec<enum_value::Ident>, HydrateError> {
        assert_locations("enum values", &locations, &descriptors);
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.enum_value(
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

    fn enum_value(
        &mut self,
        r#enum: EnumValue,
        nodes: &mut NodeMap,
    ) -> Result<enum_value::Ident, HydrateError> {
        let EnumValue {
            descriptor,
            fqn,
            location,
            enum_,
            file,
            package,
        } = r#enum;
        let EnumValueDescriptorProto {
            name,
            number,
            options,
            special_fields,
        } = descriptor;
        let key = self.ast.enum_values.get_or_insert_key(r#enum.fqn.clone());
        let name: Name = r#enum.descriptor.name.unwrap_or_default().into();
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
        nodes.insert(enum_value.fqn(), enum_value.node_key());
        self.ast
            .nodes
            .insert(enum_value.fqn(), enum_value.node_key());
        Ok(enum_value)
    }

    fn oneof(
        &self,
        oneof: Oneof,
        nodes: &mut NodeMap,
    ) -> Result<super::node::Ident<oneof::Key>, HydrateError> {
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
            fields: todo!(),
            name,
            location: location.detail,
            options,
            special_fields,
            message,
            file,
            package,
        })?;
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

fn assert_locations<T, L>(
    kind: &'static str,
    locations: &[L],
    descriptors: &[T],
) -> Result<(), HydrateError> {
    if locations.len() != descriptors.len() {
        Err(HydrateError::invalid_locations(
            kind,
            descriptors.len(),
            locations.len(),
        ))
    } else {
        Ok(())
    }
}
