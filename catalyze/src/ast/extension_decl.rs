use std::ops::{Deref, DerefMut};

use super::{file, location, package, reference, resolve};

slotmap::new_key_type! {
    pub(super) struct Key;
}

pub(super) struct Table(super::table::Table<Key, Inner>);
impl Deref for Table {
    type Target = super::table::Table<Key, Inner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Table {
    pub fn push(&mut self, mut inner: Inner) {
        let key = self.0.map.insert(inner);
        inner.key = key;
        self.0.order.push(key);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    span: location::Span,
    node_path: Box<[i32]>,
    comments: Option<location::Comments>,
    extensions: Vec<Key>,
    references: Vec<reference::Inner>,
    file: file::Key,
    package: Option<package::Key>,
}

impl Inner {
    pub(super) fn new(
        location: location::Detail,
        file: file::Key,
        package: Option<package::Key>,
        ext_count: usize,
    ) -> Self {
        Self {
            key: Key::default(),
            span: location.span,
            node_path: location.path,
            comments: location.comments,
            extensions: Vec::with_capacity(ext_count),
            references: Vec::with_capacity(ext_count),
            file,
            package,
        }
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
