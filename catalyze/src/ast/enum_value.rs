use crate::ast::{
    impl_traits_and_methods, uninterpreted::UninterpretedOption, FullyQualifiedName, Resolver,
};

use super::{
    access::{AtPath, NodeKeys},
    file, package, Comments, Span,
};

pub struct EnumValue<'ast>(Resolver<'ast, Key, Inner>);

slotmap::new_key_type! {
    pub(super) struct Key;
}
impl_traits_and_methods!(EnumValue, Key, Inner);

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    node_path: Box<[i32]>,
    file: file::Key,
    span: Span,
    comments: Option<Comments>,
    package: Option<package::Key>,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
}

impl AtPath for Inner {
    fn at_path(&self, path: &[i32]) -> Option<super::Key> {
        if path.is_empty() {
            Some(self.key.into())
        } else {
            None
        }
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::Key> {
        std::iter::empty()
    }
}
