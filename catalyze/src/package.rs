use std::{
    fmt::Debug,
    sync::{Arc, Weak},
};

use inherent::inherent;

use crate::{
    file::File,
    fqn::{Fqn, FullyQualifiedName},
    node::Upgrade,
};

pub(crate) struct Hydrate {
    name: String,
    is_well_known: bool,
    files: Vec<File>,
}

#[derive(PartialEq)]
struct Inner {
    name: String,
    is_well_known: bool,
    files: Vec<File>,
    fqn: FullyQualifiedName,
}

#[derive(Clone, PartialEq)]
pub struct Package(Arc<Inner>);

#[inherent]
impl Fqn for Package {
    /// Alias for `fully_qualified_name` - returns the [`FullyQualifiedName`] of
    /// the Package.
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
    /// Alias for `fully_qualified_name` - returns the [`FullyQualifiedName`] of
    /// the Package.
    pub fn fqn(&self) -> &FullyQualifiedName {
        self.fully_qualified_name()
    }
}

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
impl Upgrade for WeakPackage {
    type Target = Package;
    fn upgrade(&self) -> Self::Target {
        Package(self.0.upgrade().unwrap())
    }
}
impl PartialEq<Package> for WeakPackage {
    fn eq(&self, other: &Package) -> bool {
        self == other
    }
}
impl PartialEq<WeakPackage> for Package {
    fn eq(&self, other: &WeakPackage) -> bool {
        let other = other.upgrade();
        self == &other
    }
}
impl PartialEq for WeakPackage {
    fn eq(&self, other: &Self) -> bool {
        let u = self.upgrade();
        u == other.upgrade()
    }
}
