pub mod access;
pub mod container;
pub mod dependency;
pub mod dependent;
pub mod enum_;
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
pub mod value;

mod collection;
mod hydrate;
mod map_try_into_usize;
mod resolve;
mod table;

pub use container::Container;
pub use dependency::Dependency;
pub use dependent::Dependent;
pub use enum_::Enum;
pub use enum_value::EnumValue;
pub use extension::Extension;
pub use field::Field;
pub use file::File;
pub use location::{Comments, Span};
pub use message::Message;
pub use method::Method;
pub use node::Node;
pub use oneof::Oneof;
pub use package::Package;
pub use reference::{Reference, Referent};
pub use service::Service;

use crate::{error::Error, HashMap};

use protobuf::descriptor::FileDescriptorProto;
use std::{borrow::Borrow, fmt, ops::Deref, path::PathBuf};

use self::{
    enum_::EnumTable,
    enum_value::EnumValueTable,
    extension::ExtensionTable,
    extension_decl::ExtensionDeclTable,
    field::FieldTable,
    file::FileTable,
    message::MessageTable,
    method::MethodTable,
    node::NodeMap,
    oneof::OneofTable,
    package::{PackageKey, PackageTable},
    service::ServiceTable,
};

trait FromFqn {
    fn from_fqn(fqn: FullyQualifiedName) -> Self;
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
    extension_decls: ExtensionDeclTable,
    nodes: NodeMap,
    well_known: PackageKey,
}

impl Ast {
    pub(crate) fn build(
        file_descriptors: Vec<FileDescriptorProto>,
        targets: &[String],
    ) -> Result<Self, Error> {
        hydrate::run(file_descriptors, targets)
    }
    pub(crate) fn new(file_count: usize) -> Self {
        let (well_known, packages) = Self::create_package_table();
        let files = FileTable::with_capacity(file_count);
        Self {
            packages,
            files,
            well_known,
            ..Default::default()
        }
    }
    fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }
    fn create_package_table() -> (PackageKey, PackageTable) {
        let mut packages = PackageTable::with_capacity(1);
        let key = packages.get_or_insert_key(FullyQualifiedName::for_package(
            package::WELL_KNOWN.to_string(),
        ));
        (key, packages)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownType {
    Enum(enum_::WellKnownEnum),
    Message(message::WellKnownMessage),
}

impl WellKnownType {
    pub const PACKAGE: &'static str = "google.protobuf";
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FullyQualifiedName(Box<str>);

impl From<&std::path::Path> for FullyQualifiedName {
    fn from(path: &std::path::Path) -> Self {
        Self(path.to_string_lossy().into())
    }
}
impl Borrow<str> for FullyQualifiedName {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl PartialEq<&str> for FullyQualifiedName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
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
    pub fn new(value: &str, container: Option<Self>) -> Self {
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
    pub fn for_file(package: Option<Self>) -> Self {
        package.map_or(Self(".".into()), |fqn| fqn)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(Box<str>);
impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.0.into()
    }
}
impl From<Name> for Box<str> {
    fn from(name: Name) -> Self {
        name.0
    }
}
impl From<Name> for PathBuf {
    fn from(value: Name) -> Self {
        value.as_str().into()
    }
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

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
        impl<'ast> crate::ast::access::AccessReserved for $node<'ast> {
            fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
    };
}

macro_rules! inner_method_package {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::PackageKey> {
                self.package
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
            pub(super) fn key(self) -> $key {
                self.0.key
            }
        }
    };
}

macro_rules! inner_method_hydrate_location {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Location) {
                self.comments = location.comments;
                self.span = location.span;
                self.proto_path = location.path.into();
            }
        }
    };
}

macro_rules! impl_base_traits_and_methods {
    ($node:ident, $key:ident, $inner:ident) => {
        crate::ast::node_method_new!($node, $key);
        crate::ast::node_method_key!($node, $key);
        crate::ast::node_method_ast!($node);
        crate::ast::impl_clone_copy!($node);
        crate::ast::impl_eq!($node);
        crate::ast::impl_resolve!($node, $key, $inner);
        crate::ast::impl_from_key_and_ast!($node, $key, $inner);
        crate::ast::impl_fmt!($node, $key, $inner);
        crate::ast::impl_from_fqn!($inner);
        // crate::ast::impl_state!($inner);
        // crate::ast::impl_fsm!($inner);
    };
}
macro_rules! impl_traits_and_methods {
    (ExtensionDecl, $key:ident, $inner: ident) => {
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
        crate::ast::inner_method_package!($inner);
    };

    (Message, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Message, $key, $inner);
        crate::ast::impl_access_reserved!(Message, $inner);
        crate::ast::inner_method_package!($inner);
        crate::ast::inner_method_hydrate_location!($inner);
    };

    (Enum, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Enum, $key, $inner);
        crate::ast::impl_access_reserved!(Enum, $inner);
        crate::ast::inner_method_package!($inner);
        crate::ast::inner_method_hydrate_location!($inner);
    };

    ($node: ident, $key: ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!($node, $key, $inner);
        crate::ast::inner_method_package!($inner);
        crate::ast::inner_method_hydrate_location!($inner);
    };
}

use impl_access_reserved;
use impl_base_traits_and_methods;
use impl_clone_copy;
use impl_eq;
use impl_fmt;
use impl_from_fqn;
use impl_from_key_and_ast;
use impl_resolve;
use impl_traits_and_methods;
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
