use ahash::HashMapExt;
use itertools::Itertools;
use protobuf::{
    descriptor::{
        field_descriptor_proto, DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto,
        FieldDescriptorProto, FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto,
        ServiceDescriptorProto, SourceCodeInfo,
    },
    EnumOrUnknown, MessageField,
};
use snafu::{Backtrace, ResultExt};
use std::{path::PathBuf, str::FromStr};

use crate::{
    ast::{access::AccessFqn, container::ContainerKey, field::Scalar, file},
    error::{self, Error, HydrationCtx, HydrationFailed, InvalidMapKey},
    HashMap,
};

use super::{
    container, dependency, dependent,
    enum_::{self, EnumKey},
    enum_value,
    extension::{self, ExtensionIdent},
    extension_decl::ExtensionDeclKey,
    field::{self, FieldTypeInner, MapInner, MapKey, ValueInner},
    file::{FileIdent, FileKey},
    location::{self, ExtensionDeclLocation, ExtensionLocation, FileLocation, OneofLocation},
    message::{self, MessageKey},
    method, node, oneof,
    package::PackageKey,
    reference::{self, ReferenceInner, ReferrerKey},
    service::{self, ServiceKey},
    Ast, FullyQualifiedName, Name,
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
            self.hydrate_file(descriptor, targets)
                .with_context(|_| HydrationCtx {
                    file_path: file_path.clone(),
                })?;
        }
        self.finalize_files()
    }

    fn finalize_files(&mut self) -> Result<(), Error> {
        // this method is a hot mess and likely has a lot of room for perf
        // improvement

        let mut dependencies = HashMap::with_capacity(self.ast.files.len());
        for (dependent_file_key, dependent) in self.ast.files.iter_mut() {
            let mut dep_refs = vec![Vec::new(); dependent.dependencies.direct.len()];
            for reference in &dependent.all_references {
                let dependency_file_key = match reference.referent {
                    reference::ReferentKey::Message(msg) => self.ast.messages[msg].file,
                    reference::ReferentKey::Enum(enm) => self.ast.enums[enm].file,
                };
                if dependency_file_key == dependent_file_key {
                    continue;
                }

                let dep_idx = dependent
                    .dependencies
                    .direct
                    .iter()
                    .position(|dep| dep.dependency == dependency_file_key);

                assert!(
                    dep_idx.is_some(),
                    "could not find dependency {dependency_file_key:?} for {dependent_file_key:?}\n{:?}\n",
                    dependent.fqn()
                );
                let dep_idx = dep_idx.unwrap();
                dep_refs[dep_idx].push(*reference);
            }

            for (idx, dep_refs) in dep_refs.into_iter().enumerate() {
                if dep_refs.is_empty() {
                    dependent.dependencies.unused.push(idx);
                }
            }
            dependencies.insert(dependent_file_key, dependent.dependencies.clone());
        }
        for (dependent_key, dependencies) in dependencies.drain() {
            for dependency in dependencies.direct.iter().as_ref() {
                let dependency_file = &mut self.ast.files[dependency.dependency];
                let idx = dependencies
                    .direct
                    .iter()
                    .position(|dep| dep.dependency == dependency_file.key)
                    .unwrap();
                let is_weak = dependencies.weak.contains(&idx);
                let is_public = dependencies.public.contains(&idx);
                let is_unused = dependencies.unused.contains(&idx);
                dependency_file.dependents.push(dependent::NewDependent {
                    dependent: dependent_key,
                    dependency: dependency_file.key,
                    is_public,
                    is_unused,
                    is_weak,
                });
            }
        }
        #[allow(clippy::unnecessary_to_owned)]
        for file in self.ast.files.keys().to_vec() {
            let mut transitive_dependents = Vec::new();
            self.collect_transitive_dependents(file, &mut transitive_dependents);
            let mut transitive_dependencies = Vec::new();
            self.collect_transitive_dependencies(file, &mut transitive_dependencies);
            let file = &mut self.ast.files[file];
            file.dependencies.transitive = transitive_dependencies;
            file.dependents.transitive = transitive_dependents;
        }

        Ok(())
    }
    fn collect_transitive_dependents(
        &self,
        key: FileKey,
        transitive: &mut Vec<dependent::DependentInner>,
    ) {
        transitive.reserve(self.ast.files[key].dependents.direct.len());
        for dep in self.ast.files[key].dependents.direct.clone() {
            transitive.push(dep);
            self.collect_transitive_dependents(dep.dependent, transitive);
        }
    }

    fn collect_transitive_dependencies(
        &self,
        key: FileKey,
        transitive: &mut Vec<dependency::DependencyInner>,
    ) {
        transitive.reserve(self.ast.files[key].dependencies.direct.len());
        for dep in self.ast.files[key].dependencies.direct.clone() {
            transitive.push(dep);
            self.collect_transitive_dependencies(dep.dependency, transitive);
        }
    }

    fn hydrate_package(
        &mut self,
        package: Option<String>,
    ) -> (Option<PackageKey>, Option<FullyQualifiedName>) {
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
    ) -> Result<FileIdent, HydrationFailed> {
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
        let path: PathBuf = name.clone().into();
        let fqn = FullyQualifiedName::for_file(package_fqn);
        assert!(!name.is_empty(), "file {name}");
        let key = self.ast.files.get_or_insert_key_by_path(&path);
        // setting it here so that it can be used during hydration of dependencies?
        self.ast.files[key].name = name.clone();

        let location = file_location(source_code_info)?;
        let mut nodes = HashMap::with_capacity(location.node_count);
        self.ast.reserve(location.node_count);

        let mut all_references = Vec::new();

        let messages = self.hydrate_messages(HydrateMessages {
            descriptors: message_type,
            locations: location.messages,
            container: ContainerKey::File(key),
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
        let options = options.unwrap_or_default();
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
    ) -> Result<Vec<message::MessageIdent>, HydrationFailed> {
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
    ) -> Result<message::MessageIdent, HydrationFailed> {
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
        let DescriptorProto {
            name,
            reserved_name,
            reserved_range,
            extension_range,
            options,
            special_fields,
            field,
            extension,
            nested_type,
            enum_type,
            oneof_decl,
        } = descriptor;
        let mut all_refs = Vec::new();
        let name: Name = name.unwrap_or_default().into();
        let key = self.ast.messages.get_or_insert_key(fqn.clone());
        let enums = self.hydrate_enums(HydrateEnums {
            descriptors: enum_type,
            locations: location.enums,
            container: key.into(),
            container_fqn: fqn.clone(),
            file,
            package,
            nodes,
        })?;
        let messages = self.hydrate_messages(HydrateMessages {
            descriptors: nested_type,
            locations: location.messages,
            container: key.into(),
            container_fqn: fqn.clone(),
            ancestor_refs: &mut all_refs,
            file,
            package,
            nodes,
        })?;
        let oneofs = self.hydrate_oneofs(HydrateOneofs {
            descriptors: oneof_decl,
            locations: location.oneofs,
            message: key,
            message_fqn: fqn.clone(),
            file,
            package,
            nodes,
        })?;
        let FieldsHydrated { fields, field_refs } = self.hydrate_fields(HydrateFields {
            descriptors: field,
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
            container_fqn: fqn,
            decl_locations: location.extensions,
            ext_descriptors: extension,
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
        let options = options.unwrap_or_default();
        let msg = self.ast.messages[key].hydrate(message::Hydrate {
            name,
            location: location.detail,
            all_references: all_refs,
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
            options,
            reserved_ranges: reserved_range,
            reserved_names: reserved_name,
            extension_range,
            special_fields,
            file,
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
    ) -> Result<(field::FieldIdent, Option<ReferenceInner>), HydrationFailed> {
        let HydrateField {
            descriptor,
            fqn,
            location,
            message,
            file,
            package,
            oneof,
            nodes,
        } = field;

        if fqn == ".extended.Validated.value" {
            println!("here");
        }

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
        let options = options.unwrap_or_default();
        let name: Name = name.unwrap_or_default().into();
        let key = self.ast.fields.get_or_insert_key(fqn.clone());
        let label = label
            .and_then(|l| l.enum_value().ok())
            .map(field::Label::from);
        let type_ = self.hydrate_field_type(type_, label, type_name, &fqn)?;
        let reference = self.hydrate_reference(key.into(), type_);
        let number = number.ok_or_else(|| HydrationFailed::FieldMissingNumber {
            field_fqn: fqn.clone(),
        })?;
        let field = self.ast.fields[key].hydrate(field::Hydrate {
            name,
            location: location.detail,
            number,
            label,
            type_,
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

        let field = self.insert_node(nodes, field)?;
        if let Some(oneof) = oneof {
            self.ast.oneofs[oneof].add_field(field.clone());
        }
        Ok((field, reference))
    }

    #[allow(clippy::unused_self)]
    fn hydrate_reference(
        &mut self,
        referrer: ReferrerKey,
        field_type: FieldTypeInner,
    ) -> Option<ReferenceInner> {
        let value = match field_type {
            FieldTypeInner::Repeated(val) | FieldTypeInner::Single(val) => val,
            FieldTypeInner::Map(map) => map.value,
        };
        let referent = match value {
            ValueInner::Enum(key) => reference::ReferentKey::Enum(key),
            ValueInner::Message(key) => reference::ReferentKey::Message(key),
            ValueInner::Scalar(_) => return None,
        };

        Some(ReferenceInner { referrer, referent })
    }

    fn hydrate_dependencies(
        &mut self,
        dependent: FileKey,
        dependencies: Vec<String>,
    ) -> Result<Vec<dependency::DependencyInner>, HydrationFailed> {
        let direct_dependencies = dependencies
            .into_iter()
            .map(|dependency| {
                let dependency = self.ast.files.get_or_insert_key(dependency.into());
                dependency::DependencyInner {
                    dependency,
                    dependent,
                }
            })
            .collect_vec();
        Ok(direct_dependencies)
    }

    fn hydrate_value_type_map(&mut self, key: MessageKey) -> Result<FieldTypeInner, InvalidMapKey> {
        let map = &self.ast.messages[key];
        let map_key = self.ast.fields.get(map.fields.get(0).unwrap()).unwrap();
        let map_value = self.ast.fields.get(map.fields.get(1).unwrap()).unwrap();
        let map_key = MapKey::try_from(map_key.field_type)?;

        match map_value.field_type {
            FieldTypeInner::Single(val) => Ok(FieldTypeInner::Map(MapInner {
                key: map_key,
                value: val,
            })),
            _ => unreachable!(),
        }
    }
    fn hydrate_value_type_message(
        &mut self,
        fqn: FullyQualifiedName,
        is_repeated: bool,
    ) -> Result<FieldTypeInner, InvalidMapKey> {
        let key = self.ast.messages.get_or_insert_key(fqn);
        let is_map_entry = self.ast.messages[key].options.map_entry.unwrap_or_default();
        if is_map_entry {
            return self.hydrate_value_type_map(key);
        };
        let value = ValueInner::Message(key);
        if is_repeated {
            Ok(FieldTypeInner::Repeated(value))
        } else {
            Ok(FieldTypeInner::Single(value))
        }
    }
    fn hydrate_value_type_enum(
        &mut self,
        fqn: FullyQualifiedName,
        is_repeated: bool,
    ) -> FieldTypeInner {
        let value = ValueInner::Enum(self.ast.enums.get_or_insert_key(fqn));
        if is_repeated {
            FieldTypeInner::Repeated(value)
        } else {
            FieldTypeInner::Single(value)
        }
    }

    fn hydrate_field_type(
        &mut self,
        type_: Option<EnumOrUnknown<field_descriptor_proto::Type>>,
        label: Option<field::Label>,
        type_name: Option<String>,
        container_fqn: &FullyQualifiedName,
    ) -> Result<FieldTypeInner, HydrationFailed> {
        use field_descriptor_proto::Type as ProtoType;

        let label = label.unwrap_or_default();
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
            ProtoType::TYPE_MESSAGE => self
                .hydrate_value_type_message(extract_type_name(type_name)?, label.is_repeated())
                .with_context(|_| error::InvalidMapKeyCtx {
                    fqn: container_fqn.clone(),
                }),
            ProtoType::TYPE_ENUM => Ok(
                self.hydrate_value_type_enum(extract_type_name(type_name)?, label.is_repeated())
            ),
            ProtoType::TYPE_GROUP => {
                // using let here to avoid formatting issues
                // also, this is probably not the way i should be
                // creating this error.
                let group_not_supported = error::GroupNotSupported {
                    backtrace: Backtrace::capture(),
                };
                Err(group_not_supported).with_context(|_| error::GroupNotSupportedCtx {
                    field_fqn: container_fqn.clone(),
                })
            }
            _ => {
                let scalar = Scalar::try_from(proto_type).unwrap();
                Ok(FieldTypeInner::Single(ValueInner::Scalar(scalar)))
            }
        }
    }

    fn hydrate_enums(
        &mut self,
        enums: HydrateEnums,
    ) -> Result<Vec<enum_::EnumIdent>, HydrationFailed> {
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
                    descriptor,
                    fqn,
                    location,
                    container,
                    file,
                    package,
                    nodes,
                })
            })
            .collect()
    }

    fn hydrate_enum(&mut self, enum_: HydrateEnum) -> Result<enum_::EnumIdent, HydrationFailed> {
        let HydrateEnum {
            descriptor,
            fqn,
            location,
            container,
            file,
            package,
            nodes,
        } = enum_;
        let EnumDescriptorProto {
            name,
            options,
            special_fields,
            value,
            reserved_name,
            reserved_range,
        } = descriptor;
        let key = self.ast.enums.get_or_insert_key(fqn.clone());
        let name: Name = name.unwrap_or_default().into();
        let values = self.hydrate_enum_values(HydrateEnumValues {
            descriptors: value,
            locations: location.values,
            enum_: key,
            enum_fqn: fqn,
            file,
            package,
            nodes,
        })?;
        let well_known = if self.is_well_known(package) {
            enum_::WellKnownEnum::from_str(&name).ok()
        } else {
            None
        };
        let options = options.unwrap_or_default();
        let enum_ = self.ast.enums[key].hydrate(enum_::Hydrate {
            name,
            well_known,
            file,
            location: location.detail,
            container,
            package,
            values,
            options,
            reserved_ranges: reserved_range,
            reserved_names: reserved_name,
            special_fields,
        })?;

        nodes.insert(enum_.fqn(), enum_.node_key());
        self.ast.nodes.insert(enum_.fqn(), enum_.node_key());
        Ok(enum_)
    }

    fn hydrate_oneofs(
        &mut self,
        oneofs: HydrateOneofs,
    ) -> Result<Vec<oneof::OneofIdent>, HydrationFailed> {
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
    ) -> Result<super::node::Ident<oneof::OneofKey>, HydrationFailed> {
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
        let options = options.unwrap_or_default();
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
        let mut descriptors = ext_descriptors.into_iter();
        for decl_loc in decl_locations {
            validate_extension_locations(descriptors.len(), decl_loc.extensions.len())
                .with_context(|_| error::LocationsMisalignedCtx {
                    container_fqn: container_fqn.clone(),
                })?;

            let descriptors = descriptors.by_ref().take(decl_loc.extensions.len());
            let ext_decl = self.hydrate_extension_decl(HydrateExtensionDecl {
                descriptors,
                location: decl_loc,
                container,
                container_fqn: container_fqn.clone(),
                file,
                package,
                nodes,
            })?;
            hydrated.push(ext_decl);
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

        let (key, ext_decl) = self.ast.extension_decls.insert_default();
        ext_decl.hydrate(location.detail, file, package, descriptors.len());
        let mut hydrated = ExtensionDeclHydrated::new(key, descriptors.len());
        let mut extensions = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.zip(location.extensions) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let ExtensionHydrated {
                extension,
                reference,
            } = self.hydrate_extension(HydrateExtension {
                container,
                ext_decl: key,
                descriptor,
                file,
                package,
                location,
                nodes,
                fqn,
            })?;
            extensions.push(extension.key);
            if let Some(reference) = reference {
                hydrated.ext_refs.push(reference);
            }
            hydrated.extensions.push(extension);
        }
        self.ast.extension_decls.get_mut(key).unwrap().extensions = extensions;
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
            ext_decl: extension_decl,
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
        let label = label
            .and_then(|l| l.enum_value().ok())
            .map(field::Label::from);
        let type_ = self.hydrate_field_type(type_, label, type_name, &fqn)?;
        let reference = self.hydrate_reference(key.into(), type_);

        let number = number.ok_or_else(|| HydrationFailed::FieldMissingNumber {
            field_fqn: fqn.clone(),
        })?;
        let extendee = self
            .ast
            .messages
            .get_or_insert_key(FullyQualifiedName(extendee.unwrap().into()));

        // Adds the extension to the extendee message's applied extensions.
        self.ast
            .messages
            .get_mut(extendee)
            .unwrap()
            .applied_extensions
            .push(key);
        let options = options.unwrap_or_default();
        let extension = self.ast.extensions[key].hydrate(extension::HydrateExtension {
            name,
            location: location.detail,
            number,
            label,
            type_,
            default_value,
            json_name,
            options,
            extension_decl,
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
            extension,
            reference,
        })
    }

    fn is_well_known(&self, package: Option<PackageKey>) -> bool {
        let Some(package) = package else { return false };
        self.ast.well_known == package
    }

    fn hydrate_services(
        &mut self,
        services: HydrateServices,
    ) -> Result<Vec<service::ServiceIdent>, HydrationFailed> {
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
    ) -> Result<service::ServiceIdent, HydrationFailed> {
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
            service_fqn: fqn,
            file,
            package,
            nodes,
            service_refs: &mut references,
        })?;

        ancestor_refs.extend(references.iter().copied());
        let options = options.unwrap_or_default();
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
    ) -> Result<Vec<method::MethodIdent>, HydrationFailed> {
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
        nodes: &mut node::NodeMap,
        references: &mut Vec<ReferenceInner>,
    ) -> Result<method::MethodIdent, HydrationFailed> {
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

        references.push(ReferenceInner {
            referent: input.into(),
            referrer: (key, method::Direction::Input).into(),
        });
        references.push(ReferenceInner {
            referent: output.into(),
            referrer: (key, method::Direction::Output).into(),
        });

        let io = method::IoInner::new(
            input,
            client_streaming.unwrap_or_default(),
            output,
            server_streaming.unwrap_or_default(),
        );
        let options = options.unwrap_or_default();
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
    ) -> Result<Vec<enum_value::EnumValueIdent>, HydrationFailed> {
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
    ) -> Result<enum_value::EnumValueIdent, HydrationFailed> {
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
        let options = options.unwrap_or_default();
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
        nodes: &mut node::NodeMap,
        node: node::Ident<K>,
    ) -> Result<node::Ident<K>, HydrationFailed>
    where
        K: Copy + Into<node::NodeKey>,
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
    location: location::MessageLocation,
    container: ContainerKey,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
    ancestor_refs: &'h mut Vec<ReferenceInner>,
}
struct HydrateMessages<'h> {
    descriptors: Vec<DescriptorProto>,
    locations: Vec<location::MessageLocation>,
    container: ContainerKey,
    container_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
    ancestor_refs: &'h mut Vec<ReferenceInner>,
}

