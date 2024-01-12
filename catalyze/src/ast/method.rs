use super::{
    file, impl_traits_and_methods,
    message::{self, Message},
    package, FullyQualifiedName, Resolver, UninterpretedOption,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    fqn: FullyQualifiedName,
    package: Option<package::Key>,
    file: file::Key,
    name: String,
    uninterpreted_options: Vec<UninterpretedOption>,
    input: message::Key,
    output: message::Key,
}

pub struct Method<'ast>(Resolver<'ast, Key, Inner>);

impl<'ast> Method<'ast> {
    pub fn input(self) -> Message<'ast> {
        Message::new(self.0.input, self.0.ast)
    }

    pub(crate) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
        Self(Resolver::new(key, ast))
    }
}

impl_traits_and_methods!(Method, Key, Inner);
