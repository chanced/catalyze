use crate::{
    ast::{Accessor, Ast},
    fqn::FullyQualifiedName,
    impl_traits,
};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
}

pub struct Service<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);

impl_traits!(Service, Key, Inner);