struct HydrateMethods<'h> {
    descriptors: Vec<MethodDescriptorProto>,
    locations: Vec<location::MethodLocation>,
    service: ServiceKey,
    service_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
    service_refs: &'h mut Vec<ReferenceInner>,
}

struct HydrateEnum<'h> {
    descriptor: EnumDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::EnumLocation,
    container: ContainerKey,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}
struct HydrateEnums<'h> {
    descriptors: Vec<EnumDescriptorProto>,
    locations: Vec<location::EnumLocation>,
    container: ContainerKey,
    container_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}

struct HydrateEnumValue<'h> {
    descriptor: EnumValueDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::EnumValueLocation,
    enum_: EnumKey,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}
struct HydrateEnumValues<'h> {
    descriptors: Vec<EnumValueDescriptorProto>,
    locations: Vec<location::EnumValueLocation>,
    enum_: EnumKey,
    enum_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}

struct HydrateService<'h> {
    descriptor: ServiceDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::ServiceLocation,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
    ancestor_refs: &'h mut Vec<ReferenceInner>,
}
struct HydrateServices<'h> {
    descriptors: Vec<ServiceDescriptorProto>,
    locations: Vec<location::ServiceLocation>,
    file_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
    file_refs: &'h mut Vec<ReferenceInner>,
}

