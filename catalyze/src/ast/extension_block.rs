use super::{Comments, Resolver, Span};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    span: Span,
    comments: Option<Comments>,
    extensions: Vec<Key>,
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
/// Note that `ExtensionBlock` is not a [`node`](crate::ast::Node) in the AST,
/// but rather a construct used to organize the [`Extension`] as they are
/// defined in the protobuf. As such, the block does not have a
/// [`FullyQualifiedName`].  It does, however, have a [`Span`] and possibly
/// [`Comments`].
pub struct ExtensionBlock<'ast>(Resolver<'ast, Key, Inner>);

impl<'ast> ExtensionBlock<'ast> {
    pub fn comments(&self) -> Option<&Comments> {
        self.0.comments.as_ref()
    }
}
