use inherent::inherent;

use crate::{fqn::Fqn, message::Message, r#enum::Enum};

use super::{Inner as FieldInner, Scalar};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Int64 = 3,
    Uint64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    String = 9,
    Uint32 = 13,
    Sfixed32 = 15,
    Sfixed64 = 16,
    Sint32 = 17,
    Sint64 = 18,
}

#[derive(Debug, Clone, PartialEq)]
struct Inner<T> {
    field_inner: FieldInner,
    key: Key,
    value: T,
}

/// Represents a field marked as `repeated`. The field can hold
/// a scalar value, an enum, or a message.
#[derive(Debug, Clone, PartialEq)]
pub enum MapField {
    Scalar(MapScalarField),
    Enum(MapEnumField),
    Embed(MapEmbedField),
}

#[inherent]
impl Fqn for MapField {
    pub fn fully_qualified_name(&self) -> &crate::fqn::FullyQualifiedName {
        match self {
            Self::Scalar(f) => f.fully_qualified_name(),
            Self::Enum(f) => f.fully_qualified_name(),
            Self::Embed(f) => f.fully_qualified_name(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapScalarField(Arc<Inner<Scalar>>);

#[inherent]
impl Fqn for MapScalarField {
    pub fn fully_qualified_name(&self) -> &crate::fqn::FullyQualifiedName {
        &self.0.field_inner.fqn
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapEnumField(Arc<Inner<Enum>>);

#[inherent]
impl Fqn for MapEnumField {
    pub fn fully_qualified_name(&self) -> &crate::fqn::FullyQualifiedName {
        &self.0.field_inner.fqn
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapEmbedField(Arc<Inner<Message>>);

#[inherent]
impl Fqn for MapEmbedField {
    pub fn fully_qualified_name(&self) -> &crate::fqn::FullyQualifiedName {
        &self.0.field_inner.fqn
    }
}
