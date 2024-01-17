use std::{iter::once, path::PathBuf};

use ahash::HashMapExt;
use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
    SourceCodeInfo,
};

use crate::{
    ast::file::{DependencyInner, DependentInner},
    error::Error,
    to_i32, HashMap,
};

use super::{
    r#enum, enum_value, extension, field, file, location, message, method, oneof, package, path,
    service, ContainerKey, EnumTable, EnumValueTable, ExtensionGroupTable, ExtensionTable,
    FieldTable, FileTable, FullyQualifiedName, Key, MessageTable, MethodTable, OneofTable,
    PackageTable, ServiceTable,
};

type Hydrated<K> = (K, FullyQualifiedName, Box<[i32]>);

pub(super) fn run(
    descriptors: Vec<FileDescriptorProto>,
    targets: &[String],
    nodes: &mut AstTables,
) -> Result<Vec<file::Key>, Error> {
    let all_nodes_by_fqn = nodes.by_fqn;
    let mut files = Vec::with_capacity(descriptors.len());
    let mut nodes_by_fqn = HashMap::new();
    let mut nodes_by_path = HashMap::new();
    for (i, descriptor) in descriptors.into_iter().enumerate() {
        nodes.by_fqn = &mut nodes_by_fqn;
        let (key, fqn, path) = hydrate_file(File {
            descriptor,
            targets,
            nodes,
        })?;
        files.push(key);
        let key = key.into();
        all_nodes_by_fqn.extend(once((fqn, key)).chain(nodes_by_fqn.drain()));
    }
    Ok(files)
}

fn is_build_target(targets: &[PathBuf], name: &str) -> bool {
    targets.iter().any(|target| target.as_os_str() == name)
}

fn hydrate_package(package: Option<String>, packages: &mut PackageTable) -> Option<package::Key> {
    package.map(|pkg| {
        let fqn = FullyQualifiedName::from_package_name(&pkg);
        let (key, pkg) = packages.get_or_insert_by_fqn(fqn);
        pkg.set_name(pkg);
        key
    })
}

fn hydrate_file(hydrate: File) -> Result<Hydrated<file::Key>, Error> {
    let File {
        descriptor,
        targets,
        nodes,
    } = hydrate;
    let FileDescriptorProto {
        name,
        package,
        dependency,
        public_dependency,
        weak_dependency,
        message_type,
        enum_type,
        service,
        extension,
        options,
        source_code_info,
        syntax,
        special_fields,
    } = descriptor;

    let locations = location::File::new(source_code_info.unwrap_or_else(|| {
        panic!("source_code_info not found on FileDescriptorProto for \"{name:?}\"")
    }))?;

    let name = name.unwrap();
    let fqn = FullyQualifiedName::new(&name, package.as_ref().map(|(_, pkg)| pkg.fqn().clone()));

    let (key, file) = nodes.files.get_or_insert_by_fqn(fqn.clone());

    let package = hydrate_package(package, nodes.packages);

    let mut nodes_by_fqn = HashMap::default();
    let mut nodes_by_path = HashMap::default();
    let mut messages = hydrate_messages(
        message_type,
        locations.messages,
        nodes,
        fqn.clone(),
        key.into(),
        key,
        package,
    )?;
    let mut enums = hydrate_enums(
        enum_type,
        locations.enums,
        nodes,
        fqn.clone(),
        key.into(),
        key,
        package,
    )?;
    let services = hydrate_services(
        service,
        locations.services,
        hydrate.nodes,
        fqn.clone(),
        key.into(),
        key,
        package,
    )?;
    let (extension_groups, extensions) = hydrate_extensions(
        extension,
        locations.extensions,
        nodes,
        fqn.clone(),
        key.into(),
        key.into(),
        package,
    )?;
    let dependencies = hydrate_dependencies(
        dependency,
        public_dependency,
        weak_dependency,
        nodes,
        fqn.clone(),
        key.into(),
        package,
    )?;

    let file = &mut nodes.files[key];

    file.hydrate_options(options.unwrap_or_default(), is_build_target(targets, &name));
    file.set_package(package);
    file.set_name_and_path(name);
    file.set_syntax(syntax)?;
    file.set_dependencies(dependencies);
    file.set_messages(messages);
    file.set_enums(enums);
    file.set_services(services);
    file.set_defined_extensions(extensions);
    file.set_package(package);
    todo!()
}

