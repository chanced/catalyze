use inherent::inherent;

use crate::fqn::{Fqn, FullyQualifiedName};

use super::{EmbedField, EnumField, Scalar, ScalarField};

/// Represents a field marked as `repeated`. The field can hold
/// a scalar value, an enum, or a message.
#[derive(Debug, Clone, PartialEq)]
pub enum RepeatedField {
    Scalar(RepeatedScalarField),
    Enum(RepeatedEnumField),
    Embed(RepeatedEmbedField),
}

#[inherent]
impl Fqn for RepeatedField {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        match self {
            Self::Scalar(f) => f.fully_qualified_name(),
            Self::Enum(f) => f.fully_qualified_name(),
            Self::Embed(f) => f.fully_qualified_name(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatedScalarField(ScalarField);

#[inherent]
impl Fqn for RepeatedScalarField {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        self.0.fully_qualified_name()
    }
}
impl RepeatedScalarField {
    pub fn scalar(&self) -> Scalar {
        self.0.scalar()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatedEnumField(EnumField);

#[inherent]
impl Fqn for RepeatedEnumField {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        self.0.fully_qualified_name()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatedEmbedField(EmbedField);

#[inherent]
impl Fqn for RepeatedEmbedField {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        self.0.fully_qualified_name()
    }
}
