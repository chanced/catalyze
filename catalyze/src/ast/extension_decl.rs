use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use super::{extension, file, location, package, reference, resolve};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Default, Clone)]
pub(super) struct Table(super::table::Table<Key, Inner>);
impl Deref for Table {
    type Target = super::table::Table<Key, Inner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Table {
    pub fn push(&mut self, inner: Inner) -> Key {
        let key = self.0.map.insert(inner);
        let inner = self.0.get_mut(key).unwrap();
        inner.key = key;
        self.0.order.push(key);
        key
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    pub(super) key: Key,
    pub(super) span: location::Span,
    pub(super) node_path: Box<[i32]>,
    pub(super) comments: Option<location::Comments>,
    pub(super) extensions: Vec<extension::Key>,
    pub(super) references: Vec<reference::Inner>,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

impl Inner {
    pub(super) fn hydrate(
        &mut self,
        location: location::Detail,
        file: file::Key,
        package: Option<package::Key>,
        ext_count: usize,
    ) {
        self.span = location.span;
        self.node_path = location.path;
        self.comments = location.comments;
        self.file = file;
        self.package = package;
        self.extensions = Vec::with_capacity(ext_count);
        self.references = Vec::with_capacity(ext_count);
    }
}
/// A set of [`Extension`] which are defined together in a single message-like
/// structure.
///
/// ```proto
/// extend Foo {
///    optional int32 bar = 126;
///    optional int32 baz = 127;
/// }
/// ```
///
/// In the above example, `bar` and `baz` would be included the same block.
///
/// Note that `ExtensionDecl` is not a [`node`](crate::ast::Node) in the AST,
/// but rather a construct used to organize the [`Extension`] as they are
/// defined in the protobuf. As such, the block does not have a
/// [`FullyQualifiedName`].  It does, however, have a [`Span`] and possibly
/// [`Comments`].
pub struct ExtensionDecl<'ast>(resolve::Resolver<'ast, Key, Inner>);

impl<'ast> ExtensionDecl<'ast> {}

super::impl_traits_and_methods!(ExtensionDecl, Key, Inner);
