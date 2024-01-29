use ahash::HashMapExt;
use itertools::Itertools;
use protobuf::{
    descriptor::{
        self, field_descriptor_proto, DescriptorProto, EnumDescriptorProto,
        EnumValueDescriptorProto, FieldDescriptorProto, FileDescriptorProto, MethodDescriptorProto,
        OneofDescriptorProto, ServiceDescriptorProto, SourceCodeInfo,
    },
    EnumOrUnknown, MessageField,
};
use snafu::{Backtrace, ResultExt};
use std::{iter::Peekable, path::PathBuf, str::FromStr, vec::IntoIter};

use crate::{
    error::{self, Error, GroupNotSupported, HydrationCtx, HydrationFailed, InvalidMapKey},
    HashMap,
};

use super::{
    container, dependency, enum_, enum_value, extension, extension_decl, field::{self, TypeInner}, file, location, message, method, node, oneof, package, reference, resolve::Get, service, value::{self, MapKey}, Ast, FullyQualifiedName, Name
};

pub(super) fn run(descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Ast, Error> {
    Hydrator::run(descriptors, targets)
}

struct Hydrator {
    ast: Ast,
}

impl Hydrator {
    fn run(descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Ast, Error> {
        let mut hydrator = Self {
            ast: Ast::new(descriptors.len()),
        };
        hydrator.hydrate_files(descriptors, targets)?;
        Ok(hydrator.ast)
    }
    fn hydrate_files(
        &mut self,
        descriptors: Vec<FileDescriptorProto>,
        targets: &[String],
    ) -> Result<(), Error> {
        for descriptor in descriptors {
            let file_path = PathBuf::from(descriptor.name());
            let file = self
                .hydrate_file(descriptor, targets)
                .with_context(|_| HydrationCtx {
                    file_path: file_path.clone(),
                })?;
            self.ast.files_by_path.insert(file_path, file.key);
            self.ast.files_by_name.insert(file.name, file.key);
        }
        Ok(())
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

        let messages = self.hydrate_messages(HydrateMessages {
            descriptors: message_type,
            locations: location.messages,
            container: container::Key::File(key),
            container_fqn: fqn.clone(),
            file: key,
            package,
            nodes: &mut nodes,
            ancestor_refs: &mut all_references,
        })?;

        let enums = self.hydrate_enums(HydrateEnums {
            descriptors: enum_type,
            locations: location.enums,
            container: key.into(),
            container_fqn: fqn.clone(),
            file: key,
            package,
            nodes: &mut nodes,
        })?;

        let services = self.hydrate_services(HydrateServices {
            descriptors: service,
            locations: location.services,
            file_fqn: fqn.clone(),
            file: key,
            package,
            nodes: &mut nodes,
            file_refs: &mut all_references,
        })?;

        let ExtensionsHydrated {
            extension_decls,
            extensions,
            ext_refs: ext_references,
        } = self.hydrate_extensions(HydrateExtensions {
            ext_descriptors: extension,
            decl_locations: location.extensions,
            container: key.into(),
            container_fqn: fqn,
            file: key,
            package,
            nodes: &mut nodes,
        })?;

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
        messages: HydrateMessages,
    ) -> Result<Vec<message::Ident>, HydrationFailed> {
        let HydrateMessages {
            descriptors,
            locations,
            container,
            container_fqn,
            file,
            package,
            nodes,
            ancestor_refs,
        } = messages;
        validate_locations(
            &container_fqn,
            location::Kind::Message,
            &locations,
            &descriptors,
        )?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
                self.hydrate_message(HydrateMessage {
                    descriptor,
                    fqn,
                    location,
                    container,
                    file,
                    package,
                    nodes,
                    ancestor_refs,
                })
            })
            .collect()
    }

    fn hydrate_message(
        &mut self,
        message: HydrateMessage,
    ) -> Result<message::Ident, HydrationFailed> {
        let HydrateMessage {
            descriptor,
            fqn,
            location,
            container,
            file,
            package,
            nodes,
            ancestor_refs,
        } = message;
        let mut all_refs = Vec::new();
        let name: Name = descriptor.name.unwrap_or_default().into();
        let key = self.ast.messages.get_or_insert_key(fqn.clone());

        let enums = self.hydrate_enums(HydrateEnums {
            descriptors: descriptor.enum_type,
            locations: location.enums,
            container: key.into(),
            container_fqn: fqn.clone(),
            file,
            package,
            nodes,
        })?;
        let messages = self.hydrate_messages(HydrateMessages {
            descriptors: descriptor.nested_type,
            locations: location.messages,
            container: key.into(),
            container_fqn: fqn.clone(),
            ancestor_refs: &mut all_refs,
            file,
            package,
            nodes,
        })?;
        let oneofs = self.hydrate_oneofs(HydrateOneofs {
            descriptors: descriptor.oneof_decl,
            locations: location.oneofs,
            message: key,
            message_fqn: fqn.clone(),
            file,
            package,
            nodes,
        })?;
        let FieldsHydrated { fields, field_refs } = self.hydrate_fields(HydrateFields {
            descriptors: descriptor.field,
            locations: location.fields,
            message: key,
            message_fqn: fqn.clone(),
            file,
            package,
            nodes,
            oneofs: &oneofs,
        })?;

        let ExtensionsHydrated {
            extension_decls,
            extensions,
            mut ext_refs,
        } = self.hydrate_extensions(HydrateExtensions {
            container: key.into(),
            container_fqn: fqn.clone(),
            decl_locations: location.extensions,
            ext_descriptors: descriptor.extension,
            file,
            package,
            nodes,
        })?;

        let well_known = if self.is_well_known(package) {
            message::WellKnownMessage::from_str(&name).ok()
        } else {
            None
        };
        let mut references = field_refs;
        references.append(&mut ext_refs);
        ancestor_refs.extend(all_refs.iter().copied());

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
        self.insert_node(nodes, msg)
    }

    fn hydrate_fields(&mut self, fields: HydrateFields) -> Result<FieldsHydrated, HydrationFailed> {
        let HydrateFields {
            descriptors,
            locations,
            message,
            message_fqn,
            file,
            package,
            nodes,
            oneofs,
        } = fields;
        validate_locations(
            &message_fqn,
            location::Kind::Field,
            &locations,
            &descriptors,
        )?;
        let mut fields = Vec::with_capacity(descriptors.len());
        let mut references = Vec::new();
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(message_fqn.clone()));
            let oneof =
                oneof_for_field(&descriptor, oneofs).with_context(|_| error::OneofIndexCtx {
                    field_fqn: fqn.clone(),
                })?;

            let (field, reference) = self.hydrate_field(HydrateField {
                descriptor,
                fqn,
                location,
                message,
                file,
                package,
                oneof,
                references: &mut references,
                nodes,
            })?;
            fields.push(field);
            if let Some(reference) = reference {
                references.push(reference);
            }
        }
        Ok(FieldsHydrated {
            fields,
            field_refs: references,
        })
    }

    fn hydrate_field(
        &mut self,
        field: HydrateField,
    ) -> Result<(field::Ident, Option<reference::Inner>), HydrationFailed> {
        let HydrateField {
            descriptor,
            fqn,
            location,
            message,
            file,
            package,
            oneof,
            references,
            nodes,
        } = field;

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
        let key = self.ast.fields.get_or_insert_key(fqn.clone());

        let reference = self.hydrate_reference(key.into(), value);

        if let Some(reference) = reference {
            references.push(reference);
        }
        let type_ = type_.unwrap();

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
            reference,
        })?;
        if let Some(oneof) = oneof {
            self.ast.oneofs[oneof].add_field(field.key);
        }

        let field = self.insert_node(nodes, field)?;
        Ok((field, reference))
    }
    fn hydrate_value_enum(
        &mut self,
        type_name: String,
    ) -> Result<value::Inner, error::EmptyTypeName> {
        if type_name.is_empty() {
            return Err(error::EmptyTypeName {
                backtrace: Backtrace::capture(),
                type_not_found: error::TypeNotFound::Enum,
            });
        }
        let fqn = FullyQualifiedName(type_name.into());
        let key = self.ast.enums.get_or_insert_key(fqn);
        Ok(value::Inner::Enum(key))
    }
    fn hydrate_value_message(
        &mut self,
        type_name: String,
    ) -> Result<value::Inner, error::EmptyTypeName> {
        if type_name.is_empty() {
            return Err(error::EmptyTypeName {
                backtrace: Backtrace::capture(),
                type_not_found: error::TypeNotFound::Message,
            });
        }
        let fqn = FullyQualifiedName(type_name.into());
        let key = self.ast.messages.get_or_insert_key(fqn);
        Ok(value::Inner::Message(key))
    }
    fn hydrate_field_value(
        &mut self,
        proto_type: field_descriptor_proto::Type,
        type_name: String,
        field_fqn: &FullyQualifiedName,
    ) -> Result<value::Inner, HydrationFailed> {
        use field_descriptor_proto::Type as ProtoType;
        match proto_type {
            ProtoType::TYPE_ENUM => {
                self.hydrate_value_enum(type_name)
                    .with_context(|_| error::EmptyTypeNameCtx {
                        field_fqn: field_fqn.clone(),
                    })
            }
            ProtoType::TYPE_MESSAGE => {
                self.hydrate_value_message(type_name)
                    .with_context(|_| error::EmptyTypeNameCtx {
                        field_fqn: field_fqn.clone(),
                    })
            }
            ProtoType::TYPE_GROUP => Err(GroupNotSupported {
                backtrace: Backtrace::capture(),
            })
            .with_context(|_| error::GroupNotSupportedCtx {
                field_fqn: field_fqn.clone(),
            }),
            _ => Ok(value::Inner::new(proto_type, None, None)),
        }
    }

    #[allow(clippy::unused_self)]
    fn hydrate_reference(
        &self,
        referrer: reference::ReferrerKey,
        type_: TypeInner,
    ) -> Option<reference::Inner> {

        let value = match type_ {
            TypeInner::Single(val) => val,
            TypeInner::Repeated(val) => val,
            TypeInner::Map(map) => map.value,
        };
        let referent = match value {
            value::Inner::Enum(key) => reference::ReferentKey::Enum(key),
            value::Inner::Message(key) => reference::ReferentKey::Message(key),
            _ => return None,
        };
        Some(reference::Inner { referrer, referent })
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
                let dependency = self.ast.files.get_or_insert_key(fqn);
                dependency::Inner {
                    dependent,
                    dependency,
                }
            })
            .collect_vec();
        Ok(direct_dependencies)
    }
    fn hydrate_value_type_map(&mut self, key: message::Key) -> Result<field::TypeInner, InvalidMapKey>{
        let map = self.ast.get(key);
        let map_key = self.ast.fields.get(map.fields.get(0).unwrap()).unwrap();
        let map_value = self.ast.fields.get(map.fields.get(1).unwrap()).unwrap();
        let map_key = MapKey::try_from(map_key.type_)?;

        match map_value.type_ {
            field::TypeInner::Single(val) => {
                Ok(field::TypeInner::Map(value::MapInner{
                    key: map_key,
                    value: val
                }))
            },
            _ => unreachable!()
        }
    }
    fn hydrate_value_type_message(&mut self, 
        fqn: FullyQualifiedName,
        is_repeated: bool
    ) -> Result<field::TypeInner, InvalidMapKey>{
        let msg = self.ast.messages.get_or_insert(fqn);
        // TODO: what if the message is not yet hydrated?
        // as of right now, i dont think it is possible for a message
        // to be both a map -and- not already hydrated.
        if msg.is_map_entry{
            return self.hydrate_value_type_map(msg.key);
        }
        if is_repeated {
            Ok(field::TypeInner::Repeated(value::Inner::Message(msg.key)))
        } else {
            Ok(field::TypeInner::Single(value::SingleInner::Message(msg.key)))
        }
    }

    fn hydrate_value_type(
        &mut self,
        type_: Option<EnumOrUnknown<field_descriptor_proto::Type>>,
        is_repeated: bool,
        type_name: Option<String>,
        container_fqn: &FullyQualifiedName,
    ) -> Result<field::Type, HydrationFailed> {
        use field_descriptor_proto::Type as ProtoType;
        let proto_type = type_
            .unwrap()
            .enum_value()
            .map_err(|type_| error::UnknownFieldType {
                backtrace: Backtrace::capture(),
                type_,
            })
            .with_context(|_| error::UnknownFieldTypeCtx {
                field_fqn: container_fqn.clone(),
            })?;

        let extract_type_name = |type_name: Option<String>| {
            type_name
                .ok_or(error::EmptyTypeName {
                    backtrace: Backtrace::capture(),
                    type_not_found: error::TypeNotFound::Message,
                })
                .with_context(|_| error::EmptyTypeNameCtx {
                    field_fqn: container_fqn.clone(),
                })
                .map(|type_name| FullyQualifiedName(type_name.into()))
        };
        match proto_type {
            ProtoType::TYPE_MESSAGE => {
                let type_name = extract_type_name(type_name)?;
                let type_ = self.hydrate_value_type_message(type_name, is_repeated);
                Ok(type_)
            },
            ProtoType::TYPE_ENUM => {
                let typ = self
                .hydrate_value_enum(extract_type_name(type_name)?)
                .with_context(|_| error::EmptyTypeNameCtx {
                    field_fqn: container_fqn.clone(),
                })?;
                Ok(typ)
            },
            ProtoType::TYPE_GROUP => Err(error::GroupNotSupported {
                backtrace: Backtrace::capture(),
            })
            .with_context(|_| error::GroupNotSupportedCtx {
                field_fqn: container_fqn.clone(),
            }),
            _ => 
        }
        let value = self.hydrate_field_value(
            proto_type,
            type_name.clone().unwrap_or_default(),
            &container_fqn,
        )?;
    }

    fn hydrate_enums(&mut self, enums: HydrateEnums) -> Result<Vec<enum_::Ident>, HydrationFailed> {
        let HydrateEnums {
            descriptors,
            locations,
            container,
            container_fqn,
            file,
            package,
            nodes,
        } = enums;
        validate_locations(
            &container_fqn,
            location::Kind::Enum,
            &locations,
            &descriptors,
        )?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
                self.hydrate_enum(HydrateEnum {
                    fqn,
                    descriptor,
                    location,
                    file,
                    package,
                    container,
                    nodes,
                })
            })
            .collect()
    }

    fn hydrate_enum(&mut self, enum_: HydrateEnum) -> Result<enum_::Ident, HydrationFailed> {
        let HydrateEnum {
            descriptor,
            fqn,
            location,
            container,
            file,
            package,
            nodes,
        } = enum_;
        let key = self.ast.enums.get_or_insert_key(fqn.clone());
        let name: Name = descriptor.name.unwrap_or_default().into();
        let values = self.hydrate_enum_values(HydrateEnumValues {
            descriptors: descriptor.value,
            locations: location.values,
            enum_: key,
            enum_fqn: fqn.clone(),
            file,
            package,
            nodes,
        })?;
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
        oneofs: HydrateOneofs,
    ) -> Result<Vec<oneof::Ident>, HydrationFailed> {
        use location::Kind;
        let HydrateOneofs {
            descriptors,
            locations,
            message,
            message_fqn,
            file,
            package,
            nodes,
        } = oneofs;
        validate_locations(&message_fqn, Kind::Oneof, &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(message_fqn.clone()));
                self.hydrate_oneof(HydrateOneof {
                    descriptor,
                    fqn,
                    location,
                    message,
                    file,
                    package,
                    nodes,
                })
            })
            .collect()
    }

    fn hydrate_oneof(
        &mut self,
        oneof: HydrateOneof,
    ) -> Result<super::node::Ident<oneof::Key>, HydrationFailed> {
        let HydrateOneof {
            descriptor,
            fqn,
            location,
            message,
            file,
            package,
            nodes,
        } = oneof;

        let OneofDescriptorProto {
            name,
            options,
            special_fields,
        } = descriptor;

        let key = self.ast.oneofs.get_or_insert_key(fqn);
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
        self.insert_node(nodes, oneof)
    }

    fn hydrate_extensions(
        &mut self,
        extensions: HydrateExtensions,
    ) -> Result<ExtensionsHydrated, HydrationFailed> {
        let HydrateExtensions {
            ext_descriptors,
            decl_locations,
            container,
            container_fqn,
            file,
            package,
            nodes,
        } = extensions;
        let mut hydrated = ExtensionsHydrated::new(&decl_locations, &ext_descriptors);
        let mut descriptors = ext_descriptors.into_iter().peekable();

        for decl_loc in decl_locations {
            validate_extension_locations(&descriptors, &decl_loc.extensions).with_context(
                |_| error::LocationsMisalignedCtx {
                    container_fqn: container_fqn.clone(),
                },
            )?;
            let descriptors = descriptors.take(decl_loc.extensions.len());
            let ext_decl = self.hydrate_extension_decl(HydrateExtensionDecl {
                descriptors,
                location: decl_loc,
                container,
                container_fqn: container_fqn.clone(),
                file,
                package,
                nodes,
            })?;
            hydrated.push(ext_decl)
        }
        Ok(hydrated)
    }

    fn hydrate_extension_decl<D>(
        &mut self,
        ext_decl: HydrateExtensionDecl<D>,
    ) -> Result<ExtensionDeclHydrated, HydrationFailed>
    where
        D: ExactSizeIterator<Item = FieldDescriptorProto>,
    {
        let HydrateExtensionDecl {
            descriptors,
            location,
            container,
            container_fqn,
            file,
            package,
            nodes,
        } = ext_decl;
        let inner = extension_decl::Inner::new(location.detail, file, package, descriptors.len());
        let key = inner.key();
        let mut hydrated = ExtensionDeclHydrated::new(key, descriptors.len());
        self.ast.extension_decls.push(inner);
        for (descriptor, location) in descriptors.zip(location.extensions) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let ExtensionHydrated {
                extension,
                reference,
                ext_decl,
            } = self.hydrate_extension(HydrateExtension {
                container: container.into(),
                ext_decl: key,
                descriptor,
                file,
                package,
                location,
                nodes,
                fqn,
            })?;
            if let Some(reference) = reference {
                hydrated.ext_refs.push(reference);
            }
            hydrated.extensions.push(extension);
        }
        Ok(hydrated)
    }

    fn hydrate_extension(
        &mut self,
        extension: HydrateExtension,
    ) -> Result<ExtensionHydrated, HydrationFailed> {
        let HydrateExtension {
            descriptor,
            fqn,
            location,
            container,
            file,
            package,
            nodes,
            ext_decl,
        } = extension;
        let FieldDescriptorProto {
            name,
            number,
            label,
            type_,
            type_name,
            extendee,
            default_value,
            json_name,
            options,
            proto3_optional,
            special_fields,
            oneof_index: _, // not needed - extension fields cant be part of a oneof
        } = descriptor;

        let name: Name = name.unwrap_or_default().into();
        let key = self.ast.extensions.get_or_insert_key(fqn.clone());

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

        let reference = self.hydrate_reference(key.into(), value);
        let extendee = self
            .ast
            .messages
            .get_or_insert_key(FullyQualifiedName(extendee.unwrap().into()));

        let extension = self.ast.extensions[key].hydrate(extension::Hydrate {
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
            options,
            proto3_optional,
            special_fields,
            container,
            extendee,
            file,
            package,
            reference,
        })?;

        let extension = self.insert_node(nodes, extension)?;
        Ok(ExtensionHydrated {
            ext_decl,
            extension,
            reference,
        })
    }

    fn is_well_known(&self, package: Option<package::Key>) -> bool {
        let Some(package) = package else { return false };
        self.ast.well_known == package
    }

    fn hydrate_services(
        &mut self,
        services: HydrateServices,
    ) -> Result<Vec<service::Ident>, HydrationFailed> {
        let HydrateServices {
            descriptors,
            locations,
            file_fqn,
            file,
            package,
            nodes,
            file_refs: references,
        } = services;
        validate_locations(&file_fqn, location::Kind::Service, &locations, &descriptors)?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                let fqn = FullyQualifiedName::new(descriptor.name(), Some(file_fqn.clone()));
                self.hydrate_service(HydrateService {
                    descriptor,
                    fqn,
                    location,
                    file,
                    package,
                    nodes,
                    ancestor_refs: references,
                })
            })
            .collect()
    }

    fn hydrate_service(
        &mut self,
        service: HydrateService,
    ) -> Result<service::Ident, HydrationFailed> {
        let HydrateService {
            descriptor,
            fqn,
            location,
            file,
            package,
            ancestor_refs,
            nodes,
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
        let methods = self.hydrate_methods(HydrateMethods {
            descriptors: method,
            locations: location.methods,
            service: key,
            service_fqn: fqn.clone(),
            file,
            package,
            nodes,
            service_refs: &mut references,
        })?;

        ancestor_refs.extend(references.iter().copied());

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
        self.insert_node(nodes, service)
    }

    fn hydrate_methods(
        &mut self,
        methods: HydrateMethods,
    ) -> Result<Vec<method::Ident>, HydrationFailed> {
        let HydrateMethods {
            descriptors,
            locations,
            service,
            service_fqn,
            file,
            package,
            nodes,
            service_refs,
        } = methods;
        validate_locations(
            &service_fqn,
            location::Kind::Method,
            &locations,
            &descriptors,
        )?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.hydrate_method(
                    HydrateMethod {
                        fqn: FullyQualifiedName::new(descriptor.name(), Some(service_fqn.clone())),
                        descriptor,
                        location,
                        file,
                        package,
                        service,
                    },
                    nodes,
                    service_refs,
                )
            })
            .collect()
    }

    fn hydrate_method(
        &mut self,
        method: HydrateMethod,
        nodes: &mut node::Map,
        references: &mut Vec<reference::Inner>,
    ) -> Result<method::Ident, HydrationFailed> {
        let HydrateMethod {
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
            referrer: (key.into(), method::Direction::Input).into(),
        });
        references.push(reference::Inner {
            referent: output.into(),
            referrer: (key.into(), method::Direction::Output).into(),
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
        enum_values: HydrateEnumValues,
    ) -> Result<Vec<enum_value::Ident>, HydrationFailed> {
        let HydrateEnumValues {
            descriptors,
            locations,
            enum_,
            enum_fqn,
            file,
            package,
            nodes,
        } = enum_values;
        validate_locations(
            &enum_fqn,
            location::Kind::EnumValue,
            &locations,
            &descriptors,
        )?;
        descriptors
            .into_iter()
            .zip(locations)
            .map(|(descriptor, location)| {
                self.hydrate_enum_value(HydrateEnumValue {
                    fqn: FullyQualifiedName::new(descriptor.name(), Some(enum_fqn.clone())),
                    descriptor,
                    location,
                    file,
                    package,
                    enum_,
                    nodes,
                })
            })
            .collect()
    }

    fn hydrate_enum_value(
        &mut self,
        enum_value: HydrateEnumValue,
    ) -> Result<enum_value::Ident, HydrationFailed> {
        let HydrateEnumValue {
            descriptor,
            fqn,
            location,
            enum_,
            file,
            package,
            nodes,
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
        nodes: &mut node::Map,
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

struct HydrateMessage<'h> {
    descriptor: DescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Message,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
    ancestor_refs: &'h mut Vec<reference::Inner>,
}
struct HydrateMessages<'h> {
    descriptors: Vec<DescriptorProto>,
    locations: Vec<location::Message>,
    container: container::Key,
    container_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
    ancestor_refs: &'h mut Vec<reference::Inner>,
}

struct HydrateMethods<'h> {
    descriptors: Vec<MethodDescriptorProto>,
    locations: Vec<location::Method>,
    service: service::Key,
    service_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
    service_refs: &'h mut Vec<reference::Inner>,
}

