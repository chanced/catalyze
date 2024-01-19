use std::{
    iter::once as iter_once,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{error::Error, to_i32, HashMap, Mutex};

use super::{
    container, r#enum, enum_value, extension, extension_block, field,
    file::{self, DependencyInner},
    location, message, method, node, oneof, package, service, Ast, EnumTable, EnumValueTable,
    ExtensionBlockTable, ExtensionTable, FieldTable, FileTable, FullyQualifiedName, Hydrated,
    MessageTable, MethodTable, OneofTable, PackageTable, ServiceTable,
};

pub(crate) fn run(
    file_descriptors: Vec<FileDescriptorProto>,
    targets: &[String],
) -> Result<super::Ast, Error> {
    Hydrate::new(file_descriptors, targets).map(Into::into)
}

#[derive(Default)]
pub(super) struct Hydrate {
    packages: Mutex<PackageTable>,
    files: Mutex<FileTable>,
    messages: Mutex<MessageTable>,
    enums: Mutex<EnumTable>,
    enum_values: Mutex<EnumValueTable>,
    services: Mutex<ServiceTable>,
    methods: Mutex<MethodTable>,
    fields: Mutex<FieldTable>,
    oneofs: Mutex<OneofTable>,
    extensions: Mutex<ExtensionTable>,
    extension_blocks: Mutex<ExtensionBlockTable>,
    xnodes: HashMap<FullyQualifiedName, node::Key>,
    well_known_package: package::Key,
}

impl Into<Ast> for Hydrate {
    fn into(self) -> Ast {
        Ast {
            packages: self.packages.take(),
            files: self.files.take(),
            messages: self.messages.take(),
            enums: self.enums.take(),
            enum_values: self.enum_values.take(),
            services: self.services.take(),
            methods: self.methods.take(),
            fields: self.fields.take(),
            oneofs: self.oneofs.take(),
            extensions: self.extensions.take(),
            extension_blocks: self.extension_blocks.take(),
            nodes: self.xnodes,
            well_known_package: self.well_known_package,
        }
    }
}

impl Hydrate {
    fn new(file_descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Self, Error> {
        Self::default().init(file_descriptors, targets)
    }
    fn init(
        mut self,
        file_descriptors: Vec<FileDescriptorProto>,
        targets: &[String],
    ) -> Result<Self, Error> {
        let well_known = self.hydrate_package(Some("google.protobuf".to_string()));
        let len = file_descriptors.len();
        let nodes = file_descriptors
            .into_par_iter()
            .map(|descriptor| self.hydrate_file(descriptor, targets));
        todo!()

        // self.xnodes = nodes?;

        // Ok(self)
    }

    fn hydrate_file(
        &mut self,
        descriptor: FileDescriptorProto,
        targets: &[String],
    ) -> Result<(Hydrated<file::Key>, Vec<(FullyQualifiedName, node::Key)>), Error> {
        let name = descriptor.name.unwrap();
        let locations = location::File::new(descriptor.source_code_info.unwrap_or_else(|| {
            panic!("source_code_info not found on FileDescriptorProto for \"{name}\"")
        }))?;

        let is_build_target = targets.iter().any(|t| t == &name);

        let (package, package_fqn) = self.hydrate_package(descriptor.package);
        let fqn = FullyQualifiedName::new(&name, package_fqn);
        let key = self.files.lock().get_or_insert_key_by_fqn(fqn.clone());

        let (messages, message_nodes) = self.hydrate_messages(
            descriptor.message_type,
            locations.messages,
            fqn.clone(),
            key.into(),
            key,
            package,
        )?;

        let (enums, enum_nodes) = self.hydrate_enums(
            descriptor.enum_type,
            locations.enums,
            key.into(),
            fqn.clone(),
            key,
            package,
        );

        let (services, service_nodes) = self.hydrate_services(
            descriptor.service,
            locations.services,
            fqn.clone(),
            key,
            package,
        )?;
        let (extension_blocks, extensions, extension_nodes) = self.hydrate_extensions(
            descriptor.extension,
            locations.extensions,
            key.into(),
            fqn,
            key,
            package,
        )?;

        let dependencies = self.hydrate_dependencies(
            key,
            descriptor.dependency,
            descriptor.public_dependency,
            descriptor.weak_dependency,
        );

        let file = &mut self.files.lock()[key];
        let (key, fqn, name) = file.hydrate(file::Hydrate {
            name: name.into_boxed_str(),
            syntax: descriptor.syntax,
            options: descriptor.options.unwrap_or_default(),
            package,
            messages,
            enums,
            services,
            extensions,
            extension_blocks,
            dependencies,
            package_comments: locations.package.and_then(|loc| loc.comments),
            comments: locations.syntax.and_then(|loc| loc.comments),
            is_build_target,
        })?;
        let iter = iter_once((fqn.clone(), key.into()))
            .chain(message_nodes)
            .chain(enum_nodes)
            .chain(service_nodes)
            .chain(extension_nodes)
            .collect::<Vec<_>>();
        Ok(((key, fqn, name), iter))
    }

    fn hydrate_package(
        &mut self,
        package: Option<String>,
    ) -> (Option<package::Key>, Option<FullyQualifiedName>) {
        let Some(package) = package else {
            return (None, None);
        };

        let is_well_known = package == package::WELL_KNOWN;
        if package.is_empty() {
            return (None, None);
        }
        let fqn = FullyQualifiedName::for_package(package);
        let (key, pkg) = self.packages.lock().get_or_insert_by_fqn(fqn.clone());
        pkg.set_name(package);
        (Some(key), Some(fqn))
    }

    fn hydrate_dependencies(
        &mut self,
        dependent: file::Key,
        dependencies_by_fqn: Vec<String>,
        public_dependencies: Vec<i32>,
        weak_dependencies: Vec<i32>,
    ) -> file::DependenciesInner {
        let mut all = Vec::with_capacity(dependencies_by_fqn.len());
        let mut weak = Vec::with_capacity(weak_dependencies.len());
        let mut public = Vec::with_capacity(public_dependencies.len());

        for (i, dependency) in dependencies_by_fqn.into_iter().enumerate() {
            let index = to_i32(i);
            let is_weak = weak_dependencies.contains(&index);
            let is_public = public_dependencies.contains(&index);
            let fqn = FullyQualifiedName::from(dependency);
            let (dependency_key, dependency_file) =
                self.files.lock().get_or_insert_by_fqn(fqn.clone());

            let dep = DependencyInner {
                is_used: bool::default(),
                is_public,
                is_weak,
                dependent,
                dependency: dependency_key,
            };
            dependency_file.add_dependent(dep.into());
            all.push(dep);

            if is_public {
                public.push(dep);
            }
            if is_weak {
                weak.push(dep);
            }
        }
        file::DependenciesInner {
            all,
            public,
            weak,
            unusued: Vec::default(),
        }
    }

    fn hydrate_messages(
        &mut self,
        descriptors: Vec<DescriptorProto>,
        locations: Vec<location::Message>,
        container_fqn: FullyQualifiedName,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<
        (
            Vec<Hydrated<message::Key>>,
            impl Iterator<Item = (FullyQualifiedName, node::Key)>,
        ),
        Error,
    > {
        assert_message_locations(&container_fqn, &locations, &descriptors);
        let mut messages = Vec::with_capacity(descriptors.len());
        let mut all_nodes = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let ((key, fqn, name), nodes) =
                self.hydrate_message(descriptor, fqn, location, container, file, package)?;
            all_nodes.push(iter_once((fqn.clone(), key.into())).chain(nodes));
            messages.push((key, fqn, name));
        }

        Ok((messages, all_nodes.into_iter().flatten()))
    }

    fn is_well_known(&self, package: Option<package::Key>) -> bool {
        if let Some(package) = package {
            return package == self.well_known_package;
        }
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn hydrate_message(
        &mut self,
        descriptor: DescriptorProto,
        fqn: FullyQualifiedName,
        location: location::Message,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<
        (
            Hydrated<message::Key>,
            impl Iterator<Item = (FullyQualifiedName, node::Key)>,
        ),
        Error,
    > {
        let name = descriptor.name.unwrap_or_default().into_boxed_str();
        let key = self.messages.lock().get_or_insert_key_by_fqn(fqn.clone());

        let well_known = if self.is_well_known(package) {
            message::WellKnownMessage::from_str(&name).ok()
        } else {
            None
        };

        let (extension_blocks, extensions, extension_nodes) = self.hydrate_extensions(
            descriptor.extension,
            location.extensions,
            key.into(),
            fqn.clone(),
            file,
            package,
        )?;

        let (messages, message_nodes) = self.hydrate_messages(
            descriptor.nested_type,
            location.messages,
            fqn.clone(),
            key.into(),
            file,
            package,
        )?;

        let (enums, enum_nodes) = self.hydrate_enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            file,
            package,
        );
        let (oneofs, oneof_nodes) = self.hydrate_oneofs(
            descriptor.oneof_decl,
            location.oneofs,
            key,
            fqn.clone(),
            file,
            package,
        )?;
        let (fields, field_nodes) = self.hydrate_fields(
            descriptor.field,
            location.fields,
            key.into(),
            fqn,
            file,
            package,
        )?;
        let location = location.detail;
        let (key, fqn, name) = self.messages.lock()[key].hydrate(message::Hydrate {
            name,
            container,
            fields,
            location,
            messages,
            oneofs,
            enums,
            well_known,
            extension_blocks,
            extensions,
            package,
            options: descriptor.options,
            reserved_names: descriptor.reserved_name,
            reserved_ranges: descriptor.reserved_range,
            special_fields: descriptor.special_fields,
            extension_range: descriptor.extension_range,
        });
        let nodes = iter_once((fqn.clone(), key.into()))
            .chain(message_nodes)
            .chain(enum_nodes)
            .chain(oneof_nodes)
            .chain(field_nodes)
            .chain(extension_nodes);
        Ok(((key, fqn, name), nodes))
    }

    fn hydrate_enums(
        &mut self,
        descriptors: Vec<EnumDescriptorProto>,
        locations: Vec<location::Enum>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> (
        Vec<Hydrated<r#enum::Key>>,
        impl Iterator<Item = (FullyQualifiedName, node::Key)>,
    ) {
        assert_enum_locations(&container_fqn, &locations, &descriptors);
        let mut enums = Vec::with_capacity(descriptors.len());
        let mut iters = Vec::new();
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let ((key, fqn, name), nodes) =
                self.hydrate_enum(descriptor, fqn.clone(), location, container, file, package);
            iters.push(iter_once((fqn.clone(), key.into())).chain(nodes));
            enums.push((key, fqn.clone(), name));
        }
        (enums, iters.into_iter().flatten())
    }

    fn hydrate_enum(
        &mut self,
        descriptor: EnumDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::Enum,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> (
        Hydrated<r#enum::Key>,
        impl Iterator<Item = (FullyQualifiedName, node::Key)>,
    ) {
        let name = descriptor.name.clone().unwrap_or_default().into_boxed_str();
        let key = self.enums.lock().get_or_insert_key_by_fqn(fqn.clone());
        let (values, iter) =
            self.hydrate_enum_values(descriptor.value, location.values, key, fqn, file, package);
        let well_known = if self.is_well_known(package) {
            r#enum::WellKnownEnum::from_str(&name).ok()
        } else {
            None
        };

        let (key, fqn, name) = self.enums.lock()[key].hydrate(r#enum::Hydrate {
            name,
            values,
            container,
            location: location.detail,
            options: descriptor.options,
            reserved_names: descriptor.reserved_name,
            reserved_ranges: descriptor.reserved_range,
            special_fields: descriptor.special_fields,
            well_known,
        });
        ((key, fqn, name), iter)
    }

    fn hydrate_enum_values(
        &mut self,
        descriptors: Vec<EnumValueDescriptorProto>,
        locations: Vec<location::EnumValue>,
        r#enum: r#enum::Key,
        enum_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> (
        Vec<Hydrated<enum_value::Key>>,
        impl Iterator<Item = (FullyQualifiedName, node::Key)>,
    ) {
        assert_enum_value_locations(&enum_fqn, &locations, &descriptors);
        let mut values = Vec::with_capacity(descriptors.len());

        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(enum_fqn.clone()));
            let (key, fqn, name) =
                self.hydrate_enum_value(descriptor, fqn.clone(), location, r#enum, file, package);
            values.push((key, fqn, name));
        }
        let iter = values
            .clone()
            .into_iter()
            .map(|(key, fqn, _)| (fqn, key.into()));
        (values, iter)
    }

    fn hydrate_enum_value(
        &mut self,
        descriptor: EnumValueDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::EnumValue,
        r#enum: r#enum::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Hydrated<enum_value::Key> {
        let mut enum_values = self.enum_values.lock();
        let key = enum_values.get_or_insert_key_by_fqn(fqn);
        let (key, fqn, name) = enum_values[key].hydrate(enum_value::Hydrate {
            name: descriptor.name().into(),
            number: descriptor.number(),
            location: location.detail,
            options: descriptor.options,
            special_fields: descriptor.special_fields,
            r#enum,
            file,
            package,
        });
        (key, fqn, name)
    }

    fn hydrate_services(
        &mut self,
        descriptors: Vec<ServiceDescriptorProto>,
        locations: Vec<location::Service>,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<
        (
            Vec<Hydrated<service::Key>>,
            impl Iterator<Item = (FullyQualifiedName, node::Key)>,
        ),
        Error,
    > {
        assert_service_locations(&container_fqn, &locations, &descriptors);
        let mut services = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let (key, fqn, name) =
                self.hydrate_service(descriptor, fqn.clone(), location, file, package)?;
            services.push((key, fqn.clone(), name));
        }
        let iter = services
            .clone()
            .into_iter()
            .map(|(key, fqn, _)| (fqn, key.into()));

        Ok((services, iter))
    }

    fn hydrate_service(
        &mut self,
        descriptor: ServiceDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::Service,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Hydrated<service::Key>, Error> {
        todo!()
    }

    fn hydrate_methods(
        &mut self,
        descriptors: Vec<MethodDescriptorProto>,
        locations: Vec<location::Method>,
        container_fqn: FullyQualifiedName,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<method::Key>, Error> {
        todo!()
    }

    fn hydrate_method(&mut self) -> Result<Hydrated<method::Key>, Error> {
        todo!()
    }

    fn hydrate_fields(
        &mut self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::Field>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<
        (
            Vec<Hydrated<field::Key>>,
            impl Iterator<Item = (FullyQualifiedName, node::Key)>,
        ),
        Error,
    > {
        todo!()
    }

    fn hydrate_field(&mut self) -> Result<Hydrated<field::Key>, Error> {
        todo!()
    }

    fn hydrate_oneofs(
        &mut self,
        descriptors: Vec<OneofDescriptorProto>,
        locations: Vec<location::Oneof>,
        message: message::Key,
        message_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<
        (
            Vec<Hydrated<oneof::Key>>,
            impl Iterator<Item = (FullyQualifiedName, node::Key)>,
        ),
        Error,
    > {
        todo!()
    }

    fn hydrate_oneof(&mut self) -> Result<Hydrated<oneof::Key>, Error> {
        todo!()
    }

    fn hydrate_extensions(
        &mut self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::ExtensionBlock>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<
        (
            Vec<extension_block::Key>,
            Vec<Hydrated<extension::Key>>,
            impl Iterator<Item = (FullyQualifiedName, node::Key)>,
        ),
        Error,
    > {
        // let mut services = Vec::with_capacity(service.len());
        // for (i, descriptor) in nodes.service.into_iter().enumerate() {
        //     let index = to_i32(i);
        //     let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
        //     let node_path = vec![path::File::Service.as_i32(), index];
        //     let key = hydrate_service(Service {
        //         fqn: fqn.clone(),
        //         descriptor,
        //         location: locations.services[i],
        //         nodes,
        //         file: key,
        //         index,
        //         package,
        //     })?;
        //     services.push(key);
        //     nodes_by_fqn.insert(fqn, key.into());
        //     nodes_by_path.insert(node_path, key.into());
        // }
        todo!()
    }
}

fn assert_enum_locations(
    container_fqn: &str,
    locations: &[location::Enum],
    descriptors: &[EnumDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for enums in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_enum_value_locations(
    enum_fqn: &str,
    locations: &[location::EnumValue],
    descriptors: &[EnumValueDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for enum values in \"{enum_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_message_locations(
    container_fqn: &str,
    locations: &[location::Message],
    descriptors: &[DescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for messages in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_oneof_locations(
    message_fqn: &str,
    locations: &[location::Oneof],
    descriptors: &[OneofDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for oneofs in \"{message_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_service_locations(
    container_fqn: &str,
    locations: &[location::Service],
    descriptors: &[ServiceDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for services in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_method_locations(
    service_fqn: &str,
    locations: &[location::Method],
    descriptors: &[MethodDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for methods in \"{service_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_field_locations(
    message_fqn: &str,
    locations: &[location::Field],
    descriptors: &[FieldDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for fields in \"{message_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_extension_locations(
    container_fqn: &str,
    locations: &[location::ExtensionBlock],
    descriptors: &[FieldDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for extensions in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_file_locations(locations: &[location::File], descriptors: &[FileDescriptorProto]) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of file locations for files , expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}
