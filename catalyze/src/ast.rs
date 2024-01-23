pub mod access;
pub mod container;
pub mod r#enum;
pub mod enum_value;
pub mod extension;
pub mod extension_decl;
pub mod field;
pub mod file;
pub mod location;
pub mod message;
pub mod method;
pub mod node;
pub mod oneof;
pub mod package;
pub mod path;
pub mod reference;
pub mod reserved;
pub mod service;
pub mod uninterpreted;

mod hydrate;
mod resolve;

use crate::HashMap;

use protobuf::descriptor::FileDescriptorProto;
use std::{
    fmt,
    ops::{Deref, Index, IndexMut},
};

use slotmap::SlotMap;

use crate::error::Error;

trait FromFqn {
    fn from_fqn(fqn: FullyQualifiedName) -> Self;
}

#[derive(Debug, Clone)]
struct Table<K, V, I = HashMap<FullyQualifiedName, K>>
where
    K: slotmap::Key,
{
    map: SlotMap<K, V>,
    index: I,
    order: Vec<K>,
}

trait WithCapacity {
    fn with_capacity(len: usize) -> Self;
}
impl<K, V> WithCapacity for HashMap<K, V> {
    fn with_capacity(capacity: usize) -> Self {
        ahash::HashMapExt::with_capacity(capacity)
    }
}

impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
    I: Default,
{
    fn with_capacity(len: usize) -> Self {
        Self {
            map: SlotMap::with_capacity_and_key(len),
            index: I::default(),
            order: Vec::with_capacity(len),
        }
    }
}

impl<K, V, N> Default for Table<K, V, N>
where
    K: slotmap::Key,
    N: Default,
{
    fn default() -> Self {
        Self {
            map: SlotMap::with_key(),
            index: Default::default(),
            order: Vec::default(),
        }
    }
}
impl<K, V> Table<K, V, HashMap<FullyQualifiedName, K>>
where
    K: slotmap::Key,
    V: access::FullyQualifiedName,
{
    fn get_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&V> {
        self.index.get(fqn).map(|key| &self.map[*key])
    }
    fn get_mut_by_fqn(&mut self, fqn: &FullyQualifiedName) -> Option<&mut V> {
        self.index.get(fqn).map(|key| &mut self.map[*key])
    }
}

impl<K, V, N> Table<K, V, N>
where
    K: slotmap::Key,
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
}

impl<K, V, N> Index<K> for Table<K, V, N>
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
            index: HashMap::default(),
            order: Vec::new(),
        }
    }
    fn get_or_insert_key(&mut self, fqn: FullyQualifiedName) -> K {
        self.get_or_insert(fqn).0
    }

    fn get_or_insert(&mut self, fqn: FullyQualifiedName) -> (K, &mut V) {
        let key = *self
            .index
            .entry(fqn.clone())
            .or_insert_with(|| self.map.insert(fqn.into()));
        let value = &mut self.map[key];
        if value.key() != key {
            value.set_key(key);
        }
        (key, value)
    }
}

type PackageTable = Table<package::Key, package::Inner>;
type FileTable = Table<file::Key, file::Inner>;
type MessageTable = Table<message::Key, message::Inner>;
type EnumTable = Table<r#enum::Key, r#enum::Inner>;
type EnumValueTable = Table<enum_value::Key, enum_value::Inner>;
type ServiceTable = Table<service::Key, service::Inner>;
type MethodTable = Table<method::Key, method::Inner>;
type FieldTable = Table<field::Key, field::Inner>;
type OneofTable = Table<oneof::Key, oneof::Inner>;
type ExtensionTable = Table<extension::Key, extension::Inner>;
type ExtensionDeclTable = Table<extension_decl::Key, extension_decl::Inner, ()>;

#[derive(Debug, Clone)]
struct Set<K> {
    set: Vec<K>,
    by_name: HashMap<Box<str>, K>,
}

impl<K> Extend<node::Ident<K>> for Set<K>
where
    K: Copy,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = node::Ident<K>>,
    {
    }
}

impl<K> Default for Set<K> {
    fn default() -> Self {
        Self {
            set: Vec::default(),
            by_name: HashMap::default(),
        }
    }
}

impl<K> Set<K>
where
    K: Copy,
{
    fn from_vec(nodes: Vec<node::Ident<K>>) -> Self {
        let mut set = Vec::with_capacity(nodes.len());
        let mut by_name = HashMap::with_capacity(nodes.len());
        for node in nodes {
            set.push(node.key);
            by_name.insert(node.name, node.key);
        }
        Self { set, by_name }
    }
}
impl<K> Set<K>
where
    K: slotmap::Key + Copy,
{
    fn by_name(&self, name: &str) -> Option<K> {
        self.by_name.get(name).copied()
    }
    fn get(&self, index: usize) -> Option<K> {
        self.set.get(index).copied()
    }

    fn from_slice(nodes: &[node::Ident<K>]) -> Self {
        let mut set = Vec::with_capacity(nodes.len());
        let mut by_name = HashMap::with_capacity(nodes.len());
        for node in nodes {
            set.push(node.key);
            by_name.insert(node.name.clone(), node.key);
        }
        Self { set, by_name }
    }
}
impl<K> From<Vec<node::Ident<K>>> for Set<K>
where
    K: Copy,
{
    fn from(v: Vec<node::Ident<K>>) -> Self {
        Self::from_vec(v)
    }
}
impl Index<usize> for Set<package::Key> {
    type Output = package::Key;
    fn index(&self, index: usize) -> &Self::Output {
        &self.set[index]
    }
}

