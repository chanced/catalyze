use crate::ast::{impl_traits, Accessor, FullyQualifiedName, UninterpretedOption};

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

pub struct Extension<'ast>(Accessor<'ast, Key, Inner>);
impl_traits!(Extension, Key, Inner);
