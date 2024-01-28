use std::{borrow::Borrow, ops::Deref};

use snafu::ResultExt;

use crate::error;

use super::{
    file::{self},
    index::{self, update_and_collect},
    FullyQualifiedName,
};


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct Inner {
    pub(super) dependency: file::Key,
    pub(super) dependent: file::Key,
}

pub struct Import<'ast> {
    /// The imported `File`
    pub dependency: file::File<'ast>,
    /// The [`File`] containing this import.
    pub dependent: file::File<'ast>,
}

impl<'ast> Deref for Import<'ast> {
    type Target = file::File<'ast>;

    fn deref(&self) -> &Self::Target {
        &self.dependency
    }
}

impl<'ast> Import<'ast> {
    #[must_use]
    pub fn is_used(self) -> bool {
        // self.dependency.dependencies().direct().
        todo!()
    }
    #[must_use]
    pub fn is_public(self) -> bool {
        todo!()
    }
    #[must_use]
    pub fn is_weak(self) -> bool {
        todo!()
    }
    /// The imported [`File`]
    #[must_use]
    pub fn import(self) -> file::File<'ast> {
        self.dependency
    }

    /// The [`File`] containing this import.
    #[must_use]
    pub fn imported_by(self) -> file::File<'ast> {
        self.dependent
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct DependenciesInner {
    pub(super) direct: Vec<Inner>,
    pub(super) transitive: Vec<Inner>,
    pub(super) public: Vec<Inner>,
    pub(super) weak: Vec<Inner>,
    pub(super) unusued: Vec<Inner>,
}

fn x(v: impl ExactSizeIterator<Item = i32>) {}
impl DependenciesInner {
    fn new(
        direct: Vec<Inner>,
        dependent: file::Key,
        public: Vec<i32>,
        weak: Vec<i32>,
        files: &file::Table,
        container_fqn: &FullyQualifiedName,
    ) -> Result<Self, error::HydrationFailed> {
        #[rustfmt::skip]
        let weak = index::Iter::new(weak.into_iter())
            .map(|dep|{
                let dep = dep?;
                
            })
            .colklect::<Vec<_>>();

        let public = update_and_collect(public, &mut direct, Inner::mark_public)
            .with_context(|_| error::InvalidIndexCtx {
                fully_qualified_name: container_fqn.clone(),
                index_kind: error::IndexKind::PublicDependency,
            })?;

        let transitive = direct.clone();

        // we can't determine unused yet - need to wait for all files to be hydrated
        Ok(Self {
            direct,
            transitive,
            public,
            weak,
            unusued: Vec::new(),
        })
    }
}
