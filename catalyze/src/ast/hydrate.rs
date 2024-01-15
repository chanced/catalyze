use std::path::PathBuf;

use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};

use crate::HashMap;

use super::{r#enum, file, message, package, service, ContainerKey, FullyQualifiedName, Key};

pub(super) struct File<'hydrate> {
    pub(super) descriptor: FileDescriptorProto,
    pub(super) all_nodes: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    pub(super) targets: &'hydrate [PathBuf],
}

pub(super) struct Enum<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: EnumDescriptorProto,
    pub(super) index: i32,
    pub(super) rel_pos: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) nodes_by_fqn: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    pub(super) nodes_by_path: &'hydrate mut HashMap<Vec<i32>, Key>,
    pub(super) container: ContainerKey,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

pub(super) struct EnumValue {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: EnumValueDescriptorProto,
    pub(super) index: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) r#enum: r#enum::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

pub(super) struct Message<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: DescriptorProto,
    pub(super) index: i32,
    pub(super) rel_pos: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) nodes_by_fqn: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    pub(super) nodes_by_path: &'hydrate mut HashMap<Vec<i32>, Key>,
    pub(super) container: ContainerKey,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

pub(super) struct Extension {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: FieldDescriptorProto,
    pub(super) index: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) container: ContainerKey,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

pub(super) struct Service<'hydrate> {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: ServiceDescriptorProto,
    pub(super) index: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) nodes_by_fqn: &'hydrate mut HashMap<FullyQualifiedName, Key>,
    pub(super) nodes_by_path: &'hydrate mut HashMap<Vec<i32>, Key>,
}

pub(super) struct Field {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: FieldDescriptorProto,
    pub(super) index: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) message: message::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

pub(super) struct Oneof {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: OneofDescriptorProto,
    pub(super) index: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) message: message::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

pub(super) struct Method {
    pub(super) fqn: FullyQualifiedName,
    pub(super) descriptor: MethodDescriptorProto,
    pub(super) index: i32,
    pub(super) node_path: Vec<i32>,
    pub(super) service_key: service::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}
