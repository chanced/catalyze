use inherent::inherent;

use crate::fqn::Fqn;

use super::Inner as FieldInner;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
struct Inner {
    field: FieldInner,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OneofField(Arc<Inner>);

#[inherent]
impl Fqn for OneofField {
    pub fn fully_qualified_name(&self) -> &crate::fqn::FullyQualifiedName {
        &self.0.field.fqn
    }
}
