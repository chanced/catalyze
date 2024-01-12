use crate::ast::{impl_traits_and_methods, FullyQualifiedName, Resolver, UninterpretedOption};

use super::{file, package};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    fqn: FullyQualifiedName,
    package: Option<package::Key>,
    file: file::Key,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
}

pub struct Extension<'ast>(Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(Extension, Key, Inner);
