use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, Index, IndexMut},
    path::PathBuf,
};

use ahash::HashMapExt;
use slotmap::SlotMap;

use crate::{
    r#enum::{self, Enum, WellKnownEnum},
    enum_value::{self, EnumValue},
    error::Error,
    extension::{self, Extension},
    field::{self, Field},
    file::{self, File},
    message::{self, Message, WellKnownMessage},
    method::{self, Method},
    oneof::{self, Oneof},
    package::{self, Package},
    service::{self, Service},
    HashMap,
};

pub(crate) mod node_path {
    pub(crate) const PACKAGE: i32 = 2;
    pub(crate) const MESSAGE_TYPE: i32 = 4;
    pub(crate) const ENUM_TYPE: i32 = 5;
    pub(crate) const SERVICE: i32 = 6;
    pub(crate) const SYNTAX: i32 = 12;
    pub(crate) const MESSAGE_TYPE_FIELD: i32 = 2;
    pub(crate) const MESSAGE_TYPE_NESTED_TYPE: i32 = 3;
    pub(crate) const MESSAGE_TYPE_ENUM_TYPE: i32 = 4;
    pub(crate) const MESSAGE_TYPE_ONEOF_DECL: i32 = 8;
    pub(crate) const ENUM_TYPE_VALUE: i32 = 2;
    pub(crate) const SERVICE_TYPE_METHOD: i32 = 2;
}
#[doc(hidden)]
pub(crate) trait Get<K, T> {
    fn get(&self, key: K) -> &T;
}

