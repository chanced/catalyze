use crate::{
    ast::{Accessor, Ast},
    impl_traits,
};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Inner {}

#[derive(Debug)]
pub struct Extension<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);
impl_traits!(Extension, Key, Inner);
