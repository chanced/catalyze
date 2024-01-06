use crate::{
    ast::{Accessor, Ast, FullyQualifiedName},
    impl_traits,
};

use std::fmt;

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
}

pub struct Enum<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);
impl_traits!(Enum, Key, Inner);

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

    fn from_str(s: &str) -> ::std::result::Result<WellKnownEnum, Self::Err> {
        match s {
            Self::FIELD_CARDINALITY => Ok(WellKnownEnum::FieldCardinality),
            Self::FIELD_KIND => Ok(WellKnownEnum::FieldKind),
            Self::NULL_VALUE => Ok(WellKnownEnum::NullValue),
            Self::SYNTAX => Ok(WellKnownEnum::Syntax),
            _ => Err(()),
        }
    }
}