fn hydrate_dependencies(
    dependencies_by_fqn: Vec<String>,
    public_dependencies: Vec<i32>,
    weak_dependencies: Vec<i32>,
    nodes: &mut Nodes,
    file_fqn: FullyQualifiedName,
    dependent: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<DependencyInner>, Error> {
    let mut dependencies = Vec::with_capacity(dependencies_by_fqn.len());

    for (i, dependency) in dependencies_by_fqn.into_iter().enumerate() {
        let index = to_i32(i);
        let is_weak = weak_dependencies.contains(&index);
        let is_public = public_dependencies.contains(&index);
        let fqn = FullyQualifiedName(dependency);
        let (dependency_key, dependency_file) = nodes.files.get_or_insert_by_fqn(fqn.clone());

        dependency_file.add_dependent(DependentInner {
            is_used: bool::default(),
            is_public,
            is_weak,
            dependent,
            dependency: dependency_key,
        });
        dependencies.push(DependencyInner {
            is_used: bool::default(),
            is_public,
            is_weak,
            dependent,
            dependency: dependency_key,
        });
    }
    todo!()
}

fn hydrate_extensions(
    descriptors: Vec<FieldDescriptorProto>,
    locations: Vec<location::ExtensionGroup>,
    nodes: &mut Nodes,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<(Vec<extension::GroupKey>, Vec<extension::Key>), Error> {
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

fn hydrate_enums(
    descriptors: Vec<EnumDescriptorProto>,
    locations: Vec<location::Enum>,
    nodes: &mut Nodes,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<r#enum::Key>, Error> {
    let mut enums = Vec::with_capacity(descriptors.len());
    for (i, descriptor) in descriptors.into_iter().enumerate() {
        let index = to_i32(i);
        let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn));
        let node_path = vec![path::File::Enum.as_i32(), index];
        let (key, fqn, path) = hydrate_enum(Enum {
            fqn: fqn.clone(),
            descriptor,
            index,
            location: locations[i],
            container,
            file,
            package,
            nodes,
        })?;
        nodes.by_fqn.insert(fqn, key.into());
        nodes.by_path.insert(path, key.into());
        enums.push(key);
    }
    Ok(enums)
}

fn hydrate_enum(hydrate: Enum) -> Result<Hydrated<r#enum::Key>, Error> {
    let Enum {
        fqn,
        descriptor,
        index,
        location,
        container,
        file,
        package,
        nodes,
    } = hydrate;
    let EnumDescriptorProto {
        name,
        value,
        options,
        reserved_range,
        reserved_name,
        special_fields,
    } = descriptor;

    let name = name.unwrap_or_default();
    let (key, enm) = nodes.enums.get_or_insert_by_fqn(fqn.clone());
    enm.hydrate_options(options.unwrap_or_default());
    enm.set_container(container);
    enm.set_name(name);
    let enum_values = hydrate_enum_values(
        value,
        location.enum_values,
        nodes,
        fqn.clone(),
        key.into(),
        file,
        package,
    )?;
    Ok(key)
}

fn hydrate_enum_values(
    descriptors: Vec<EnumValueDescriptorProto>,
    locations: Vec<location::EnumValue>,
    nodes: &mut Nodes<'_>,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<enum_value::Key>, Error> {
}

fn hydrate_messages(
    descriptors: Vec<DescriptorProto>,
    locations: Vec<location::Message>,
    nodes: &mut Nodes,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<message::Key>, Error> {
    let mut messages = Vec::with_capacity(descriptors.len());
    for (i, descriptor) in descriptors.into_iter().enumerate() {
        let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn));
        let index = to_i32(i);
        let location = locations[i];
        let node_path = location.detail.path.clone();
        let key = hydrate_message(Message {
            fqn: fqn.clone(),
            descriptor,
            index,
            location,
            container,
            file,
            package,
            nodes,
        })?;
        nodes.by_fqn.insert(fqn, key.into());
        nodes.by_path.insert(node_path, key.into());
        messages.push(key);
    }
    Ok(messages)
}

pub(super) fn hydrate_message(hydrate: Message) -> Result<message::Key, Error> {
    // TODO: remove destructor once all fields have been used

    let Message {
        fqn,
        descriptor,
        index,
        location,
        container,
        file,
        package,
        nodes,
    } = hydrate;

    let DescriptorProto {
        name,
        field,
        extension,
        nested_type,
        enum_type,
        extension_range,
        oneof_decl,
        options,
        reserved_range,
        reserved_name,
        special_fields,
    } = descriptor;

    let name = name.unwrap_or_default();
    let (key, msg) = nodes.messages.get_or_insert_by_fqn(fqn.clone());
    msg.hydrate_options(options.unwrap_or_default());
    msg.set_container(container);
    msg.set_name(name);
    let messages = hydrate_messages(
        nested_type,
        location.messages,
        nodes,
        fqn.clone(),
        key.into(),
        file,
        package,
    );

    Ok(key)
}

