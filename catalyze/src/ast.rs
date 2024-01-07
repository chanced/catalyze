use std::{borrow::Cow, fmt, ops::Deref};

use slotmap::SlotMap;

use crate::{
    ast,
    r#enum::{self, Enum, WellKnownEnum},
    enum_value::{self, EnumValue},
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

mod hydrate;

#[doc(hidden)]
pub trait Get<K, T> {
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

#[derive(Debug)]
pub struct Ast {
    packages: SlotMap<package::Key, package::Inner>,
    files: SlotMap<file::Key, file::Inner>,
    messages: SlotMap<message::Key, message::Inner>,
    enums: SlotMap<r#enum::Key, r#enum::Inner>,
    enum_values: SlotMap<enum_value::Key, enum_value::Inner>,
    services: SlotMap<service::Key, service::Inner>,
    methods: SlotMap<method::Key, method::Inner>,
    fields: SlotMap<field::Key, field::Inner>,
    oneofs: SlotMap<oneof::Key, oneof::Inner>,
    extensions: SlotMap<extension::Key, extension::Inner>,
    // defined_extensions: Vec<Extension>,
    nodes: HashMap<FullyQualifiedName, Key>,
}

pub(crate) struct Accessor<'ast, K, I, A> {
    ast: &'ast A,
    key: K,
    marker: std::marker::PhantomData<I>,
}

impl<'ast, K, I, A> Accessor<'ast, K, I, A> {
    pub(crate) fn new(key: K, ast: &'ast A) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I, A> Clone for Accessor<'ast, K, I, A>
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

impl<'ast, K, I, A> Deref for Accessor<'ast, K, I, A>
where
    A: Get<K, I>,
    K: Copy,
{
    type Target = I;
    fn deref(&self) -> &Self::Target {
        self.access()
    }
}

impl<'ast, K, I, A> fmt::Display for Accessor<'ast, K, I, A>
where
    A: Get<K, I>,
    I: fmt::Display,
    K: Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.access().fmt(f)
    }
}

impl<'ast, K, I, A> fmt::Debug for Accessor<'ast, K, I, A>
where
    A: Get<K, I>,
    I: fmt::Debug,
    K: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Accessor")
            .field(&self.key)
            .field(self.access())
            .finish()
    }
}

