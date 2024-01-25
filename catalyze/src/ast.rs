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
pub mod relationship;
pub mod reserved;
pub mod service;
pub mod uninterpreted;

mod collection;
mod hydrate;
mod resolve;
mod table;

use crate::HashMap;

use protobuf::descriptor::FileDescriptorProto;
use std::{
    borrow::Borrow,
    fmt,
    ops::{Deref, Index, IndexMut},
    path::PathBuf,
};

use slotmap::SlotMap;

use crate::error::Error;

trait FromFqn {
    fn from_fqn(fqn: FullyQualifiedName) -> Self;
}

#[derive(Debug, Default)]
pub struct Ast {
    packages: PackageTable,
    files: FileTable,
    files_by_name: HashMap<Name, file::Key>,
    files_by_path: HashMap<PathBuf, file::Key>,
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
    well_known: package::Key,
}

impl Ast {
    fn build(
        file_descriptors: Vec<FileDescriptorProto>,
        targets: &[String],
    ) -> Result<Self, Error> {
        hydrate::run(file_descriptors, targets)
    }

    fn new(file_count: usize) -> Self {
        let (well_known, packages) = Self::create_package_table();
        let files = FileTable::with_capacity(file_count);
        Self {
            packages,
            well_known,
            files,
            ..Default::default()
        }
    }

    fn create_package_table() -> (package::Key, PackageTable) {
        let mut packages = PackageTable::with_capacity(1);
        let key = packages.get_or_insert_key(FullyQualifiedName::for_package(
            package::WELL_KNOWN.to_string(),
        ));
        (key, packages)
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

impl Borrow<str> for FullyQualifiedName {
    fn borrow(&self) -> &str {
        &self.0
    }
}
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
    fn for_package(package: String) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(Box<str>);

impl Borrow<str> for Name {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<Box<str>> for Name {
    fn from(value: Box<str>) -> Self {
        Self(value)
    }
}
impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl Deref for Name {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Name {
    fn default() -> Self {
        Self(Box::default())
    }
}

impl Name {
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Name {
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

use self::table::{
    EnumTable, EnumValueTable, ExtensionDeclTable, ExtensionTable, FieldTable, FileTable,
    MessageTable, MethodTable, OneofTable, PackageTable, ServiceTable,
};

// use impl_state;

#[cfg(test)]
mod test {
    use protobuf::{plugin::CodeGeneratorRequest, Message};
    pub(super) fn code_generator_request(bytes: impl AsRef<[u8]>) -> CodeGeneratorRequest {
        CodeGeneratorRequest::parse_from_bytes(bytes.as_ref()).unwrap()
    }
    pub(super) fn commented_cgr() -> CodeGeneratorRequest {
        code_generator_request(include_bytes!(
            "../../fixtures/cgr/commented/code_generator_request.bin"
        ))
    }
}
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
