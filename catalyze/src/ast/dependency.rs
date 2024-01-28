use std::{collections::HashSet, hash::Hash, ops::Deref};

use itertools::Itertools;
use snafu::ResultExt;

use crate::error::{self, InvalidIndex};

use super::{file, map_try_into_usize, reference, FullyQualifiedName};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct Inner {
    pub(super) dependency: file::Key,
    pub(super) dependent: file::Key,
}

pub struct Dependency<'ast> {
    /// The imported `File`
    pub dependency: file::File<'ast>,
    /// The [`File`] containing this import.
    pub dependent: file::File<'ast>,
}

impl<'ast> Deref for Dependency<'ast> {
    type Target = file::File<'ast>;

    fn deref(&self) -> &Self::Target {
        &self.dependency
    }
}

impl<'ast> Dependency<'ast> {
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
    pub(super) public: Vec<usize>,
    pub(super) weak: Vec<usize>,
    pub(super) unusued: Vec<usize>,
}

impl DependenciesInner {
    pub(crate) fn new(
        direct: Vec<Inner>,
        public: Vec<i32>,
        weak: Vec<i32>,
    ) -> Result<Self, error::HydrationFailed> {
        let len = direct.len();
        let check_len = |i: Result<usize, InvalidIndex>| {
            let i = i?;
            if i >= len {
                let index: i32 = i.try_into().unwrap();
                return Err(InvalidIndex {
                    index,
                    backtrace: snafu::Backtrace::capture(),
                });
            }
            Ok(i)
        };
        let weak = weak.into_iter().unique();
        let weak = map_try_into_usize::MapTryIntoUsize::new(weak)
            .map(check_len)
            .collect::<Result<_, InvalidIndex>>()
            .with_context(|_| error::DependencyIndexCtx {
                dependency_kind: error::DependencyKind::Weak,
            })?;

        let public = public.into_iter().unique();
        let public = map_try_into_usize::MapTryIntoUsize::new(public)
            .map(check_len)
            .collect::<Result<_, InvalidIndex>>()
            .with_context(|_| error::DependencyIndexCtx {
                dependency_kind: error::DependencyKind::Public,
            })?;
        let transitive = direct.clone();
        // we cannot determine unused yet - need to wait for all files to be hydrated
        Ok(Self {
            direct,
            transitive,
            public,
            weak,
            unusued: Vec::new(),
        })
    }
}