impl<'ast, K, I, A> From<(K, &'ast A)> for Accessor<'ast, K, I, A> {
    fn from((key, ast): (K, &'ast A)) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I, A> Copy for Accessor<'ast, K, I, A> where K: Copy {}

impl<'ast, K, I, A> PartialEq for Accessor<'ast, K, I, A>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<'ast, K, I, A> Eq for Accessor<'ast, K, I, A> where K: Eq {}

impl<'ast, K, I, A> Access<I> for Accessor<'ast, K, I, A>
where
    A: Get<K, I>,
    K: Copy,
{
    fn access(&self) -> &I {
        self.ast.get(self.key)
    }
}

macro_rules! impl_get {
    ($($col: ident -> $mod: ident,)+) => {
        $(
            impl Get<$mod::Key, $mod::Inner> for Ast {
                fn get(& self, key: $mod::Key) -> &$mod::Inner {
                    &self.$col[key]
                }
            }
        )+
    };
}

impl_get!(
    packages -> package,
    files -> file,
    messages -> message,
    enums -> r#enum,
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Nodes<K> {
    fqn_lookup: HashMap<FullyQualifiedName, usize>,
    name_lookup: HashMap<String, usize>,
    list: Vec<K>,
}
impl Default for Nodes<ast::Key> {
    fn default() -> Self {
        Self {
            fqn_lookup: HashMap::default(),
            list: Vec::new(),
            name_lookup: HashMap::default(),
        }
    }
}
impl<T> Nodes<T>
where
    T: Fqn,
{
    pub fn insert(&mut self, node: T) {
        if self.fqn_lookup.contains_key(node.fully_qualified_name()) {
            return;
        }
        self.fqn_lookup.insert(node.fqn().clone(), self.list.len());
        self.list.push(node);
    }
}
impl<T> Nodes<T>
where
    T: Clone,
{
    pub fn get(&self, fqn: &FullyQualifiedName) -> Option<T> {
        self.fqn_lookup.get(fqn).map(|i| self.list[*i].clone())
    }
}
impl<T> Nodes<T> {
    pub fn new() -> Self {
        Self {
            fqn_lookup: HashMap::default(),
            list: Vec::new(),
            name_lookup: HashMap::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.list.iter()
    }
}

impl Deref for Nodes<ast::Key> {
    type Target = [ast::Key];
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl<'ast> AsRef<[Node<'ast>]> for Nodes<Node<'ast>> {
    fn as_ref(&self) -> &[Node<'ast>] {
        &self.list
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
    pub fn new(value: impl AsRef<str>, container: Option<FullyQualifiedName>) -> Self {
        let value = value.as_ref();
        if value.is_empty() {
            if let Some(fqn) = container {
                return fqn;
            }
            return Self::default();
        }
        Self(format!("{}.{}", container.unwrap_or_default(), &value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub(crate) fn push(&mut self, value: impl AsRef<str>) {
        let value = value.as_ref();
        if value.is_empty() {
            return;
        }
        self.0.push('.');
        self.0.push_str(value);
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
    ($typ: ident, $key: ident,$inner: ident) => {
        impl<'ast, A> crate::ast::Access<$inner> for $typ<'ast, A>
        where
            A: crate::ast::Get<$key, $inner>,
        {
            fn access(&self) -> &$inner {
                self.0.access()
            }
        }
    };
}
macro_rules! impl_copy_clone {
    ($typ:ident) => {
        impl<'ast, A> Clone for $typ<'ast, A> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast, A> Copy for $typ<'ast, A> {}
    };
}
macro_rules! impl_fqn {
    ($typ:ident, $key:ident, $inner: ident) => {
        #[inherent::inherent]
        impl<'ast, A> crate::ast::Fqn for $typ<'ast, A>
        where
            A: crate::ast::Get<$key, $inner>,
        {
            #[doc = "Returns the [`FullyQualifiedName`] of the Message."]
            pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.0.fqn
            }
            /// Alias for `fully_qualified_name` - returns the [`FullyQualifiedName`] of
            /// the Package.
            pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
    };
}

macro_rules! impl_eq {
    ($typ:ident) => {
        impl<'ast, A> PartialEq for $typ<'ast, A> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast, A> Eq for $typ<'ast, A> {}
    };
    () => {};
}

macro_rules! impl_from_key_and_ast {
    ($typ:ident, $key:ident, $inner:ident) => {
        impl<'ast, A> From<($key, &'ast A)> for $typ<'ast, A>
        where
            A: crate::ast::Get<$key, $inner>,
        {
            fn from((key, ast): ($key, &'ast A)) -> Self {
                Self(crate::ast::Accessor::new(key, ast))
            }
        }
        impl<'ast, A> From<crate::ast::Accessor<'ast, $key, $inner, A>> for $typ<'ast, A> {
            fn from(accessor: crate::ast::Accessor<'ast, $key, $inner, A>) -> Self {
                Self(accessor)
            }
        }
    };
}
macro_rules! impl_fmt {
    ($typ: ident, $key: ident, $inner: ident) => {
        impl<'ast, A> ::std::fmt::Display for $typ<'ast, A>
        where
            A: crate::ast::Get<$key, $inner>,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::Access;
                ::std::fmt::Display::fmt(&self.access().fqn, f)
            }
        }

        impl<'ast, A> ::std::fmt::Debug for $typ<'ast, A>
        where
            A: crate::ast::Get<$key, $inner>,
        {
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
        // crate::ast::impl_fqn!($typ, $key, $inner);
        crate::ast::impl_from_key_and_ast!($typ, $key, $inner);
        crate::ast::impl_fmt!($typ, $key, $inner);
    };
}

pub(crate) use impl_access;
pub(crate) use impl_copy_clone;
pub(crate) use impl_eq;
pub(crate) use impl_fmt;
pub(crate) use impl_fqn;
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
