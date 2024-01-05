use crate::{
    ast::{Accessor, Ast},
    fqn::FullyQualifiedName,
};

#[derive(Debug)]
pub struct Oneof<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);

crate::impl_traits!(Oneof, Inner);

slotmap::new_key_type! {
    pub(crate) struct Key;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
}
