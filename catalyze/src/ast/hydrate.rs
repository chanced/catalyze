use std::{iter::once, path::PathBuf, sync::atomic::AtomicBool};

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
    r#enum, enum_value, extension, extension_block, field, file, location, message, method, oneof,
    package, path, service, ContainerKey, EnumTable, EnumValueTable, ExtensionBlockTable,
    ExtensionTable, FieldTable, FileTable, FullyQualifiedName, FullyQualifiedNames, Key,
    MessageTable, MethodTable, OneofTable, PackageTable, ServiceTable,
};

type Hydrated<K> = (K, FullyQualifiedName, Box<[i32]>);

pub(super) fn run(
    descriptors: Vec<FileDescriptorProto>,
    targets: &[String],
    fqns: &mut FullyQualifiedNames,
    ast_tables: &mut AstTables,
) -> Result<Vec<file::Key>, Error> {
    let mut files = Vec::with_capacity(descriptors.len());
    let mut by_fqn = HashMap::new();
    let mut all_by_fqn = ast_tables.by_fqn;
    ast_tables.by_fqn = &mut by_fqn;
    for (i, descriptor) in descriptors.into_iter().enumerate() {
        let (key, fqn, path) = hydrate_file(File {
            fqns,
            descriptor,
            targets,
            ast_tables,
        })?;
        files.push(key);
        all_by_fqn.extend(once((fqn, key.into())).chain(by_fqn.drain()));
    }
    Ok(files)
}

fn is_build_target(targets: &[String], name: &str) -> bool {
    targets.iter().any(|target| target == name)
}

fn hydrate_package(
    package: Option<String>,
    packages: &mut PackageTable,
    fqns: &mut FullyQualifiedNames,
) -> (Option<package::Key>, Option<FullyQualifiedName>) {
    let Some(package) = package else {
        return (None, None);
    };
    let fqn = fqns.for_package(&package);
    let (key, pkg) = packages.get_or_insert_by_fqn(fqn.clone());
    pkg.set_name(package);
    (Some(key), Some(fqn))
}

fn hydrate_file(hydrate: File) -> Result<Hydrated<file::Key>, Error> {
    let File {
        fqns,
        descriptor,
        targets,
        ast_tables,
    } = hydrate;
    // TODO: handle SpecialFields
    let mut by_path = HashMap::default();
    let tables = &mut Tables::new(ast_tables, &mut by_path);
    let name = descriptor.name.unwrap();
    let locations = location::File::new(descriptor.source_code_info.unwrap_or_else(|| {
        panic!("source_code_info not found on FileDescriptorProto for \"{name}\"")
    }))?;
    let (package, package_fqn) = hydrate_package(descriptor.package, tables.packages, fqns);
    let fqn = fqns.create(&name, package_fqn);
    let (key, file) = tables.files.get_or_insert_by_fqn(fqn.clone());
    let mut messages = hydrate_messages(
        descriptor.message_type,
        locations.messages,
        tables,
        fqns,
        fqn.clone(),
        key.into(),
        key,
        package,
    )?;
    let mut enums = hydrate_enums(
        descriptor.enum_type,
        locations.enums,
        tables,
        fqns,
        fqn.clone(),
        key.into(),
        key,
        package,
    )?;
    let services = hydrate_services(
        descriptor.service,
        locations.services,
        tables,
        fqns,
        fqn.clone(),
        key.into(),
        key,
        package,
    )?;
    let (extension_blocks, extensions) = hydrate_extensions(
        descriptor.extension,
        locations.extensions,
        tables,
        fqns,
        fqn.clone(),
        key.into(),
        key.into(),
        package,
    )?;
    let dependencies = hydrate_dependencies(
        descriptor.dependency,
        descriptor.public_dependency,
        descriptor.weak_dependency,
        tables,
        fqns,
        fqn.clone(),
        key.into(),
        package,
    )?;
    let file = &mut tables.files[key];

    file.hydrate(file::Hydrate {
        key,
        name,
        syntax: descriptor.syntax,
        options: descriptor.options,
        package,
        messages,
        enums,
        services,
        extensions,
        extension_blocks,
        dependencies,
        package_comments: locations.package,
        comments: locations.syntax,
        is_build_target,
    })
}

