use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct Inner {
    fqn: FullyQualifiedName,
}

pub struct Method<'ast>(Accessor<'ast, Key, Inner>);

impl_traits!(Method, Key, Inner);
