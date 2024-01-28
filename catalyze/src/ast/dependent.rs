use std::{borrow::Borrow, ops::Deref};

use super::{
    file,
    import::{self, Import},
};

pub(super) struct DependentsInner {
    pub(super) direct: Vec<Inner>,
    pub(super) transitive: Vec<Inner>,
    pub(super) public: Vec<Inner>,
    pub(super) weak: Vec<Inner>,
    pub(super) unusued: Vec<Inner>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(super) struct Inner {
    pub(super) dependent: file::Key,
    pub(super) dependency: file::Key,
}
impl From<import::Inner> for Inner {
    fn from(dep: import::Inner) -> Self {
        Self {
            dependent: dep.dependent,
            dependency: dep.dependency,
        }
    }
}
impl From<Inner> for import::Inner {
    fn from(dep: Inner) -> Self {
        Self {
            dependent: dep.dependent,
            dependency: dep.dependency,
        }
    }
}
pub struct Dependent<'ast> {
    pub is_used: bool,
    pub is_public: bool,
    pub is_weak: bool,
    /// The `File`
    pub dependent: file::File<'ast>,
    /// The [`File`] containing this import.
    pub dependency: file::File<'ast>,
}

impl<'ast> Dependent<'ast> {
    pub fn as_dependency(self) -> Import<'ast> {
        Import {
            dependency: self.dependency,
            dependent: self.dependent,
        }
    }
    #[must_use]
    pub fn is_used(self) -> bool {
        self.is_used
    }
    #[must_use]
    pub fn is_public(self) -> bool {
        self.is_public
    }
    #[must_use]
    pub fn is_weak(self) -> bool {
        self.is_weak
    }
    #[must_use]
    pub fn dependent(self) -> file::File<'ast> {
        self.dependent
    }
    #[must_use]
    pub fn dependency(self) -> file::File<'ast> {
        self.dependency
    }
    #[must_use]
    pub fn as_file(self) -> file::File<'ast> {
        self.dependent
    }
}
impl<'ast> Borrow<file::File<'ast>> for Dependent<'ast> {
    fn borrow(&self) -> &file::File<'ast> {
        &self.dependent
    }
}
impl<'ast> AsRef<file::File<'ast>> for Dependent<'ast> {
    fn as_ref(&self) -> &file::File<'ast> {
        &self.dependent
    }
}
impl<'ast> Deref for Dependent<'ast> {
    type Target = file::File<'ast>;

    fn deref(&self) -> &Self::Target {
        &self.dependent
    }
}
