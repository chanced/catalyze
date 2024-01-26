use super::{file, location, package, resolve};

slotmap::new_key_type! {
    pub(super) struct Key;
}

pub(super) type Table = super::table::Table<Key, Inner>;

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    span: location::Span,
    node_path: Box<[i32]>,
    comments: Option<location::Comments>,
    extensions: Vec<Key>,
    file: file::Key,
    package: Option<package::Key>,
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
