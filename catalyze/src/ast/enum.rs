use protobuf::descriptor::EnumOptions;

use crate::{
    ast::{
        impl_traits_and_methods, uninterpreted::UninterpretedOption, ContainerKey,
        FullyQualifiedName, Resolver,
    },
    error::Error,
};

use std::fmt;

use super::{
    access::NodeKeys, enum_value, file, package, reference::ReferrerKey, Comments, Reserved, Span,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    node_path: Box<[i32]>,
    span: Span,
    comments: Option<Comments>,
    reserved: Reserved,
    package: Option<package::Key>,
    file: file::Key,
    container: ContainerKey,
    name: String,
    referenced_by: Vec<ReferrerKey>,
    values: Vec<super::enum_value::Key>,
    uninterpreted_options: Vec<UninterpretedOption>,
}
impl Inner {
    fn hydrate_options(&mut self, options: EnumOptions) -> Result<(), Error> {
        let EnumOptions {
            allow_alias,
            deprecated,
            uninterpreted_option,
            special_fields,
        } = options;
    }

    pub(crate) fn hydrate(
        &mut self,
        values: Vec<enum_value::Key>,
        options: protobuf::MessageField<EnumOptions>,
        reserved_name: Vec<String>,
        reserved_range: Vec<protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>,
    ) -> Result<(Key, FullyQualifiedName, Box<[i32]>), Error> {
        todo!()
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::Key> {
        self.values.iter().copied().map(super::Key::EnumValue)
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

impl fmt::Display for WellKnownEnum {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl std::str::FromStr for WellKnownEnum {
    type Err = ();

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            Self::FIELD_CARDINALITY => Ok(Self::FieldCardinality),
            Self::FIELD_KIND => Ok(Self::FieldKind),
            Self::NULL_VALUE => Ok(Self::NullValue),
            Self::SYNTAX => Ok(Self::Syntax),
            _ => Err(()),
        }
    }
}