struct HydrateMethod {
    fqn: FullyQualifiedName,
    location: location::MethodLocation,
    descriptor: MethodDescriptorProto,
    service: ServiceKey,
    file: FileKey,
    package: Option<PackageKey>,
}

struct HydrateField<'h> {
    descriptor: FieldDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::FieldLocation,
    message: MessageKey,
    file: FileKey,
    package: Option<PackageKey>,
    oneof: Option<oneof::OneofKey>,
    nodes: &'h mut node::NodeMap,
}
struct HydrateFields<'h> {
    descriptors: Vec<FieldDescriptorProto>,
    locations: Vec<location::FieldLocation>,
    message: MessageKey,
    message_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
    oneofs: &'h [oneof::OneofIdent],
}

struct HydrateOneof<'h> {
    descriptor: OneofDescriptorProto,
    fqn: FullyQualifiedName,
    location: OneofLocation,
    message: MessageKey,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}
struct HydrateOneofs<'h> {
    descriptors: Vec<OneofDescriptorProto>,
    locations: Vec<OneofLocation>,
    message: MessageKey,
    message_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}

struct HydrateExtension<'h> {
    descriptor: FieldDescriptorProto,
    location: ExtensionLocation,
    ext_decl: ExtensionDeclKey,
    fqn: FullyQualifiedName,
    container: ContainerKey,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}

