use std::sync::{Arc, Weak};

use inherent::inherent;

use crate::fqn::{Fqn, FullyQualifiedName};

pub struct WeakEnum(Weak<Inner>);

#[derive(Debug, Clone, PartialEq)]
pub struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum(Arc<Inner>);

#[inherent]
impl Fqn for Enum {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
