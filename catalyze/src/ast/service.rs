use super::{
    access::NodeKeys, file, impl_traits_and_methods, location, node, package, resolve,
    uninterpreted::UninterpretedOption, FullyQualifiedName,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Inner {
    key: Key,

    fqn: FullyQualifiedName,
    node_path: Box<[i32]>,
    span: location::Span,
    comments: Option<location::Comments>,
    name: Box<str>,
    package: Option<package::Key>,
    file: file::Key,
    uninterpreted_options: Vec<UninterpretedOption>,
    methods: Vec<super::method::Key>,
}
impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = node::Key> {
        self.methods.iter().copied().map(Into::into)
    }
}
pub struct Service<'ast>(resolve::Resolver<'ast, Key, Inner>);

impl_traits_and_methods!(Service, Key, Inner);
