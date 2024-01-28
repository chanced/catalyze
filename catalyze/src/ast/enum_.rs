use protobuf::{
    descriptor::{enum_descriptor_proto, EnumOptions},
    SpecialFields,
};

use crate::{
    ast::{
        access::NodeKeys,
        file, impl_traits_and_methods,
        location::{Comments, Span},
        package,
        reference::ReferrerKey,
        resolve::Resolver,
        uninterpreted::UninterpretedOption,
        FullyQualifiedName,
    },
    error::HydrationFailed,
};

use std::{fmt, str::FromStr};

use super::{collection::Collection, container, enum_value, location, node, Name};

slotmap::new_key_type! {
    pub(super) struct Key;
}
pub(super) type Table = super::table::Table<Key, Inner>;
pub(super) type Ident = node::Ident<Key>;

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) package: Option<package::Key>,
    pub(super) file: file::Key,
    pub(super) values: Vec<node::Ident<enum_value::Key>>,
    pub(super) location: location::Detail,
    pub(super) options: protobuf::MessageField<EnumOptions>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) reserved_names: Vec<String>,
    pub(super) reserved_ranges: Vec<enum_descriptor_proto::EnumReservedRange>,
    pub(super) container: container::Key,
    pub(super) well_known: Option<WellKnownEnum>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Name,
    node_path: Box<[i32]>,
    span: Span,
    comments: Option<Comments>,
    reserved: super::reserved::Reserved,
    package: Option<package::Key>,
    file: file::Key,
    container: container::Key,
    referenced_by: Vec<ReferrerKey>,
    values: Collection<super::enum_value::Key>,
    well_known: Option<WellKnownEnum>,
    allow_alias: bool,
    deprecated: bool,
    option_special_fields: SpecialFields,
    uninterpreted_options: Vec<UninterpretedOption>,
    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
}

impl Inner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            name,
            package,
            file,
            values,
            location,
            options,
            special_fields,
            reserved_names,
            reserved_ranges,
            container,
            well_known,
        } = hydrate;

        self.values = values.into();
        self.name = name;
        self.file = file;
        self.package = package;

        self.set_reserved(reserved_names, reserved_ranges);
        self.container = container;
        self.well_known = well_known;
        self.special_fields = special_fields;
        self.hydrate_location(location);
        self.hydrate_options(options.unwrap_or_default());
        Ok(self.into())
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
    pub const fn as_str(self) -> &'static str {
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
