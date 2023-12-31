use inherent::inherent;
use std::sync::{Arc, Weak};

use crate::fqn::{Fqn, FullyQualifiedName};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValue(Arc<Inner>);

#[inherent]
impl Fqn for EnumValue {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}

pub struct WeakEnumValue(Weak<Inner>);