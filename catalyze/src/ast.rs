pub mod access;
pub mod r#enum;
pub mod enum_value;
pub mod extension;
pub mod field;
pub mod file;
pub mod message;
pub mod method;
pub mod oneof;
pub mod package;
pub mod path;
pub mod reference;
pub mod service;

mod hydrate;
mod location;

use std::{
    borrow::Cow,
    fmt,
    iter::Empty,
    ops::{Deref, Index, IndexMut},
    path::PathBuf,
};

use ahash::{HashMapExt, HashSetExt};
use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};
use slotmap::SlotMap;

use crate::{
    ast::file::{DependencyInner, DependentInner},
    error::Error,
    HashMap, HashSet,
};
use r#enum::{Enum, WellKnownEnum};
use enum_value::EnumValue;
use extension::Extension;
use field::Field;
use file::File;
use message::{Message, WellKnownMessage};
use method::Method;
use oneof::Oneof;
use package::Package;
use service::Service;

/// Zero-based spans of a node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start_line: i32,
    pub start_column: i32,
    pub end_line: i32,
    pub end_column: i32,
}
impl Span {
    fn new(span: &[i32]) -> Result<Self, ()> {
        match span.len() {
            3 => Ok(Self {
                start_line: span[0],
                start_column: span[1],
                end_line: span[0],
                end_column: span[2],
            }),
            4 => Ok(Self {
                start_line: span[0],
                start_column: span[1],
                end_line: span[2],
                end_column: span[3],
            }),
            _ => Err(()),
        }
    }
    pub fn start_line(&self) -> i32 {
        self.start_line
    }
    pub fn start_column(&self) -> i32 {
        self.start_column
    }
    pub fn end_line(&self) -> i32 {
        self.end_line
    }
    pub fn end_column(&self) -> i32 {
        self.end_column
    }
}

trait FromFqn {
    fn from_fqn(fqn: FullyQualifiedName) -> Self;
}

#[doc(hidden)]
trait Get<K, T> {
    fn get(&self, key: K) -> &T;
}

trait Resolve<T> {
    fn resolve(&self) -> &T;
}

struct Resolver<'ast, K, I> {
    ast: &'ast Ast,
    key: K,
    marker: std::marker::PhantomData<I>,
}

