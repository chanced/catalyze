use std::sync::{Arc, Weak};

use crate::file::File;

struct Inner {
    name: String,
    is_well_known: bool,
    files: Vec<File>,
}

pub struct Package(Arc<Inner>);

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

pub(crate) struct WeakPackage(Weak<Inner>);