fn hydrate_services(
    descriptors: Vec<ServiceDescriptorProto>,
    locations: Vec<location::Service>,
    nodes: &mut Nodes,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<service::Key>, Error> {
    let mut services = Vec::with_capacity(descriptors.len());
    for (i, descriptor) in descriptors.into_iter().enumerate() {
        let index = to_i32(i);
        let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn));
        let node_path = vec![path::File::Service.as_i32(), index];
        let (key, fqn, path) = hydrate_service(Service {
            fqn: fqn.clone(),
            descriptor,
            location: locations[i],
            nodes,
            file,
            index,
            package,
        })?;
        services.push(key);
        nodes.by_fqn.insert(fqn, key.into());
        nodes.by_path.insert(node_path, key.into());
    }
    todo!()
}

fn hydrate_service(hydrate: Service) -> Result<Hydrated<service::Key>, Error> {
    todo!()
}

fn hydrate_methods(
    descriptors: Vec<MethodDescriptorProto>,
    locations: Vec<location::Method>,
    nodes: &mut Nodes,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<method::Key>, Error> {
    todo!()
}

fn hydrate_method(hydrate: Method) -> Result<Hydrated<method::Key>, Error> {
    todo!()
}

fn hydrate_fields(
    descriptors: Vec<FieldDescriptorProto>,
    locations: Vec<location::Field>,
    nodes: &mut Nodes,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<field::Key>, Error> {
    todo!()
}

fn hydrate_field(hydrate: Field) -> Result<Hydrated<field::Key>, Error> {
    todo!()
}

fn hydrate_oneofs(
    descriptors: Vec<OneofDescriptorProto>,
    locations: Vec<location::Oneof>,
    nodes: &mut Nodes,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<oneof::Key>, Error> {
    todo!()
}

fn hydrate_oneof(hydrate: Oneof) -> Result<Hydrated<oneof::Key>, Error> {
    todo!()
}

#[derive(Debug)]
pub(super) struct AstTables<'hydrate> {
    pub(super) by_fqn: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    pub(super) packages: &'hydrate mut PackageTable,
    pub(super) files: &'hydrate mut FileTable,
    pub(super) messages: &'hydrate mut MessageTable,
    pub(super) enums: &'hydrate mut EnumTable,
    pub(super) enum_values: &'hydrate mut EnumValueTable,
    pub(super) services: &'hydrate mut ServiceTable,
    pub(super) methods: &'hydrate mut MethodTable,
    pub(super) fields: &'hydrate mut FieldTable,
    pub(super) oneofs: &'hydrate mut OneofTable,
    pub(super) extensions: &'hydrate mut ExtensionTable,
    pub(super) extension_groups: &'hydrate mut ExtensionGroupTable,
}
#[derive(Debug)]
pub(super) struct Nodes<'hydrate> {
    pub(super) by_fqn: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    pub(super) by_path: &'hydrate mut HashMap<Box<[i32]>, Key>,
    pub(super) packages: &'hydrate mut PackageTable,
    pub(super) files: &'hydrate mut FileTable,
    pub(super) messages: &'hydrate mut MessageTable,
    pub(super) enums: &'hydrate mut EnumTable,
    pub(super) enum_values: &'hydrate mut EnumValueTable,
    pub(super) services: &'hydrate mut ServiceTable,
    pub(super) methods: &'hydrate mut MethodTable,
    pub(super) fields: &'hydrate mut FieldTable,
    pub(super) oneofs: &'hydrate mut OneofTable,
    pub(super) extensions: &'hydrate mut ExtensionTable,
    pub(super) extension_groups: &'hydrate mut ExtensionGroupTable,
}

pub(super) struct File<'hydrate> {
    pub(super) descriptor: FileDescriptorProto,
    pub(super) targets: &'hydrate [PathBuf],
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct Enum<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: EnumDescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::Enum,
    pub(super) container: ContainerKey,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct EnumValue<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: EnumValueDescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::EnumValue,
    pub(super) r#enum: r#enum::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct Message<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: DescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::Message,
    pub(super) container: ContainerKey,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct ExtensionGroup<'hydrate> {
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) container: ContainerKey,
    pub(super) extensions: &'hydrate mut std::vec::IntoIter<FieldDescriptorProto>,
    pub(super) location: location::ExtensionGroup,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct Extension<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: FieldDescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::Field,
    pub(super) container: ContainerKey,
    pub(super) file: file::Key,
    pub(super) group: extension::GroupKey,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct Service<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: ServiceDescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::Service,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct Field<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: FieldDescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::Field,
    pub(super) message: message::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct Oneof<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: OneofDescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::Oneof,
    pub(super) message: message::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}

pub(super) struct Method<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: MethodDescriptorProto,
    pub(super) index: i32,
    pub(super) location: location::Method,
    pub(super) service_key: service::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes: &'hydrate mut Nodes<'hydrate>,
}
