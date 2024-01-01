use std::{
    fmt,
    path::Display,
    sync::{Arc, Weak},
};

use inherent::inherent;

use crate::{
    fqn::{Fqn, FullyQualifiedName},
    node::{Downgrade, Upgrade},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum(Arc<Inner>);

#[inherent]
impl Fqn for Enum {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
impl Downgrade for Enum {
    type Target = WeakEnum;

    fn downgrade(&self) -> Self::Target {
        self.clone().into()
    }
}

#[derive(Debug)]
pub(crate) struct WeakEnum(Weak<Inner>);

impl From<Enum> for WeakEnum {
    fn from(value: Enum) -> Self {
        Self(Arc::downgrade(&value.0))
    }
}

impl PartialEq<Enum> for WeakEnum {
    fn eq(&self, other: &Enum) -> bool {
        self.upgrade() == *other
    }
}
impl PartialEq<WeakEnum> for Enum {
    fn eq(&self, other: &WeakEnum) -> bool {
        *self == other.upgrade()
    }
}
impl PartialEq for WeakEnum {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade() == other.upgrade()
    }
}
impl Upgrade for WeakEnum {
    type Target = Enum;

    fn upgrade(&self) -> Self::Target {
        Enum(self.0.upgrade().unwrap())
    }
}
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
