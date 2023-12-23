use std::{
    fmt::Debug,
    sync::{Arc, Weak},
};

use slotmap::new_key_type;

use crate::file::File;

new_key_type! {
    pub(crate) struct PackageKey;
}
pub(crate) struct HydratePakage {
    name: String,
    is_well_known: bool,
    files: Vec<File>,
}

struct Inner {
    name: String,
    is_well_known: bool,
    files: Vec<File>,
}

#[derive(Clone)]
pub struct Package(Arc<Inner>);

impl Debug for Package {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Package")
            .field("name", &self.0.name)
            .field("is_well_known", &self.0.is_well_known)
            .field("files", &self.0.files)
            .finish()
    }
}
impl Package {
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.name
    }
    pub fn name(&self) -> &str {
        self.0.name.as_ref()
    }

    pub fn is_well_known(&self) -> bool {
        self.0.is_well_known
    }

    pub fn files(&self) -> &[File] {
        &self.0.files
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakPackage(Weak<Inner>);