pub(crate) trait Access<T> {
    fn access(&self) -> &T;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key {
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
pub(crate) enum ContainerKey {
    Message(message::Key),
    File(file::Key),
}

impl ContainerKey {
    /// Returns `true` if the container key is [`Message`].
    ///
    /// [`Message`]: ContainerKey::Message
    #[must_use]
    pub(crate) fn is_message(&self) -> bool {
        matches!(self, Self::Message(..))
    }

    #[must_use]
    pub(crate) fn as_message(self) -> Option<message::Key> {
        if let Self::Message(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn try_into_message(self) -> Result<message::Key, Self> {
        if let Self::Message(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the container key is [`File`].
    ///
    /// [`File`]: ContainerKey::File
    #[must_use]
    pub(crate) fn is_file(self) -> bool {
        matches!(self, Self::File(..))
    }

    #[must_use]
    pub(crate) fn as_file(self) -> Option<file::Key> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn try_into_file(self) -> Result<file::Key, Self> {
        if let Self::File(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
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
    pub fn is_message(self) -> bool {
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

    #[must_use]
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

    #[must_use]
    pub fn try_into_file(self) -> Result<File<'ast>, Self> {
        if let Self::File(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Table<K, V>
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
{
    pub(crate) fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.order.iter().map(move |key| (*key, &self.map[*key]))
    }
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.map.iter_mut()
    }
    pub(crate) fn keys<'ast>(&'ast self) -> impl 'ast + Iterator<Item = K> {
        self.order.iter().copied()
    }
    pub(crate) fn get_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&V> {
        self.lookup.get(fqn).map(|key| &self.map[*key])
    }
    pub(crate) fn get_mut_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&mut V> {
        self.lookup.get(fqn).map(|key| &mut self.map[*key])
    }

    pub(crate) fn get(&self, key: K) -> &V {
        &self.map[key]
    }
    pub(crate) fn get_mut(&mut self, key: K) -> &mut V {
        &mut self.map[key]
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
    V: From<FullyQualifiedName>,
{
    pub fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::new(),
            order: Vec::new(),
        }
    }
    pub fn get_or_insert_mut_by_fqn(&mut self, fqn: FullyQualifiedName) -> (K, &mut V) {
        let key = self
            .lookup
            .entry(fqn.clone())
            .or_insert_with(|| self.map.insert(fqn.into()))
            .clone();
        (key, &mut self.map[key])
    }
    pub fn get_or_insert_by_fqn(&mut self, fqn: FullyQualifiedName) -> (K, &V) {
        let key = self
            .lookup
            .entry(fqn.clone())
            .or_insert_with(|| self.map.insert(fqn.into()))
            .clone();
        (key, &self.map[key])
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
    nodes: HashMap<FullyQualifiedName, Key>,
    pkg_lookup: HashMap<String, package::Key>,
}

impl Ast {
    pub(crate) fn new(
        input: &[protobuf::descriptor::FileDescriptorProto],
        targets: &[String],
    ) -> Result<Self, Error> {
        let targets = targets.iter().map(PathBuf::from).collect::<Vec<_>>();
        let mut this = Self::default();
        for fd in input {
            this.hydrate_file(fd, &targets)?;
        }
        Ok(this)
    }

    fn hydrate_file(
        &mut self,
        descriptor: &protobuf::descriptor::FileDescriptorProto,
        targets: &[PathBuf],
    ) -> Result<file::Key, Error> {
        let pkg = descriptor.package.map(|pkg| {
            self.packages
                .get_or_insert_by_fqn(FullyQualifiedName::from_package_name(pkg))
        });

        let fqn = FullyQualifiedName::new(descriptor.name(), pkg.as_ref().map(|(_, pkg)| pkg.fqn));
        let is_build_target = targets
            .iter()
            .any(|target| target.as_os_str() == descriptor.name());
        let (key, file) = self.files.get_or_insert_mut_by_fqn(fqn.clone());

        if file.package.is_none() {
            pkg.map(|(pkg_key, pkg)| {
                file.package = Some(pkg_key);
                pkg.files.push(key);
            });
        }

        file.hydrate_data(descriptor, is_build_target)?;

        if !file.is_fully_hydrated() {
            return Ok(());
        }
        for msg in &descriptor.message_type {
            self.hydrate_message(
                FullyQualifiedName::new(msg.name(), Some(fqn)),
                msg,
                key.into(),
            )?;
        }
        for enm in &descriptor.enum_type {
            self.hydrate_enum(
                FullyQualifiedName::new(enm.name(), Some(fqn)),
                enm,
                key.into(),
            )?;
        }
        for service in &descriptor.service {
            self.hydrate_service(
                FullyQualifiedName::new(service.name(), Some(fqn)),
                service,
                key,
            )?;
        }
        for ext in &descriptor.extension {
            self.hydrate_extension(
                FullyQualifiedName::new(ext.name(), Some(fqn)),
                ext,
                key.into(),
            )?;
        }
        // TODO: comments
        todo!()
    }

    fn hydrate_message(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::DescriptorProto,
        container_key: ContainerKey,
    ) -> Result<message::Key, Error> {
        let (key, msg) = self.messages.get_or_insert_mut_by_fqn(fqn.clone());
        msg.hydrate_data(descriptor, container_key)?;
        for nested in &descriptor.nested_type {
            self.hydrate_message(
                FullyQualifiedName::new(nested.name(), Some(fqn.clone())),
                nested,
                key.into(),
            )?;
        }
        for enm in &descriptor.enum_type {
            self.hydrate_enum(
                FullyQualifiedName::new(enm.name(), Some(fqn.clone())),
                enm,
                key.into(),
            )?;
        }
        for oneof in &descriptor.oneof_decl {
            self.hydrate_oneof(
                FullyQualifiedName::new(oneof.name(), Some(fqn.clone())),
                oneof,
                key.into(),
            )?;
        }
        for field in &descriptor.field {
            self.hydrate_field(
                FullyQualifiedName::new(field.name(), Some(fqn.clone())),
                field,
                key.into(),
            )?;
        }
        todo!()
    }

    fn hydrate_enum(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::EnumDescriptorProto,
        file_key: ContainerKey,
    ) -> Result<r#enum::Key, Error> {
        todo!()
    }

    fn hydrate_extension(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::FieldDescriptorProto,
        container_key: ContainerKey,
    ) -> Result<extension::Key, Error> {
        todo!()
    }

    fn hydrate_service(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::ServiceDescriptorProto,
        file_key: file::Key,
    ) -> Result<(), Error> {
        todo!()
    }

    fn hydrate_enum_value(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::EnumValueDescriptorProto,
        enum_key: r#enum::Key,
    ) -> Result<(), Error> {
        todo!()
    }

    fn hydrate_field(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::FieldDescriptorProto,
        msg_key: message::Key,
    ) -> Result<(), Error> {
        todo!()
    }

    fn hydrate_oneof(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::OneofDescriptorProto,
        message_key: message::Key,
    ) -> Result<(), Error> {
        todo!()
    }

    fn hydrate_method(
        &self,
        fqn: FullyQualifiedName,
        descriptor: &protobuf::descriptor::MethodDescriptorProto,
        service_key: service::Key,
    ) -> Result<(), Error> {
        todo!()
    }
}

pub(crate) struct Accessor<'ast, K, I> {
    pub(crate) ast: &'ast Ast,
    pub(crate) key: K,
    marker: std::marker::PhantomData<I>,
}

impl<'ast, K, I> Accessor<'ast, K, I> {
    pub(crate) const fn new(key: K, ast: &'ast Ast) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> Clone for Accessor<'ast, K, I>
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

impl<'ast, K, I> From<(K, &'ast Ast)> for Accessor<'ast, K, I> {
    fn from((key, ast): (K, &'ast Ast)) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> Copy for Accessor<'ast, K, I> where K: Copy {}

impl<'ast, K, I> PartialEq for Accessor<'ast, K, I>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<'ast, K, I> Eq for Accessor<'ast, K, I> where K: Eq {}

macro_rules! impl_get_access {

    ($($col: ident -> $mod: ident,)+) => {
        $(
            impl Get<$mod::Key, $mod::Inner> for Ast {
                fn get(& self, key: $mod::Key) -> &$mod::Inner {
                    &self.$col[key]
                }
            }
            impl<'ast> Access<$mod::Inner> for Accessor<'ast, $mod::Key, $mod::Inner>
            {
                fn access(&self) -> &$mod::Inner {
                    self.ast.get(self.key.clone())
                }
            }
            impl<'ast> Deref for Accessor<'ast, $mod::Key, $mod::Inner>{
                type Target = $mod::Inner;
                fn deref(&self) -> &Self::Target {
                    self.access()
                }
            }
            impl<'ast> fmt::Debug for Accessor<'ast, $mod::Key, $mod::Inner>
            {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_tuple("Accessor")
                        .field(&self.key)
                        .field(self.access())
                        .finish()
                }
            }



        )+
    };
}

impl_get_access!(
    packages -> package,
    files -> file,
    messages -> message,
    enums -> r#enum,
    enum_values -> enum_value,
    oneofs -> oneof,
    services -> service,
    methods -> method,
    fields -> field,
    extensions -> extension,
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

impl fmt::Debug for Node<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
        // match self {
        //     Self::Package(p) => p.fmt(fmt),
        //     Self::File(f) => f.fmt(fmt),
        //     Self::Message(m) => m.fmt(fmt),
        //     Self::Oneof(o) => o.fmt(fmt),
        //     Self::Enum(e) => e.fmt(fmt),
        //     Self::EnumValue(e) => e.fmt(fmt),
        //     Self::Service(s) => s.fmt(fmt),
        //     Self::Method(m) => m.fmt(fmt),
        //     Self::Field(f) => f.fmt(fmt),
        //     Self::Extension(e) => e.fmt(fmt),
        // }
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

/// A trait implemented by all nodes that have a [`FullyQualifiedName`].
pub trait Fqn {
    /// Returns the [`FullyQualifiedName`] of the node.
    fn fully_qualified_name(&self) -> &FullyQualifiedName;

    /// Alias for `fully_qualified_name` - returns the [`FullyQualifiedName`] of
    /// the node.
    fn fqn(&self) -> &FullyQualifiedName {
        self.fully_qualified_name()
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
    pub(crate) fn push(&mut self, value: impl AsRef<str>) {
        let value = value.as_ref();
        if value.is_empty() {
            return;
        }
        if !self.0.is_empty() {
            self.0.push('.');
        }
        self.0.push_str(value);
    }
    pub(crate) fn from_package_name(package_name: impl AsRef<str>) -> Self {
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
            name: option
                .name
                .into_iter()
                .map(|part| part.into())
                .collect::<Vec<_>>(),
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
    value: String,
    is_extension: bool,
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

macro_rules! impl_access {
    ($typ: ident, $key: ident, $inner: ident) => {
        impl<'ast> crate::ast::Access<$inner> for $typ<'ast> {
            fn access(&self) -> &$inner {
                crate::ast::Access::access(&self.0)
            }
        }
    };
}
macro_rules! impl_copy_clone {
    ($typ:ident) => {
        impl<'ast> Clone for $typ<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for $typ<'ast> {}
    };
}
macro_rules! impl_fqn {
    ($typ:ident, $key:ident, $inner: ident) => {
        #[inherent::inherent]
        impl<'ast> crate::ast::Fqn for $typ<'ast> {
            #[doc = "Returns the [`FullyQualifiedName`] of the Message."]
            pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::Access;
                &self.access().fqn
            }
            #[doc = "Alias for `fully_qualified_name` - returns the [`FullyQualifiedName`] of the Package."]
            pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
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
    };
}
macro_rules! impl_eq {
    ($typ:ident) => {
        impl<'ast> PartialEq for $typ<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for $typ<'ast> {}
    };
    () => {};
}

macro_rules! impl_from_key_and_ast {
    ($typ:ident, $key:ident, $inner:ident) => {
        impl<'ast> From<($key, &'ast Ast)> for $typ<'ast> {
            fn from((key, ast): ($key, &'ast Ast)) -> Self {
                Self(crate::ast::Accessor::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::Accessor<'ast, $key, $inner>> for $typ<'ast> {
            fn from(accessor: crate::ast::Accessor<'ast, $key, $inner>) -> Self {
                Self(accessor)
            }
        }
    };
}
macro_rules! impl_fmt {
    ($typ: ident, $key: ident, $inner: ident) => {
        impl<'ast> ::std::fmt::Display for $typ<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::Access;
                ::std::fmt::Display::fmt(&self.access().fqn, f)
            }
        }

        impl<'ast> ::std::fmt::Debug for $typ<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::Access;
                ::std::fmt::Debug::fmt(self.access(), f)
            }
        }
    };
}

macro_rules! impl_traits {
    ($typ: ident, $key: ident, $inner: ident) => {
        crate::ast::impl_copy_clone!($typ);
        crate::ast::impl_eq!($typ);
        crate::ast::impl_access!($typ, $key, $inner);
        crate::ast::impl_fqn!($typ, $key, $inner);
        crate::ast::impl_from_key_and_ast!($typ, $key, $inner);
        crate::ast::impl_fmt!($typ, $key, $inner);
        crate::ast::impl_from_fqn!($inner);
    };
}

pub(crate) use impl_access;
pub(crate) use impl_copy_clone;
pub(crate) use impl_eq;
pub(crate) use impl_fmt;
pub(crate) use impl_fqn;
pub(crate) use impl_from_fqn;
pub(crate) use impl_from_key_and_ast;
pub(crate) use impl_traits;

#[cfg(test)]
mod tests {
    use super::*;

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
