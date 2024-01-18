use protobuf::SpecialFields;

use crate::ast::{
    impl_traits_and_methods, uninterpreted::UninterpretedOption, FullyQualifiedName, Resolver,
};

use super::{access::NodeKeys, file, node, package, Comments, Span};

pub struct EnumValue<'ast>(Resolver<'ast, Key, Inner>);

slotmap::new_key_type! {
    pub(super) struct Key;
}
impl_traits_and_methods!(EnumValue, Key, Inner);

pub(super) struct Hydrate {
    pub(super) name: Box<str>,
    pub(super) number: i32,
    pub(super) location: super::location::Location,
    pub(super) options: protobuf::MessageField<protobuf::descriptor::EnumValueOptions>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) r#enum: super::r#enum::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    number: i32,
    node_path: Box<[i32]>,
    r#enum: super::r#enum::Key,
    file: file::Key,
    package: Option<package::Key>,

    span: Span,
    comments: Option<Comments>,
    uninterpreted_options: Vec<UninterpretedOption>,
    ///  Is this enum value deprecated?
    ///
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating enum values.
    pub deprecated: bool,
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
        self.hydrate_options(options.unwrap_or_default());
        (self.key, self.fqn.clone(), self.name.clone())
    }

    fn hydrate_options(&mut self, options: protobuf::descriptor::EnumValueOptions) {
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
