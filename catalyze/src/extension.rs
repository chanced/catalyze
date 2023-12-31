use std::sync::{Arc, Weak};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension(Arc<Inner>);

pub(crate) struct WeakExtension(Weak<Inner>);
