use inherent::inherent;
use slotmap::new_key_type;
use std::sync::{Arc, Weak};

use crate::fqn::{Fqn, FullyQualifiedName};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oneof(Arc<Inner>);
#[inherent]
impl Fqn for Oneof {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
pub(crate) struct WeakOneof(Weak<Inner>);

pub(crate) struct Hydrate {}
