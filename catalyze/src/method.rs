use crate::{
    ast::{impl_traits, Accessor, Ast, FullyQualifiedName, UninterpretedOption},
    file, package,
};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    fqn: FullyQualifiedName,
    package: Option<package::Key>,
    file: file::Key,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
}

pub struct Method<'ast>(Accessor<'ast, Key, Inner>);

impl_traits!(Method, Key, Inner);
