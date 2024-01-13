use super::{
    access::NodeKeys,
    file, impl_traits_and_methods,
    message::{self, Message},
    package,
    reference::{ReferenceInner, References},
    Comments, FullyQualifiedName, Resolver, Span, State, UninterpretedOption,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    state: State,
    fqn: FullyQualifiedName,
    node_path: Vec<i32>,
    span: Span,
    comments: Option<Comments>,
    package: Option<package::Key>,
    file: file::Key,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
    input: message::Key,
    output: message::Key,
    references: [ReferenceInner; 2],
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::Key> {
        std::iter::empty()
    }
}
impl Inner {
    pub(super) fn references_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
        self.references.iter_mut()
    }
}

pub struct Method<'ast>(Resolver<'ast, Key, Inner>);

impl<'ast> Method<'ast> {
    pub fn input(self) -> Message<'ast> {
        Message::new(self.0.input, self.0.ast)
    }
}
impl<'ast> Method<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        super::access::References::references(self)
    }
}

impl<'ast> super::access::References<'ast> for Method<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_slice(&self.0.references, self.ast())
    }
}
impl_traits_and_methods!(Method, Key, Inner);
