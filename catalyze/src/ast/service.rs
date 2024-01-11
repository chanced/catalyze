use super::{file, impl_traits, package, Accessor, FullyQualifiedName, UninterpretedOption};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Inner {
    fqn: FullyQualifiedName,
    name: String,
    package: Option<package::Key>,
    file: file::Key,
    uninterpreted_options: Vec<UninterpretedOption>,
}

pub struct Service<'ast>(Accessor<'ast, Key, Inner>);

impl_traits!(Service, Key, Inner);
