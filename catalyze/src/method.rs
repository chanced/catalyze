use crate::{
    ast::{Accessor, Ast},
    fqn::{Fqn, FullyQualifiedName},
    impl_traits,
};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug)]
pub struct Method<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);

impl_traits!(Method, Inner);