struct HydrateExtensions<'h> {
    ext_descriptors: Vec<FieldDescriptorProto>,
    decl_locations: Vec<ExtensionDeclLocation>,
    container: ContainerKey,
    container_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}

struct HydrateExtensionDecl<'h, D> {
    descriptors: D,
    location: ExtensionDeclLocation,
    container: ContainerKey,
    container_fqn: FullyQualifiedName,
    file: FileKey,
    package: Option<PackageKey>,
    nodes: &'h mut node::NodeMap,
}

struct ExtensionsHydrated {
    extension_decls: Vec<ExtensionDeclKey>,
    extensions: Vec<ExtensionIdent>,
    ext_refs: Vec<ReferenceInner>,
}
impl ExtensionsHydrated {
    fn new(decls: &[ExtensionDeclLocation], exts: &[FieldDescriptorProto]) -> Self {
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
    extension: ExtensionIdent,
    reference: Option<ReferenceInner>,
}

struct FieldsHydrated {
    fields: Vec<field::FieldIdent>,
    field_refs: Vec<ReferenceInner>,
}

struct ExtensionDeclHydrated {
    key: ExtensionDeclKey,
    extensions: Vec<ExtensionIdent>,
    ext_refs: Vec<ReferenceInner>,
}
impl ExtensionDeclHydrated {
    fn new(key: ExtensionDeclKey, len: usize) -> Self {
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

fn file_location(info: MessageField<SourceCodeInfo>) -> Result<FileLocation, HydrationFailed> {
    let info = info
        .0
        .ok_or_else(|| error::HydrationFailed::MissingSourceCodeInfo)?;
    FileLocation::new(*info)
}

fn oneof_for_field(
    descriptor: &FieldDescriptorProto,
    oneofs: &[oneof::OneofIdent],
) -> Result<Option<oneof::OneofKey>, error::InvalidIndex> {
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
    descriptors: usize,
    locations: usize,
) -> Result<(), error::LocationsMisaligned> {
    if descriptors < locations {
        return Err(error::LocationsMisaligned {
            expected: locations,
            actual: descriptors,
            kind: location::Kind::Extension,
            backtrace: Backtrace::capture(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn test_() {
        let cgr = crate::test::cgr::extended();
        let ast = Ast::build(cgr.proto_file, &[]).unwrap();
        fs::write("./dbg.rs", format!("{ast:#?}")).unwrap();
    }
}
