use std::sync::Arc;

use inherent::inherent;

use crate::fqn::{Fqn, FullyQualifiedName};

use super::{Inner as FieldInner, Scalar};

#[derive(Debug, Clone, PartialEq)]
struct Inner {
    field: FieldInner,
    scalar: Scalar,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarField(Arc<Inner>);

impl ScalarField {
    pub fn scalar(&self) -> Scalar {
        self.0.scalar
    }
}

#[inherent]
impl Fqn for ScalarField {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.field.fqn
    }
}
