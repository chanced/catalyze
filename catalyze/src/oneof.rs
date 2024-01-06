use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};

pub struct Oneof<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);

impl_traits!(Oneof, Key, Inner);

slotmap::new_key_type! {
    pub(crate) struct Key;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
}