struct HydrateEnum<'h> {
    descriptor: EnumDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Enum,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}
struct HydrateEnums<'h> {
    descriptors: Vec<EnumDescriptorProto>,
    locations: Vec<location::Enum>,
    container: container::Key,
    container_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}

struct HydrateEnumValue<'h> {
    descriptor: EnumValueDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::EnumValue,
    enum_: enum_::Key,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}
struct HydrateEnumValues<'h> {
    descriptors: Vec<EnumValueDescriptorProto>,
    locations: Vec<location::EnumValue>,
    enum_: enum_::Key,
    enum_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}

struct HydrateService<'h> {
    descriptor: ServiceDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Service,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
    ancestor_refs: &'h mut Vec<reference::Inner>,
}
struct HydrateServices<'h> {
    descriptors: Vec<ServiceDescriptorProto>,
    locations: Vec<location::Service>,
    file_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
    file_refs: &'h mut Vec<reference::Inner>,
}

struct HydrateMethod {
    fqn: FullyQualifiedName,
    location: location::Method,
    descriptor: MethodDescriptorProto,
    service: service::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct HydrateField<'h> {
    descriptor: FieldDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Field,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
    oneof: Option<oneof::Key>,
    references: &'h mut Vec<reference::Inner>,
    nodes: &'h mut node::Map,
}
struct HydrateFields<'h> {
    descriptors: Vec<FieldDescriptorProto>,
    locations: Vec<location::Field>,
    message: message::Key,
    message_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
    oneofs: &'h [oneof::Ident],
}

