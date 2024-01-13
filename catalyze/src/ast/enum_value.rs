use core::panic;

use crate::ast::{impl_traits_and_methods, FullyQualifiedName, Resolver, UninterpretedOption};

use super::{
    access::{NodeAtPath, NodeKeys},
    file, package, State,
};

pub struct EnumValue<'ast>(Resolver<'ast, Key, Inner>);

slotmap::new_key_type! {
    pub(super) struct Key;
}
impl_traits_and_methods!(EnumValue, Key, Inner);

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    state: State,
    fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
}

impl NodeAtPath for Inner {
    fn node_at_path(&self, path: &[i32]) -> Option<super::Key> {
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
