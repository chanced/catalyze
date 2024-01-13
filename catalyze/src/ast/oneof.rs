use super::{
    file, impl_traits_and_methods, package, FullyQualifiedName, Resolver, State,
    UninterpretedOption,
};

pub struct Oneof<'ast>(Resolver<'ast, Key, Inner>);

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    state: State,
    fqn: FullyQualifiedName,
    package: Option<package::Key>,
    file: file::Key,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
}

impl_traits_and_methods!(Oneof, Key, Inner);

slotmap::new_key_type! {
    pub(super) struct Key;
}