struct HydrateOneof<'h> {
    descriptor: OneofDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Oneof,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}
struct HydrateOneofs<'h> {
    descriptors: Vec<OneofDescriptorProto>,
    locations: Vec<location::Oneof>,
    message: message::Key,
    message_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}

struct HydrateExtension<'h> {
    descriptor: FieldDescriptorProto,
    location: location::Extension,
    ext_decl: extension_decl::Key,
    fqn: FullyQualifiedName,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}

struct HydrateExtensions<'h> {
    ext_descriptors: Vec<FieldDescriptorProto>,
    decl_locations: Vec<location::ExtensionDecl>,
    container: container::Key,
    container_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}

struct HydrateExtensionDecl<'h, D> {
    descriptors: D,
    location: location::ExtensionDecl,
    container: container::Key,
    container_fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'h mut node::Map,
}

struct ExtensionsHydrated {
    extension_decls: Vec<extension_decl::Key>,
    extensions: Vec<extension::Ident>,
    ext_refs: Vec<reference::Inner>,
}
impl ExtensionsHydrated {
    fn new(decls: &[location::ExtensionDecl], exts: &[FieldDescriptorProto]) -> Self {
        Self {
            extension_decls: Vec::with_capacity(decls.len()),
            extensions: Vec::with_capacity(exts.len()),
            ext_refs: Vec::with_capacity(exts.len()),
        }
    }
    fn push(&mut self, decl: ExtensionDeclHydrated) {
        self.extension_decls.push(decl.key);
        self.extensions.extend(decl.extensions);
        self.ext_refs.extend(decl.ext_refs);
    }
}

