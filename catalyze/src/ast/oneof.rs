use super::{
    access::NodeKeys, field, file, impl_traits_and_methods, package,
    uninterpreted::UninterpretedOption, Comments, FullyQualifiedName, Resolver, Span,
};

pub struct Oneof<'ast>(Resolver<'ast, Key, Inner>);

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    package: Option<package::Key>,
    node_path: Box<[i32]>,
    span: Span,
    comments: Option<Comments>,
    file: file::Key,
    uninterpreted_options: Vec<UninterpretedOption>,
    fields: Vec<field::Key>,
}
impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::node::Key> {
        self.fields.iter().copied().map(super::node::Key::Field)
    }
}
impl_traits_and_methods!(Oneof, Key, Inner);

slotmap::new_key_type! {
    pub(super) struct Key;
}
