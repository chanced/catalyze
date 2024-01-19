use protobuf::{descriptor::EnumValueOptions, SpecialFields};

use crate::ast::{
    impl_traits_and_methods, resolve::Resolver, uninterpreted::UninterpretedOption,
    FullyQualifiedName,
};

use super::{access::NodeKeys, file, location, node, package};

pub struct EnumValue<'ast>(Resolver<'ast, Key, Inner>);

slotmap::new_key_type! {
    pub(super) struct Key;
}
impl_traits_and_methods!(EnumValue, Key, Inner);

pub(super) struct Hydrate {
    pub(super) name: Box<str>,
    pub(super) number: i32,
    pub(super) location: location::Detail,
    pub(super) options: protobuf::MessageField<EnumValueOptions>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) r#enum: super::r#enum::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

/// [`EnumValue`] inner data.
#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    /// enum_value::Key
    key: Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    node_path: Box<[i32]>,

    number: i32,

    r#enum: super::r#enum::Key,
    file: file::Key,
    package: Option<package::Key>,

    span: location::Span,
    comments: Option<location::Comments>,

    // options
    deprecated: bool,

    uninterpreted_options: Vec<UninterpretedOption>,

    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
}
impl Inner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> super::Hydrated<Key> {
        let Hydrate {
            name,
            number,
            location,
            options,
            special_fields,
            r#enum,
            file,
            package,
        } = hydrate;
        self.name = name;
        self.number = number;
        self.comments = location.comments;
        self.file = file;
        self.span = location.span;
        self.package = package;
        self.special_fields = special_fields;
        self.r#enum = r#enum;

        let opts = options.clone().unwrap();

        self.hydrate_options(options.unwrap_or_default());
        (self.key, self.fqn.clone(), self.name.clone())
    }

    fn hydrate_options(&mut self, options: EnumValueOptions) {
        self.options_special_fields = options.special_fields;
        self.deprecated = options.deprecated.unwrap_or(false);
        self.set_uninterpreted_options(options.uninterpreted_option);
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = node::Key> {
        std::iter::empty()
    }
}
