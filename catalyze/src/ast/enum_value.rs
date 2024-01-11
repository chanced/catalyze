use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName, UninterpretedOption};

use super::{file, package};

pub struct EnumValue<'ast>(Accessor<'ast, Key, Inner>);

slotmap::new_key_type! {
    pub(crate) struct Key;
}
impl_traits!(EnumValue, Key, Inner);

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
    file: file::Key,
    package: Option<package::Key>,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
}
