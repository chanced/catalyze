use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Inner {
    fqn: FullyQualifiedName,
}

pub struct Method<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);

impl_traits!(Method, Key, Inner);