impl<'ast, K, I> Resolver<'ast, K, I> {
    const fn new(key: K, ast: &'ast Ast) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> Clone for Resolver<'ast, K, I>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Self {
            ast: self.ast,
            key: self.key.clone(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> From<(K, &'ast Ast)> for Resolver<'ast, K, I> {
    fn from((key, ast): (K, &'ast Ast)) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> Copy for Resolver<'ast, K, I> where K: Copy {}

impl<'ast, K, I> PartialEq for Resolver<'ast, K, I>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<'ast, K, I> Eq for Resolver<'ast, K, I> where K: Eq {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Package(package::Key),
    File(file::Key),
    Message(message::Key),
    Enum(r#enum::Key),
    EnumValue(enum_value::Key),
    Service(service::Key),
    Method(method::Key),
    Field(field::Key),
    Oneof(oneof::Key),
    Extension(extension::Key),
}

impl From<package::Key> for Key {
    fn from(key: package::Key) -> Self {
        Self::Package(key)
    }
}
impl From<file::Key> for Key {
    fn from(key: file::Key) -> Self {
        Self::File(key)
    }
}
impl From<message::Key> for Key {
    fn from(key: message::Key) -> Self {
        Self::Message(key)
    }
}
impl From<r#enum::Key> for Key {
    fn from(key: r#enum::Key) -> Self {
        Self::Enum(key)
    }
}
impl From<enum_value::Key> for Key {
    fn from(key: enum_value::Key) -> Self {
        Self::EnumValue(key)
    }
}
impl From<service::Key> for Key {
    fn from(key: service::Key) -> Self {
        Self::Service(key)
    }
}
impl From<method::Key> for Key {
    fn from(key: method::Key) -> Self {
        Self::Method(key)
    }
}
impl From<field::Key> for Key {
    fn from(key: field::Key) -> Self {
        Self::Field(key)
    }
}
impl From<oneof::Key> for Key {
    fn from(key: oneof::Key) -> Self {
        Self::Oneof(key)
    }
}
impl From<extension::Key> for Key {
    fn from(key: extension::Key) -> Self {
        Self::Extension(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ContainerKey {
    Message(message::Key),
    File(file::Key),
}
impl Default for ContainerKey {
    fn default() -> Self {
        Self::File(file::Key::default())
    }
}

impl From<message::Key> for ContainerKey {
    fn from(key: message::Key) -> Self {
        Self::Message(key)
    }
}
impl From<file::Key> for ContainerKey {
    fn from(key: file::Key) -> Self {
        Self::File(key)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Container<'ast> {
    Message(Message<'ast>),
    File(File<'ast>),
}

impl<'ast> From<File<'ast>> for Container<'ast> {
    fn from(v: File<'ast>) -> Self {
        Self::File(v)
    }
}

impl<'ast> From<Message<'ast>> for Container<'ast> {
    fn from(v: Message<'ast>) -> Self {
        Self::Message(v)
    }
}

impl<'ast> Container<'ast> {
    /// Returns `true` if the container is [`Message`].
    ///
    /// [`Message`]: Container::Message
    #[must_use]
    pub const fn is_message(self) -> bool {
        matches!(self, Self::Message(..))
    }

    #[must_use]
    pub fn as_message(self) -> Option<Message<'ast>> {
        if let Self::Message(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_message(self) -> Result<Message<'ast>, Self> {
        if let Self::Message(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the container is [`File`].
    ///
    /// [`File`]: Container::File
    #[must_use]
    pub fn is_file(self) -> bool {
        matches!(self, Self::File(..))
    }

    #[must_use]
    pub fn as_file(self) -> Option<File<'ast>> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_file(self) -> Result<File<'ast>, Self> {
        if let Self::File(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug, Clone)]
struct Table<K, V>
where
    K: slotmap::Key,
{
    map: SlotMap<K, V>,
    lookup: HashMap<FullyQualifiedName, K>,
    order: Vec<K>,
}
impl<K, V> Default for Table<K, V>
where
    K: slotmap::Key,
{
    fn default() -> Self {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::default(),
            order: Vec::default(),
        }
    }
}
impl<K, V> Table<K, V>
where
    K: slotmap::Key,
    V: access::FullyQualifiedName,
{
    fn get(&self, key: K) -> Option<&V> {
        self.map.get(key)
    }
    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.map.get_mut(key)
    }
    fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.order.iter().map(move |key| (*key, &self.map[*key]))
    }
    fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.map.iter_mut()
    }
    fn keys(&self) -> impl '_ + Iterator<Item = K> {
        self.order.iter().copied()
    }
    fn get_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&V> {
        self.lookup.get(fqn).map(|key| &self.map[*key])
    }
    fn get_mut_by_fqn(&mut self, fqn: &FullyQualifiedName) -> Option<&mut V> {
        self.lookup.get(fqn).map(|key| &mut self.map[*key])
    }
}

impl<K, V> Index<K> for Table<K, V>
where
    K: slotmap::Key,
{
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        &self.map[key]
    }
}
impl<K, V> IndexMut<K> for Table<K, V>
where
    K: slotmap::Key,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        &mut self.map[key]
    }
}

impl<K, V> Table<K, V>
where
    K: slotmap::Key,
    V: From<FullyQualifiedName> + access::Key<Key = K>,
{
    pub fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::default(),
            order: Vec::new(),
        }
    }
    pub fn get_or_insert_by_fqn(&mut self, fqn: FullyQualifiedName) -> (K, &mut V) {
        let key = *self
            .lookup
            .entry(fqn.clone())
            .or_insert_with(|| self.map.insert(fqn.into()));
        let value = &mut self.map[key];
        if value.key() != key {
            value.set_key(key);
        }
        (key, value)
    }
}

#[derive(Debug, Default)]
pub struct Ast {
    packages: Table<package::Key, package::Inner>,
    files: Table<file::Key, file::Inner>,
    messages: Table<message::Key, message::Inner>,
    enums: Table<r#enum::Key, r#enum::Inner>,
    enum_values: Table<enum_value::Key, enum_value::Inner>,
    services: Table<service::Key, service::Inner>,
    methods: Table<method::Key, method::Inner>,
    fields: Table<field::Key, field::Inner>,
    oneofs: Table<oneof::Key, oneof::Inner>,
    extensions: Table<extension::Key, extension::Inner>,
    extension_groups: Table<extension::GroupKey, extension::GroupInner>,
    nodes: HashMap<FullyQualifiedName, Key>,
}

impl Ast {
    fn new(input: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Self, Error> {
        let targets = targets.iter().map(PathBuf::from).collect::<Vec<_>>();
        let mut this = Self::default();
        let mut all_nodes = HashMap::new();
        for descriptor in input {
            this.hydrate_file(hydrate::File {
                descriptor,
                all_nodes: &mut all_nodes,
                targets: &targets,
            })?;
        }
        Ok(this)
    }

    fn hydrate_file(&mut self, hydrate: hydrate::File) -> Result<file::Key, Error> {
        // TODO: remove destruction once complete
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
        } = hydrate.descriptor;

        let package = package.as_ref().map(|pkg| {
            self.packages
                .get_or_insert_by_fqn(FullyQualifiedName::from_package_name(pkg))
        });

        let name = name.unwrap_or_default();
        let fqn =
            FullyQualifiedName::new(&name, package.as_ref().map(|(_, pkg)| pkg.fqn().clone()));

        let is_build_target = hydrate
            .targets
            .iter()
            .any(|target| target.as_os_str() == name.as_str());

        let (key, file) = self.files.get_or_insert_by_fqn(fqn.clone());

        let package = if let Some((pkg_key, pkg)) = package {
            pkg.add_file(key);
            Some(pkg_key)
        } else {
            None
        };

        file.set_key(key);
        file.set_package(package);
        file.set_name_and_path(name);
        file.set_syntax(syntax)?;
        file.hydrate_options(options.unwrap_or_default(), is_build_target);

        let mut nodes_by_fqn = HashMap::new();
        let mut nodes_by_path = HashMap::new();

        let mut insert = |fqn: FullyQualifiedName, path: Vec<i32>, key: Key| {
            nodes_by_fqn.insert(fqn, key);
            nodes_by_path.insert(path, key);
        };

        let mut messages = Vec::with_capacity(message_type.len());
        for (i, descriptor) in message_type.into_iter().enumerate() {
            let index = i as i32;
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
            let node_path = vec![path::File::Message.as_i32(), index];
            let key = self.hydrate_message(hydrate::Message {
                fqn: fqn.clone(),
                descriptor,
                index,
                rel_pos: path::File::Message.as_i32(),
                node_path: node_path.clone(),
                nodes_by_fqn: &mut nodes_by_fqn,
                nodes_by_path: &mut nodes_by_path,
                container: key.into(),
                file: key,
                package,
            })?;
            insert(fqn, node_path, key.into());
            messages.push(key);
        }

        let mut enums = Vec::with_capacity(enum_type.len());
        for (i, descriptor) in enum_type.into_iter().enumerate() {
            let index = i as i32;
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
            let node_path = vec![path::File::Enum.as_i32(), index];
            let key = self.hydrate_enum(hydrate::Enum {
                fqn: fqn.clone(),
                descriptor,
                index,
                rel_pos: path::File::Enum.as_i32(),
                node_path: node_path.clone(),
                nodes_by_fqn: &mut nodes_by_fqn,
                nodes_by_path: &mut nodes_by_path,
                container: key.into(),
                file: key,
                package,
            })?;
            insert(fqn, node_path, key.into());
            enums.push(key);
        }

        let mut services = Vec::with_capacity(service.len());
        for (i, descriptor) in service.into_iter().enumerate() {
            let index = i as i32;
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
            let node_path = vec![path::File::Service.as_i32(), index];
            let key = self.hydrate_service(hydrate::Service {
                fqn: fqn.clone(),
                descriptor,
                node_path: node_path.clone(),
                nodes_by_fqn: &mut nodes_by_fqn,
                nodes_by_path: &mut nodes_by_path,
                file: key,
                index,
                package,
            })?;
            services.push(key);
            insert(fqn, node_path, key.into());
        }

        let mut extensions = Vec::with_capacity(extension.len());
        for (i, descriptor) in extension.into_iter().enumerate() {
            let index = i as i32;
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
            let node_path = vec![path::File::Extension.as_i32(), index];
            let key = self.hydrate_extension(hydrate::Extension {
                fqn: fqn.clone(),
                descriptor,
                index,
                node_path: node_path.clone(),
                container: key.into(),
                file: key,
                package,
            })?;
            insert(fqn, node_path, key.into());
            extensions.push(key);
        }

        let mut dependencies = Vec::with_capacity(dependency.len());

        for (idx, dependency) in dependency.into_iter().enumerate() {
            let idx = idx as i32;
            let is_weak = weak_dependency.contains(&idx);
            let is_public = public_dependency.contains(&idx);
            let fqn = FullyQualifiedName(dependency);
            let (dependency_key, dependency_file) = self.files.get_or_insert_by_fqn(fqn.clone());
            dependency_file.add_dependent(DependentInner {
                is_used: bool::default(),
                is_public,
                is_weak,
                dependent: key,
                dependency: dependency_key,
            });
            dependencies.push(DependencyInner {
                is_used: bool::default(),
                is_public,
                is_weak,
                dependent: key,
                dependency: dependency_key,
            });
        }
        let file = &mut self.files[key];
        file.set_dependencies(dependencies);
        file.set_messages(messages);
        file.set_dependencies(dependencies);
        file.set_enums(enums);
        file.set_services(services);
        file.set_defined_extensions(extensions);
        file.set_package(package);
        // TODO: comments
        todo!()
    }

    fn hydrate_message(&mut self, hydrate: hydrate::Message) -> Result<message::Key, Error> {
        // TODO: remove destructor once all fields have been used
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
        } = hydrate.descriptor;
        let name = name.unwrap_or_default();
        let fqn = hydrate.fqn;
        let (key, msg) = self.messages.get_or_insert_by_fqn(fqn.clone());
        msg.hydrate_options(options.unwrap_or_default());
        msg.set_container(hydrate.container);
        msg.set_name(name);

        let mut messages = Vec::with_capacity(nested_type.len());
        for (i, nested) in nested_type.into_iter().enumerate() {
            let index = i as i32;
            messages.push(self.hydrate_message(hydrate::Message {
                index,
                fqn: FullyQualifiedName::new(nested.name(), Some(fqn.clone())),
                node_path: path::append(&hydrate.node_path, path::Message::Nested, index),
                container: key.into(),
                file: hydrate.file,
                descriptor: nested,
                nodes_by_fqn: hydrate.nodes_by_fqn,
                nodes_by_path: hydrate.nodes_by_path,
                package: hydrate.package,
                rel_pos: path::Message::Nested.as_i32(),
            })?);
        }
        let mut enums = Vec::with_capacity(enum_type.len());
        for (i, enm) in enum_type.into_iter().enumerate() {
            let index = i as i32;
            let fqn = FullyQualifiedName::new(enm.name(), Some(fqn.clone()));
            let node_path = path::append(&hydrate.node_path, path::Message::Enum, index);
            let key = self.hydrate_enum(hydrate::Enum {
                index,
                fqn: fqn.clone(),
                node_path: node_path.clone(),
                container: key.into(),
                file: hydrate.file,
                descriptor: enm,
                nodes_by_fqn: hydrate.nodes_by_fqn,
                nodes_by_path: hydrate.nodes_by_path,
                package: hydrate.package,
                rel_pos: path::Message::Enum.as_i32(),
            })?;
            enums.push(key);
            hydrate.nodes_by_fqn.insert(fqn, key.into());
            hydrate.nodes_by_path.insert(node_path, key.into());
        }
        for (i, descriptor) in oneof_decl.into_iter().enumerate() {
            let index = i as i32;
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
            let oneof_key = self.hydrate_oneof(hydrate::Oneof {
                index,
                fqn: fqn.clone(),
                node_path: path::append(&hydrate.node_path, path::Message::Oneof, index),
                file: hydrate.file,
                descriptor,
                message: key,
                package: hydrate.package,
            })?;
        }
        for (i, descriptor) in field.into_iter().enumerate() {
            let index = i as i32;
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
            let field_key = self.hydrate_field(hydrate::Field {
                descriptor,
                file: hydrate.file,
                fqn: fqn.clone(),
                message: key,
                package: hydrate.package,
                index,
                node_path: path::append(&hydrate.node_path, path::Message::Oneof, index),
            })?;
        }

        Ok(key)
    }

    fn hydrate_enum(&mut self, hydrate: hydrate::Enum) -> Result<r#enum::Key, Error> {
        todo!()
    }
    fn hydrate_service(&mut self, hydrate: hydrate::Service) -> Result<service::Key, Error> {
        todo!()
    }

    fn hydrate_extension(&mut self, hydrate: hydrate::Extension) -> Result<extension::Key, Error> {
        todo!()
    }

    fn hydrate_enum_value(&mut self, hydrate: hydrate::EnumValue) -> Result<r#enum::Key, Error> {
        todo!()
    }

    fn hydrate_field(&mut self, hydrate: hydrate::Field) -> Result<field::Key, Error> {
        todo!()
    }

    fn hydrate_oneof(
        &mut self,
        hydrate: hydrate::Oneof,
    ) -> Result<(oneof::Key, Box<dyn Iterator<Item = Key>>), Error> {
        todo!()
    }

    fn hydrate_method(&mut self, hydrate: hydrate::Message) -> Result<method::Key, Error> {
        todo!()
    }
}

macro_rules! impl_resolve {

    ($($col:ident => ($key:ident, $inner:ident),)+) => {
        $(
            impl Get<$key, $inner> for Ast {
                fn get(& self, key: $key) -> &$inner {
                    &self.$col[key]
                }
            }
            impl<'ast> Resolve<$inner> for Resolver<'ast, $key, $inner>
            {
                fn resolve(&self) -> &$inner {
                    self.ast.get(self.key.clone())
                }
            }
            impl<'ast> Deref for Resolver<'ast, $key, $inner>{
                type Target = $inner;
                fn deref(&self) -> &Self::Target {
                    self.resolve()
                }
            }
            impl<'ast> fmt::Debug for Resolver<'ast, $key, $inner>
            {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_tuple("Resolver")
                        .field(&self.key)
                        .field(self.resolve())
                        .finish()
                }
            }
        )+
    };
}

use r#enum::{Inner as EnumInner, Key as EnumKey};
use enum_value::{Inner as EnumValueInner, Key as EnumValueKey};
use extension::{
    GroupInner as ExtensionGroupInner, GroupKey as ExtensionGroupKey, Inner as ExtensionInner,
    Key as ExtensionKey,
};
use field::{Inner as FieldInner, Key as FieldKey};
use file::{Inner as FileInner, Key as FileKey};
use message::{Inner as MessageInner, Key as MessageKey};
use method::{Inner as MethodInner, Key as MethodKey};
use oneof::{Inner as OneofInner, Key as OneofKey};
use package::{Inner as PackageInner, Key as PackageKey};
use service::{Inner as ServiceInner, Key as ServiceKey};
impl_resolve!(
    packages => (PackageKey, PackageInner),
    files => (FileKey, FileInner),
    messages => (MessageKey, MessageInner),
    enums => (EnumKey, EnumInner),
    enum_values => (EnumValueKey, EnumValueInner),
    oneofs => (OneofKey, OneofInner),
    services => (ServiceKey, ServiceInner),
    methods => (MethodKey, MethodInner),
    fields => (FieldKey, FieldInner),
    extensions => (ExtensionKey, ExtensionInner),
    extension_groups => (ExtensionGroupKey, ExtensionGroupInner),
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Package,
    File,
    Message,
    Oneof,
    Enum,
    EnumValue,
    Service,
    Method,
    Field,
    Extension,
}

impl fmt::Display for Kind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Package => write!(fmt, "Package"),
            Self::File => write!(fmt, "File"),
            Self::Message => write!(fmt, "Message"),
            Self::Oneof => write!(fmt, "Oneof"),
            Self::Enum => write!(fmt, "Enum"),
            Self::EnumValue => write!(fmt, "EnumValue"),
            Self::Service => write!(fmt, "Service"),
            Self::Method => write!(fmt, "Method"),
            Self::Field => write!(fmt, "Field"),
            Self::Extension => write!(fmt, "Extension"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownType {
    Enum(WellKnownEnum),
    Message(WellKnownMessage),
}

impl WellKnownType {
    pub const PACKAGE: &'static str = "google.protobuf";
}

#[derive(Clone, PartialEq, Eq)]
pub enum Node<'ast> {
    Package(Package<'ast>),
    File(File<'ast>),
    Message(Message<'ast>),
    Oneof(Oneof<'ast>),
    Enum(Enum<'ast>),
    EnumValue(EnumValue<'ast>),
    Service(Service<'ast>),
    Method(Method<'ast>),
    Field(Field<'ast>),
    Extension(Extension<'ast>),
}

macro_rules! node_pass_through {
    ($method: ident) => {
        match self {
            Self::Package(n) => n.$method(),
            Self::File(n) => n.$method(),
            Self::Message(n) => n.$method(),
            Self::Oneof(n) => n.$method(),
            Self::Enum(n) => n.$method(),
            Self::EnumValue(n) => n.$method(),
            Self::Service(n) => n.$method(),
            Self::Method(n) => n.$method(),
            Self::Field(n) => n.$method(),
            Self::Extension(n) => n.$method(),
        }
    };
}

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Package(n) => n.fmt(f),
            Self::File(n) => n.fmt(f),
            Self::Message(n) => n.fmt(f),
            Self::Oneof(n) => n.fmt(f),
            Self::Enum(n) => n.fmt(f),
            Self::EnumValue(n) => n.fmt(f),
            Self::Service(n) => n.fmt(f),
            Self::Method(n) => n.fmt(f),
            Self::Field(n) => n.fmt(f),
            Self::Extension(n) => n.fmt(f),
        }
    }
}

impl Node<'_> {
    pub const fn kind(&self) -> Kind {
        match self {
            Self::Package(_) => Kind::Package,
            Self::File(_) => Kind::File,
            Self::Message(_) => Kind::Message,
            Self::Oneof(_) => Kind::Oneof,
            Self::Enum(_) => Kind::Enum,
            Self::EnumValue(_) => Kind::EnumValue,
            Self::Service(_) => Kind::Service,
            Self::Method(_) => Kind::Method,
            Self::Field(_) => Kind::Field,
            Self::Extension(_) => Kind::Extension,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FullyQualifiedName(String);

impl FullyQualifiedName {
    pub fn new(value: impl AsRef<str>, container: Option<Self>) -> Self {
        let value = value.as_ref();
        if value.is_empty() {
            if let Some(fqn) = container {
                return fqn;
            }
            return Self::default();
        }
        let container = container.unwrap_or_default();
        if container.is_empty() {
            return Self(value.into());
        }
        Self(format!("{container}.{value}"))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    fn push(&mut self, value: impl AsRef<str>) {
        let value = value.as_ref();
        if value.is_empty() {
            return;
        }
        if !self.0.is_empty() {
            self.0.push('.');
        }
        self.0.push_str(value);
    }
    fn from_package_name(package_name: impl AsRef<str>) -> Self {
        let package_name = package_name.as_ref();
        if package_name.is_empty() {
            return Self::default();
        }
        Self(format!(".{package_name}"))
    }
}
impl AsRef<str> for FullyQualifiedName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FullyQualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A message representing an option that parser does not recognize.
#[derive(Debug, Clone, PartialEq)]
pub struct UninterpretedOption {
    name: Vec<NamePart>,
    identifier_value: Option<String>,
    positive_int_value: Option<u64>,
    negative_int_value: Option<i64>,
    double_value: Option<f64>,
    string_value: Option<Vec<u8>>,
    aggregate_value: Option<String>,
}

impl From<protobuf::descriptor::UninterpretedOption> for UninterpretedOption {
    fn from(option: protobuf::descriptor::UninterpretedOption) -> Self {
        Self {
            name: option.name.into_iter().map(Into::into).collect::<Vec<_>>(),
            identifier_value: option.identifier_value,
            positive_int_value: option.positive_int_value,
            negative_int_value: option.negative_int_value,
            double_value: option.double_value,
            string_value: option.string_value,
            aggregate_value: option.aggregate_value,
        }
    }
}

impl UninterpretedOption {
    #[must_use]
    pub fn name(&self) -> &[NamePart] {
        self.name.as_ref()
    }

    #[must_use]
    pub const fn identifier_value(&self) -> Option<&String> {
        self.identifier_value.as_ref()
    }

    #[must_use]
    pub const fn negative_int_value(&self) -> Option<i64> {
        self.negative_int_value
    }

    #[must_use]
    pub const fn double_value(&self) -> Option<f64> {
        self.double_value
    }

    #[must_use]
    pub fn string_value(&self) -> Option<&[u8]> {
        self.string_value.as_deref()
    }

    #[must_use]
    pub fn aggregate_value(&self) -> Option<&str> {
        self.aggregate_value.as_deref()
    }
}

//  The name of the uninterpreted option.  Each string represents a segment in
///  a dot-separated name.
///
///  E.g.,`{ ["foo", false], ["bar.baz", true], ["qux", false] }` represents
///  `"foo.(bar.baz).qux"`.
#[derive(PartialEq, Eq, Hash, Clone, Default, Debug)]
pub struct NamePart {
    pub value: String,
    pub is_extension: bool,
}

impl NamePart {
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
    /// true if a segment represents an extension (denoted with parentheses in
    ///  options specs in .proto files).
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        self.is_extension
    }

    /// Returns the formatted value of the `NamePart`
    ///
    /// If `is_extension` is `true`, the formatted value will be wrapped in
    /// parentheses.
    #[must_use]
    pub fn formatted_value(&self) -> Cow<'_, str> {
        if self.is_extension {
            Cow::Owned(format!("({})", self.value()))
        } else {
            Cow::Borrowed(self.value())
        }
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for NamePart {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NamePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_extension {
            write!(f, "({})", self.value())
        } else {
            write!(f, "{}", self.value())
        }
    }
}

impl From<protobuf::descriptor::uninterpreted_option::NamePart> for NamePart {
    fn from(part: protobuf::descriptor::uninterpreted_option::NamePart) -> Self {
        Self {
            is_extension: part.is_extension.unwrap_or(false),
            value: part.name_part.unwrap_or_default(),
        }
    }
}

impl From<&protobuf::descriptor::uninterpreted_option::NamePart> for NamePart {
    fn from(part: &protobuf::descriptor::uninterpreted_option::NamePart) -> Self {
        Self::from(part.clone())
    }
}

#[derive(Debug, Clone)]
pub struct NameParts {
    parts: Vec<NamePart>,
}

impl std::fmt::Display for NameParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl<'a> std::iter::IntoIterator for &'a NameParts {
    type Item = &'a NamePart;
    type IntoIter = std::slice::Iter<'a, NamePart>;
    fn into_iter(self) -> Self::IntoIter {
        self.parts.iter()
    }
}

impl NameParts {
    pub fn iter(&self) -> std::slice::Iter<'_, NamePart> {
        self.parts.iter()
    }
    #[must_use]
    pub fn get(&self, idx: usize) -> Option<&NamePart> {
        self.parts.get(idx)
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.parts.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
    #[must_use]
    pub fn contains(&self, part: &str) -> bool {
        self.parts.iter().any(|p| p.value == part)
    }
    #[must_use]
    pub fn formatted(&self) -> String {
        itertools::join(self.iter().map(|v| v.formatted_value()), ".")
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

impl From<protobuf::descriptor::descriptor_proto::ReservedRange> for ReservedRange {
    fn from(range: protobuf::descriptor::descriptor_proto::ReservedRange) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl ReservedRange {
    #[must_use]
    pub fn start(&self) -> i32 {
        self.start.unwrap_or(0)
    }
    #[must_use]
    pub fn end(&self) -> i32 {
        self.end.unwrap_or(0)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Comments {
    /// Any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    leading: Option<String>,
    trailing: Option<String>,
    leading_detached: Vec<String>,
}

impl Comments {
    pub fn new_maybe(
        leading: Option<String>,
        trailing: Option<String>,
        leading_detacted: Vec<String>,
    ) -> Option<Self> {
        if leading.is_none() && trailing.is_none() && leading_detacted.is_empty() {
            return None;
        }
        Some(Self {
            leading,
            trailing,
            leading_detached: leading_detacted,
        })
    }
    /// Any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub fn leading(&self) -> Option<&str> {
        self.leading.as_deref()
    }

    /// Any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing(&self) -> Option<&str> {
        self.trailing.as_deref()
    }

    /// Each comment block or line above the entity but seperated by whitespace.
    pub fn leading_detached(&self) -> &[String] {
        &self.leading_detached
    }
}

macro_rules! impl_key {
    ($inner:ident, $key:ident) => {
        impl crate::ast::access::Key for $inner {
            type Key = $key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl $inner {
            pub(super) fn set_key(&mut self, key: $key) {
                self.key = key;
            }
        }
    };
}

// macro_rules! impl_fsm {
//     ($inner:ident) => {
//         impl crate::ast::Fsm for $inner {}
//     };
// }
macro_rules! impl_access {
    ($node: ident, $key: ident, $inner: ident) => {
        impl<'ast> crate::ast::Resolve<$inner> for $node<'ast> {
            fn resolve(&self) -> &$inner {
                crate::ast::Resolve::resolve(&self.0)
            }
        }
    };
}
macro_rules! impl_clone_copy {
    ($node:ident) => {
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for $node<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<'ast> Copy for $node<'ast> {}
    };
}
macro_rules! impl_access_fqn {
    ($node:ident, $key:ident, $inner: ident) => {
        impl<'ast> crate::ast::access::FullyQualifiedName for $node<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> $node<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for $inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
    };
}

macro_rules! impl_from_fqn {
    ($inner:ident) => {
        impl From<crate::ast::FullyQualifiedName> for $inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for $inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
    };
}
macro_rules! impl_eq {
    ($node:ident) => {
        impl<'ast> PartialEq for $node<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for $node<'ast> {}
    };
    () => {};
}

macro_rules! impl_from_key_and_ast {
    ($node:ident, $key:ident, $inner:ident) => {
        impl<'ast> From<($key, &'ast crate::ast::Ast)> for $node<'ast> {
            fn from((key, ast): ($key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::Resolver<'ast, $key, $inner>> for $node<'ast> {
            fn from(resolver: crate::ast::Resolver<'ast, $key, $inner>) -> Self {
                Self(resolver)
            }
        }
    };
}

macro_rules! impl_fmt {
    ($node: ident, $key: ident, $inner: ident) => {
        impl<'ast> ::std::fmt::Display for $node<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }

        impl<'ast> ::std::fmt::Debug for $node<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
    };
}

macro_rules! impl_access_reserved {
    ($node: ident, $inner:ident) => {
        impl $inner {
            fn set_reserved(
                &mut self,
                names: Vec<String>,
                ranges: Vec<protobuf::descriptor::descriptor_proto::ReservedRange>,
            ) {
                self.reserved_names = names;
                self.reserved_ranges = ranges.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> $node<'ast> {
            pub fn reserved_names(&self) -> &[String] {
                &self.0.reserved_names
            }
            pub fn reserved_ranges(&self) -> &[crate::ast::ReservedRange] {
                &self.0.reserved_ranges
            }
        }
        impl<'ast> crate::ast::access::Reserved for $node<'ast> {
            fn reserved_names(&self) -> &[String] {
                &self.0.reserved_names
            }
            fn reserved_ranges(&self) -> &[crate::ast::ReservedRange] {
                &self.0.reserved_ranges
            }
        }
    };
}

macro_rules! impl_access_file {
    ($node:ident, $inner: ident) => {
        impl<'ast> crate::ast::access::File<'ast> for $node<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> $node<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
    };
}
macro_rules! impl_access_package {
    ($node: ident, $inner: ident) => {
        impl<'ast> crate::ast::access::Package<'ast> for $node<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }

        impl<'ast> $node<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
    };
}
macro_rules! set_unknown_fields {
    ($inner:ident) => {
        pub(super) fn set_unknown_fields<T>(&mut self, fields: impl IntoIterator<Item = T>)
        where
            T: Into<crate::ast::UnknownFields>,
        {
            self.unknown_fields = fields.into_iter().map(Into::into);
        }
    };
}
macro_rules! impl_set_uninterpreted_options {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn set_uninterpreted_options<T>(&mut self, opts: impl IntoIterator<Item = T>)
            where
                T: Into<crate::ast::UninterpretedOption>,
            {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
    };
}
macro_rules! impl_access_name {
    ($node:ident, $inner: ident) => {
        impl $inner {
            pub(super) fn set_name(&mut self, name: impl Into<String>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for $node<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> $node<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
    };
}

macro_rules! inner_method_package {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
    };
}
macro_rules! inner_method_file {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
    };
}

macro_rules! node_method_ast {
    ($node:ident) => {
        impl<'ast> $node<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
    };
}
macro_rules! node_method_new {
    ($node:ident, $key:ident) => {
        impl<'ast> $node<'ast> {
            pub(super) fn new(key: $key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
    };
}
macro_rules! node_method_key {
    ($node:ident, $key: ident) => {
        impl<'ast> $node<'ast> {
            pub(crate) fn key(self) -> $key {
                self.0.key
            }
        }
    };
}

macro_rules! impl_span {
    ($node:ident, $inner:ident) => {
        impl<'ast> crate::ast::access::Span for $node<'ast> {
            fn span(&self) -> crate::ast::Span {
                self.0.span
            }
        }
        impl<'ast> $node<'ast> {
            pub fn span(&self) -> crate::ast::Span {
                self.0.span
            }
        }

        impl $inner {
            pub(super) fn set_span(&mut self, span: crate::ast::Span) {
                self.span = span;
            }
        }
    };
}

macro_rules! impl_comments {
    ($node:ident, $inner:ident) => {
        impl<'ast> crate::ast::access::Comments for $node<'ast> {
            fn comments(&self) -> Option<&crate::ast::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> $node<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::Comments> {
                self.0.comments.as_ref()
            }
        }

        impl $inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::Comments) {
                self.comments = Some(comments);
            }
        }
    };
}

macro_rules! impl_node_path {
    ($node:ident, $inner:ident) => {
        impl<'ast> crate::ast::access::NodePath for $node<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> $node<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl $inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
    };
}

macro_rules! impl_base_traits_and_methods {
    ($node:ident, $key:ident, $inner:ident) => {
        crate::ast::impl_key!($inner, $key);
        crate::ast::node_method_new!($node, $key);
        crate::ast::node_method_key!($node, $key);
        crate::ast::node_method_ast!($node);
        crate::ast::impl_clone_copy!($node);
        crate::ast::impl_eq!($node);
        crate::ast::impl_access!($node, $key, $inner);
        crate::ast::impl_access_fqn!($node, $key, $inner);
        crate::ast::impl_from_key_and_ast!($node, $key, $inner);
        crate::ast::impl_fmt!($node, $key, $inner);
        crate::ast::impl_from_fqn!($inner);
        crate::ast::impl_access_name!($node, $inner);
        // crate::ast::impl_state!($inner);
        // crate::ast::impl_fsm!($inner);
    };
}
macro_rules! impl_traits_and_methods {
    (Package, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Package, $key, $inner);
    };

    (File, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(File, $key, $inner);
        crate::ast::impl_access_package!(File, $inner);
        crate::ast::impl_comments!(File, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::inner_method_package!($inner);
    };

    (Message, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Message, $key, $inner);
        crate::ast::impl_access_reserved!(Message, $inner);
        crate::ast::impl_access_file!(Message, $inner);
        crate::ast::impl_access_package!(Message, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::impl_node_path!(Message, $inner);
        crate::ast::impl_span!(Message, $inner);
        crate::ast::impl_comments!(Message, $inner);
        crate::ast::inner_method_file!($inner);
        crate::ast::inner_method_package!($inner);
    };

    (Enum, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Enum, $key, $inner);
        crate::ast::impl_access_reserved!(Enum, $inner);
        crate::ast::impl_access_file!(Enum, $inner);
        crate::ast::impl_access_package!(Enum, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::impl_node_path!(Enum, $inner);
        crate::ast::impl_span!(Enum, $inner);
        crate::ast::impl_comments!(Enum, $inner);
        crate::ast::inner_method_file!($inner);
        crate::ast::inner_method_package!($inner);
    };

    ($node: ident, $key: ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!($node, $key, $inner);
        crate::ast::impl_access_file!($node, $inner);
        crate::ast::impl_access_package!($node, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::impl_node_path!($node, $inner);
        crate::ast::impl_span!($node, $inner);
        crate::ast::impl_comments!($node, $inner);
        crate::ast::inner_method_file!($inner);
        crate::ast::inner_method_package!($inner);
    };
}

use impl_access;
use impl_access_file;
use impl_access_fqn;
use impl_access_name;
use impl_access_package;
use impl_access_reserved;
use impl_base_traits_and_methods;
use impl_clone_copy;
use impl_comments;
use impl_eq;
use impl_fmt;
use impl_from_fqn;
use impl_from_key_and_ast;
use impl_key;
use impl_node_path;
use impl_set_uninterpreted_options;
use impl_span;
use impl_traits_and_methods;
use inner_method_file;
use inner_method_package;
use node_method_ast;
use node_method_key;
use node_method_new;
// use impl_state;

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_fully_qualified_name() {
        // let fqn = FullyQualifiedName::new("foo", None);
        // assert_eq!(fqn.as_str(), ".foo");

        // let fqn = FullyQualifiedName::new("foo",
        // Some(FullyQualifiedName("bar".into())); assert_eq!(fqn.
        // as_str(), "bar.foo");

        // let fqn = FullyQualifiedName::new("foo", Some(".bar"));
        // assert_eq!(fqn.as_str(), ".bar.foo");
    }
}
