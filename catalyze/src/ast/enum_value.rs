use crate::ast::{
    impl_traits_and_methods, uninterpreted::UninterpretedOption, FullyQualifiedName, Resolver,
};

use super::{access::NodeKeys, file, node, package, Comments, Span};

pub struct EnumValue<'ast>(Resolver<'ast, Key, Inner>);

slotmap::new_key_type! {
    pub(super) struct Key;
}
impl_traits_and_methods!(EnumValue, Key, Inner);

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    node_path: Box<[i32]>,
    file: file::Key,
    span: Span,
    comments: Option<Comments>,
    package: Option<package::Key>,
    uninterpreted_options: Vec<UninterpretedOption>,
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = node::Key> {
        std::iter::empty()
    }
}