fn hydrate_dependencies(
    dependencies_by_fqn: Vec<String>,
    public_dependencies: Vec<i32>,
    weak_dependencies: Vec<i32>,
    tables: &mut Tables,
    fqns: &mut FullyQualifiedNames,
    file_fqn: FullyQualifiedName,
    dependent: file::Key,
    package: Option<package::Key>,
) -> Result<file::DependenciesInner, Error> {
    let mut all = Vec::with_capacity(dependencies_by_fqn.len());
    let mut weak = Vec::with_capacity(weak_dependencies.len());
    let mut public = Vec::with_capacity(public_dependencies.len());

    for (i, dependency) in dependencies_by_fqn.into_iter().enumerate() {
        let index = to_i32(i);
        let is_weak = weak_dependencies.contains(&index);
        let is_public = public_dependencies.contains(&index);
        let fqn = fqns.insert(FullyQualifiedName::from(dependency));
        let (dependency_key, dependency_file) = tables.files.get_or_insert_by_fqn(fqn.clone());

        dependency_file.add_dependent(DependentInner {
            is_used: bool::default(),
            is_public,
            is_weak,
            dependent,
            dependency: dependency_key,
        });
        all.push(DependencyInner {
            is_used: bool::default(),
            is_public,
            is_weak,
            dependent,
            dependency: dependency_key,
        });
        if is_public {
            public.push(dependency_key);
        }
        if is_weak {
            weak.push(dependency_key);
        }
    }
    Ok(file::DependenciesInner { all, public, weak })
}

fn hydrate_enums(
    descriptors: Vec<EnumDescriptorProto>,
    locations: Vec<location::Enum>,
    tables: &mut Tables,
    fqns: &mut FullyQualifiedNames,
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
            tables,
        })?;
        tables.by_fqn.insert(fqn, key.into());
        tables.by_path.insert(path, key.into());
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
        tables,
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
    let (key, enm) = tables.enums.get_or_insert_by_fqn(fqn.clone());
    let values = hydrate_enum_values(
        value,
        location.enum_values,
        tables,
        fqn.clone(),
        key.into(),
        file,
        package,
    )?;
    enm.hydrate(values, options, reserved_name, reserved_range)
}

fn hydrate_enum_values(
    descriptors: Vec<EnumValueDescriptorProto>,
    locations: Vec<location::EnumValue>,
    tables: &mut Tables<'_>,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<enum_value::Key>, Error> {
    todo!()
}

fn hydrate_messages(
    descriptors: Vec<DescriptorProto>,
    locations: Vec<location::Message>,
    tables: &mut Tables,
    fqns: &mut FullyQualifiedNames,
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
            tables,
        })?;
        tables.by_fqn.insert(fqn, key.into());
        tables.by_path.insert(node_path, key.into());
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
        tables: nodes,
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
    tables: &mut Tables,
    fqns: &mut FullyQualifiedNames,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<Vec<service::Key>, Error> {
    let mut services = Vec::with_capacity(descriptors.len());
    for (i, descriptor) in descriptors.into_iter().enumerate() {
        let index = to_i32(i);
        let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn));
        let node_path = Box::new([path::File::Service.as_i32(), index]);
        let (key, fqn, path) = hydrate_service(Service {
            fqn: fqn.clone(),
            descriptor,
            location: locations[i],
            tables,
            file,
            index,
            package,
        })?;
        services.push(key);
        tables.by_fqn.insert(fqn, key.into());
        tables.by_path.insert(node_path, key.into());
    }
    todo!()
}

fn hydrate_service(hydrate: Service) -> Result<Hydrated<service::Key>, Error> {
    todo!()
}

