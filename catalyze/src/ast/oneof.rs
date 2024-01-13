use super::{
    access::NodeKeys, field, file, impl_traits_and_methods, package, Comments, FullyQualifiedName,
    Resolver, Span, State, UninterpretedOption,
};

pub struct Oneof<'ast>(Resolver<'ast, Key, Inner>);

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    state: State,
    fqn: FullyQualifiedName,
    package: Option<package::Key>,
    node_path: Vec<i32>,
    span: Span,
    comments: Option<Comments>,
    file: file::Key,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
    fields: Vec<field::Key>,
}
impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::Key> {
        self.fields.iter().copied().map(super::Key::Field)
    }
}
impl_traits_and_methods!(Oneof, Key, Inner);

slotmap::new_key_type! {
    pub(super) struct Key;
}
