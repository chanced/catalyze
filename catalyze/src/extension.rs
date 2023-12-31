use std::sync::{Arc, Weak};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension(Arc<Inner>);

pub struct WeakExtension(Weak<Inner>);
