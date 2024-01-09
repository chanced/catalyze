use crate::ast::{impl_traits, Access, Accessor, Ast, FullyQualifiedName};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
}

pub struct EnumValue<'ast>(Accessor<'ast, Key, Inner>);
impl_traits!(EnumValue, Key, Inner);
