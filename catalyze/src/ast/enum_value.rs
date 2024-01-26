use protobuf::{descriptor::EnumValueOptions, SpecialFields};

use crate::error::HydrationError;

use super::{
    access::NodeKeys, file, impl_traits_and_methods, location, node, package, resolve::Resolver,
    uninterpreted::UninterpretedOption, FullyQualifiedName, Name,
};

pub(super) type Ident = node::Ident<Key>;
pub(super) type Table = super::table::Table<Key, Inner>;

pub struct EnumValue<'ast>(Resolver<'ast, Key, Inner>);

slotmap::new_key_type! {
    pub(super) struct Key;
}
impl_traits_and_methods!(EnumValue, Key, Inner);

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) number: i32,
    pub(super) location: location::Detail,
    pub(super) options: protobuf::MessageField<EnumValueOptions>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) enum_: super::enum_::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
}

/// [`EnumValue`] inner data.
#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    /// enum_value::Key
    key: Key,
    fqn: FullyQualifiedName,
    name: Name,
    node_path: Box<[i32]>,

    number: i32,

    enum_: super::enum_::Key,
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
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationError> {
        let Hydrate {
            name,
            number,
            location,
            options,
            special_fields,
            enum_,
            file,
            package,
        } = hydrate;
        self.name = name;
        self.number = number;
        self.file = file;
        self.package = package;
        self.special_fields = special_fields;
        self.enum_ = enum_;
        self.hydrate_options(options.unwrap_or_default())?;
        self.hydrate_location(location);
        Ok(self.into())
    }

    fn hydrate_options(&mut self, options: EnumValueOptions) -> Result<(), HydrationError> {
        self.options_special_fields = options.special_fields;
        self.deprecated = options.deprecated.unwrap_or(false);
        self.set_uninterpreted_options(options.uninterpreted_option);
        Ok(())
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = node::Key> {
        std::iter::empty()
    }
}