fn hydrate_methods(
    descriptors: Vec<MethodDescriptorProto>,
    locations: Vec<location::Method>,
    nodes: &mut Tables,
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
    nodes: &mut Tables,
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
    nodes: &mut Tables,
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

fn hydrate_extensions(
    descriptors: Vec<FieldDescriptorProto>,
    locations: Vec<location::ExtensionBlock>,
    tables: &mut Tables,
    fqns: &mut FullyQualifiedNames,
    container_fqn: FullyQualifiedName,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
) -> Result<(Vec<extension_block::Key>, Vec<extension::Key>), Error> {
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
#[derive(Debug)]
pub(super) struct AstTables<'hydrate> {
    by_fqn: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    packages: &'hydrate mut PackageTable,
    files: &'hydrate mut FileTable,
    messages: &'hydrate mut MessageTable,
    enums: &'hydrate mut EnumTable,
    enum_values: &'hydrate mut EnumValueTable,
    services: &'hydrate mut ServiceTable,
    methods: &'hydrate mut MethodTable,
    fields: &'hydrate mut FieldTable,
    oneofs: &'hydrate mut OneofTable,
    extensions: &'hydrate mut ExtensionTable,
    extension_blocks: &'hydrate mut ExtensionBlockTable,
}
impl<'hydrate> AstTables<'hydrate> {
    pub(crate) fn new(ast: &'hydrate mut super::Ast) -> AstTables<'hydrate> {
        AstTables {
            by_fqn: &mut ast.nodes,
            packages: &mut ast.packages,
            files: &mut ast.files,
            messages: &mut ast.messages,
            enums: &mut ast.enums,
            enum_values: &mut ast.enum_values,
            services: &mut ast.services,
            methods: &mut ast.methods,
            fields: &mut ast.fields,
            oneofs: &mut ast.oneofs,
            extensions: &mut ast.extensions,
            extension_blocks: &mut ast.extension_blocks,
        }
    }
}
#[derive(Debug)]
struct Tables<'hydrate> {
    by_fqn: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    by_path: &'hydrate mut HashMap<Box<[i32]>, Key>,
    packages: &'hydrate mut PackageTable,
    files: &'hydrate mut FileTable,
    messages: &'hydrate mut MessageTable,
    enums: &'hydrate mut EnumTable,
    enum_values: &'hydrate mut EnumValueTable,
    services: &'hydrate mut ServiceTable,
    methods: &'hydrate mut MethodTable,
    fields: &'hydrate mut FieldTable,
    oneofs: &'hydrate mut OneofTable,
    extensions: &'hydrate mut ExtensionTable,
    extension_blocks: &'hydrate mut ExtensionBlockTable,
}

impl<'hydrate> Tables<'hydrate> {
    fn new(
        ast_tables: &'hydrate mut AstTables,
        by_path: &'hydrate mut HashMap<Box<[i32]>, Key>,
    ) -> Tables<'hydrate> {
        Tables {
            by_fqn: &mut ast_tables.by_fqn,
            by_path,
            packages: ast_tables.packages,
            files: ast_tables.files,
            messages: ast_tables.messages,
            enums: ast_tables.enums,
            enum_values: ast_tables.enum_values,
            services: ast_tables.services,
            methods: ast_tables.methods,
            fields: ast_tables.fields,
            oneofs: ast_tables.oneofs,
            extensions: ast_tables.extensions,
            extension_blocks: ast_tables.extension_blocks,
        }
    }
}

struct File<'hydrate> {
    fqns: &'hydrate mut FullyQualifiedNames,
    descriptor: FileDescriptorProto,
    targets: &'hydrate [String],
    ast_tables: &'hydrate mut AstTables<'hydrate>,
}

struct Enum<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: EnumDescriptorProto,
    index: i32,
    location: location::Enum,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct EnumValue<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: EnumValueDescriptorProto,
    index: i32,
    location: location::EnumValue,
    r#enum: r#enum::Key,
    file: file::Key,
    package: Option<package::Key>,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct Message<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: DescriptorProto,
    index: i32,
    location: location::Message,
    container: ContainerKey,
    file: file::Key,
    package: Option<package::Key>,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct ExtensionBlock<'hydrate> {
    file: file::Key,
    package: Option<package::Key>,
    container: ContainerKey,
    extensions: &'hydrate mut std::vec::IntoIter<FieldDescriptorProto>,
    location: location::ExtensionBlock,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct Extension<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: FieldDescriptorProto,
    index: i32,
    location: location::Field,
    container: ContainerKey,
    file: file::Key,
    block: extension::BlockKey,
    package: Option<package::Key>,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct Service<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: ServiceDescriptorProto,
    index: i32,
    location: location::Service,
    file: file::Key,
    package: Option<package::Key>,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct Field<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: FieldDescriptorProto,
    index: i32,
    location: location::Field,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct Oneof<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: OneofDescriptorProto,
    index: i32,
    location: location::Oneof,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
    tables: &'hydrate mut Tables<'hydrate>,
}

struct Method<'hydrate> {
    fqn: FullyQualifiedName,
    descriptor: MethodDescriptorProto,
    index: i32,
    location: location::Method,
    service_key: service::Key,
    file: file::Key,
    package: Option<package::Key>,
    nodes: &'hydrate mut Tables<'hydrate>,
}