impl<K> PartialEq for Set<K>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set
    }
}
impl<K> Eq for Set<K> where K: PartialEq {}

impl<K> Deref for Set<K> {
    type Target = [K];
    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

#[derive(Debug, Default)]
pub struct Ast {
    packages: PackageTable,
    files: FileTable,
    messages: MessageTable,
    enums: EnumTable,
    enum_values: EnumValueTable,
    services: ServiceTable,
    methods: MethodTable,
    fields: FieldTable,
    oneofs: OneofTable,
    extensions: ExtensionTable,
    extension_blocks: ExtensionDeclTable,
    nodes: HashMap<FullyQualifiedName, node::Key>,
    well_known_package: package::Key,
}

impl Ast {
    fn new(file_descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Self, Error> {
        hydrate::run(file_descriptors, targets)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownType {
    Enum(r#enum::WellKnownEnum),
    Message(message::WellKnownMessage),
}

impl WellKnownType {
    pub const PACKAGE: &'static str = "google.protobuf";
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FullyQualifiedName(Box<str>);

impl AsRef<str> for FullyQualifiedName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Box<str>> for FullyQualifiedName {
    fn from(value: Box<str>) -> Self {
        Self(value)
    }
}
impl From<&str> for FullyQualifiedName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl Deref for FullyQualifiedName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for FullyQualifiedName {
    fn default() -> Self {
        Self("".into())
    }
}
impl From<String> for FullyQualifiedName {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

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
        Self(format!("{container}.{value}").into())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    fn push(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }
        let mut existing = self.0.to_string();
        if !self.0.is_empty() {
            existing.push('.');
        }
        existing.push_str(value);
        self.0 = existing.into();
    }
    fn for_package(package: &str) -> Self {
        if package.is_empty() {
            return Self::default();
        }
        if package.starts_with('.') {
            Self(package.into())
        } else {
            Self(format!(".{package}").into())
        }
    }
}

impl fmt::Display for FullyQualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
            pub(super) fn key(&self) -> $key {
                self.key
            }
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
macro_rules! impl_resolve {
    ($node: ident, $key: ident, $inner: ident) => {
        impl<'ast> crate::ast::resolve::Resolve<$inner> for $node<'ast> {
            fn resolve(&self) -> &$inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
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
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> $node<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl crate::ast::access::FullyQualifiedName for $inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl $inner {
            pub(super) fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
            pub(super) fn fqn(&self) -> &crate::ast::FullyQualifiedName {
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
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, $key, $inner>> for $node<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, $key, $inner>) -> Self {
                Self(resolver)
            }
        }
    };
}

macro_rules! impl_fmt {
    ($node: ident, $key: ident, $inner: ident) => {
        impl<'ast> ::std::fmt::Display for $node<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }

        impl<'ast> ::std::fmt::Debug for $node<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
    };
}
macro_rules! impl_access_reserved {
    ($node: ident, $inner:ident) => {
        impl $inner {
            fn set_reserved<R>(&mut self, names: Vec<String>, ranges: Vec<R>)
            where
                R: Into<crate::ast::reserved::ReservedRange>,
            {
                self.reserved = crate::ast::reserved::Reserved {
                    names: names.into(),
                    ranges: ranges.into_iter().map(Into::into).collect(),
                };
            }
        }
        impl<'ast> $node<'ast> {
            pub fn reserved_names(&self) -> &[String] {
                &self.0.reserved.names
            }
            pub fn reserved_ranges(&self) -> &[crate::ast::reserved::ReservedRange] {
                &self.0.reserved.ranges
            }
            pub fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
        impl<'ast> crate::ast::access::Reserved for $node<'ast> {
            fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
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

macro_rules! impl_set_uninterpreted_options {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
    };
}
macro_rules! impl_access_name {
    ($node:ident, $inner: ident) => {
        impl $inner {
            pub(super) fn name(&self) -> &str {
                &self.name
            }
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for $inner {
            fn name(&self) -> &str {
                &self.name
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
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }

        impl<'ast> $node<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }

        impl $inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
    };
}

macro_rules! inner_method_hydrate_location {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
    };
}

macro_rules! impl_comments {
    ($node:ident, $inner:ident) => {
        impl<'ast> crate::ast::access::Comments for $node<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> $node<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }

        impl $inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
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
        crate::ast::impl_resolve!($node, $key, $inner);
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
    (ExtensionDecl, $key:ident, $inner: ident) => {
        crate::ast::impl_key!($inner, $key);
        crate::ast::node_method_new!(ExtensionDecl, $key);
        crate::ast::node_method_key!(ExtensionDecl, $key);
        crate::ast::node_method_ast!(ExtensionDecl);
        crate::ast::impl_clone_copy!(ExtensionDecl);
        crate::ast::impl_eq!(ExtensionDecl);
        crate::ast::impl_resolve!(ExtensionDecl, $key, $inner);
        crate::ast::impl_from_key_and_ast!(ExtensionDecl, $key, $inner);
        // crate::ast::impl_fmt!(ExtensionDecl, $key, $inner);
    };

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
        crate::ast::inner_method_hydrate_location!($inner);
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
        crate::ast::inner_method_hydrate_location!($inner);
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
        crate::ast::inner_method_hydrate_location!($inner);
    };
}

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
use impl_resolve;
use impl_set_uninterpreted_options;
use impl_span;
use impl_traits_and_methods;
use inner_method_file;
use inner_method_hydrate_location;
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