struct ExtensionHydrated {
    extension: extension::Ident,
    reference: Option<reference::Inner>,
    ext_decl: extension_decl::Key,
}

struct FieldsHydrated {
    fields: Vec<field::Ident>,
    field_refs: Vec<reference::Inner>,
}

struct ExtensionDeclHydrated {
    key: extension_decl::Key,
    extensions: Vec<extension::Ident>,
    ext_refs: Vec<reference::Inner>,
}
impl ExtensionDeclHydrated {
    fn new(key: extension_decl::Key, len: usize) -> Self {
        Self {
            key,
            extensions: Vec::with_capacity(len),
            ext_refs: Vec::with_capacity(len),
        }
    }
}

fn validate_locations<T, L>(
    container_fqn: &FullyQualifiedName,
    kind: location::Kind,
    locations: &[L],
    descriptors: &[T],
) -> Result<(), error::HydrationFailed> {
    if locations.len() == descriptors.len() {
        Ok(())
    } else {
        Err(error::LocationsMisaligned {
            kind,
            expected: descriptors.len(),
            actual: locations.len(),
            backtrace: Backtrace::capture(),
        })
        .with_context(|_| error::LocationsMisalignedCtx {
            container_fqn: container_fqn.clone(),
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

fn validate_extension_locations(
    descriptors: &Peekable<IntoIter<FieldDescriptorProto>>,
    locations: &[location::Field],
) -> Result<(), error::LocationsMisaligned> {
    if descriptors.len() < locations.len() {
        return Err(error::LocationsMisaligned {
            expected: locations.len(),
            actual: descriptors.len(),
            kind: location::Kind::Extension,
            backtrace: Backtrace::capture(),
        });
    }
    Ok(())
}
