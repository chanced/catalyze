use inherent::inherent;

use crate::fqn::{Fqn, FullyQualifiedName};

use super::Inner as FieldInner;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
struct Inner {
    field: FieldInner,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumField(Arc<Inner>);

#[inherent]
impl Fqn for EnumField {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.field.fqn
    }
}
