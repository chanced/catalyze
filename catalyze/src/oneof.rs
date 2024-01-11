use crate::{
    ast::{impl_traits, Accessor, Ast, FullyQualifiedName, UninterpretedOption},
    file, package,
};

pub struct Oneof<'ast>(Accessor<'ast, Key, Inner>);

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
    package: Option<package::Key>,
    file: file::Key,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
}

impl_traits!(Oneof, Key, Inner);

slotmap::new_key_type! {
    pub(crate) struct Key;
}
