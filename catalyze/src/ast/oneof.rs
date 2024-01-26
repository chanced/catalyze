use protobuf::{descriptor::OneofOptions, SpecialFields};

use crate::error::HydrateError;

use super::{
    access::NodeKeys,
    field, file, impl_traits_and_methods,
    location::{self, Comments, Span},
    message, node, package,
    resolve::Resolver,
    uninterpreted::{into_uninterpreted_options, UninterpretedOption},
    FullyQualifiedName, Name,
};

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) message: message::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) location: location::Detail,
    pub(super) fields: Vec<field::Ident>,
    pub(super) options: protobuf::MessageField<OneofOptions>,
    pub(super) special_fields: protobuf::SpecialFields,
}

pub(super) type Ident = node::Ident<Key>;
pub struct Oneof<'ast>(Resolver<'ast, Key, Inner>);

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Name,
    message: message::Key,
    package: Option<package::Key>,
    node_path: Box<[i32]>,
    span: Span,
    comments: Option<Comments>,
    file: file::Key,
    uninterpreted_options: Vec<UninterpretedOption>,
    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
    fields: Vec<field::Key>,
}
impl Inner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrateError> {
        let Hydrate {
            name,
            message,
            file,
            package,
            location,
            fields,
            options,
            special_fields,
        } = hydrate;
        self.name = name;
        self.message = message;
        self.package = package;
        self.file = file;
        self.special_fields = special_fields;
        self.hydrate_location(location);
        self.hydrate_options(options.unwrap_or_default())?;

        Ok(self.into())
    }
    fn hydrate_options(&mut self, opts: OneofOptions) -> Result<(), HydrateError> {
        let OneofOptions {
            special_fields,
            uninterpreted_option,
        } = opts;
        self.options_special_fields = special_fields;
        self.uninterpreted_options = into_uninterpreted_options(uninterpreted_option);
        Ok(())
    }
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
