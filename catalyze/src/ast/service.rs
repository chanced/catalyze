use super::{
    access::NodeKeys, file, impl_traits_and_methods, package, Comments, FullyQualifiedName,
    Resolver, Span, State, UninterpretedOption,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Inner {
    key: Key,
    state: State,
    fqn: FullyQualifiedName,
    node_path: Vec<i32>,
    span: Span,
    comments: Option<Comments>,
    name: String,
    package: Option<package::Key>,
    file: file::Key,
    uninterpreted_options: Vec<UninterpretedOption>,
    methods: Vec<super::method::Key>,
}
impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::Key> {
        self.methods.iter().copied().map(Into::into)
    }
}
pub struct Service<'ast>(Resolver<'ast, Key, Inner>);

impl_traits_and_methods!(Service, Key, Inner);
