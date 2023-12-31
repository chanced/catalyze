use std::sync::{Arc, Weak};

use inherent::inherent;

use crate::fqn::{Fqn, FullyQualifiedName};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service(Arc<Inner>);

#[inherent]
impl Fqn for Service {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}

pub(crate) struct WeakService(Weak<Inner>);

pub(crate) struct Hydrate {}
