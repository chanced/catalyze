use protobuf::{descriptor::EnumOptions, SpecialFields};
use snafu::location;

use crate::ast::{
    access::NodeKeys,
    file, impl_traits_and_methods,
    location::{Comments, Span},
    package,
    reference::ReferrerKey,
    resolve::Resolver,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName,
};

use std::{fmt, str::FromStr};

use super::{container, enum_value, location, node, Set};

slotmap::new_key_type! {
    pub(super) struct Key;
}

pub(super) struct Hydrate {
    pub(super) name: Box<str>,
    pub(super) values: Vec<node::Ident<enum_value::Key>>,
    pub(super) location: location::Detail,
    pub(super) options: protobuf::MessageField<EnumOptions>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) reserved_names: Vec<String>,
    pub(super) reserved_ranges: Vec<protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>,
    pub(super) container: container::Key,
    pub(super) well_known: Option<WellKnownEnum>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    node_path: Box<[i32]>,
    span: Span,
    comments: Option<Comments>,
    reserved: super::reserved::Reserved,
    package: Option<package::Key>,
    file: file::Key,
    container: container::Key,
    referenced_by: Vec<ReferrerKey>,
    values: Set<super::enum_value::Key>,
    well_known: Option<WellKnownEnum>,
    allow_alias: bool,
    deprecated: bool,
    option_special_fields: SpecialFields,
    uninterpreted_options: Vec<UninterpretedOption>,
    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
}

impl Inner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> node::Ident<Key> {
        self.values = hydrate.values.into();
        self.name = hydrate.name;
        self.set_reserved(hydrate.reserved_names, hydrate.reserved_ranges);
        self.container = hydrate.container;
        self.well_known = hydrate.well_known;
        self.special_fields = hydrate.special_fields;
        self.hydrate_location(hydrate.location);
        self.hydrate_options(hydrate.options.unwrap_or_default());
        self.into()
    }

    fn hydrate_options(&mut self, options: EnumOptions) {
        let EnumOptions {
            allow_alias,
            deprecated,
            uninterpreted_option,
            special_fields,
        } = options;
        self.allow_alias = allow_alias.unwrap_or(false);
        self.deprecated = deprecated.unwrap_or(false);
        self.set_uninterpreted_options(uninterpreted_option);
        self.option_special_fields = special_fields;
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::node::Key> {
        self.values.iter().copied().map(super::node::Key::EnumValue)
    }
}

pub struct Enum<'ast>(Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(Enum, Key, Inner);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownEnum {
    /// Whether a field is optional, required, or repeated.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#cardinality>
    FieldCardinality,
    /// Basic field types.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#kind>
    FieldKind,

    /// NullValue is a singleton enumeration to represent the null value for the
    /// Value type union.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#nullvalue>
    NullValue,
    /// The syntax in which a protocol buffer element is defined.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#syntax>
    Syntax,
}
impl WellKnownEnum {
    const FIELD_CARDINALITY: &'static str = "FieldCardinality";
    const FIELD_KIND: &'static str = "FieldKind";
    const NULL_VALUE: &'static str = "NullValue";
    const SYNTAX: &'static str = "Syntax";
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::FieldCardinality => Self::FIELD_CARDINALITY,
            Self::FieldKind => Self::FIELD_KIND,
            Self::NullValue => Self::NULL_VALUE,
            Self::Syntax => Self::SYNTAX,
        }
    }
}
impl FromStr for WellKnownEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::FIELD_CARDINALITY => Ok(Self::FieldCardinality),
            Self::FIELD_KIND => Ok(Self::FieldKind),
            Self::NULL_VALUE => Ok(Self::NullValue),
            Self::SYNTAX => Ok(Self::Syntax),
            _ => Err(()),
        }
    }
}

impl fmt::Display for WellKnownEnum {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}
