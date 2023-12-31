use std::sync::Arc;

use inherent::inherent;

use crate::fqn::{Fqn, FullyQualifiedName};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method(Arc<Inner>);

#[inherent]
impl Fqn for Method {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
