use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
}

pub struct Oneof<'ast>(Accessor<'ast, Key, Inner>);

impl_traits!(Oneof, Key, Inner);

slotmap::new_key_type! {
    pub(crate) struct Key;
}
