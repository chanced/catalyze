use std::{iter::Copied, ops::Deref};

use itertools::Itertools;
use snafu::ResultExt;

use crate::error::{self, InvalidIndex};

use super::{
    file::{self, FileKey},
    map_try_into_usize, Ast,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct DependencyInner {
    pub(super) dependency: FileKey,
    pub(super) dependent: FileKey,
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

pub struct Dependencies<'ast> {
    pub(super) inner: DependenciesInner,
    ast: &'ast Ast,
}

impl<'ast> Dependencies<'ast> {
    pub(super) fn new(
        direct: Vec<DependencyInner>,
        public: Vec<i32>,
        weak: Vec<i32>,
        ast: &'ast Ast,
    ) -> Result<Self, error::HydrationFailed> {
        let inner = DependenciesInner::new(direct, public, weak)?;
        Ok(Self { inner, ast })
    }
}

pub struct Iter<'ast> {
    ast: &'ast Ast,
    slice: &'ast [DependencyInner],
    cursor: usize,
    indexes: Option<Copied<std::slice::Iter<'ast, usize>>>,
}
impl<'ast> Iter<'ast> {
    pub(super) fn new(
        direct: &'ast [DependencyInner],
        indexes: Option<&'ast [usize]>,
        ast: &'ast Ast,
    ) -> Self {
        let indexes = indexes.map(|i| i.iter().copied());
        Self {
            ast,
            slice: direct,
            cursor: 0,
            indexes,
        }
    }
    fn next_cursor(&mut self) -> Option<usize> {
        self.indexes.as_mut().map_or_else(
            || {
                let cursor = self.cursor;
                if cursor >= self.slice.len() {
                    return None;
                }
                self.cursor += 1;
                Some(cursor)
            },
            Iterator::next,
        )
    }
}
impl<'ast> Iterator for Iter<'ast> {
    type Item = Dependency<'ast>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = &self.slice[self.next_cursor()?];
        Some(Dependency {
            dependency: file::File::new(next.dependency, self.ast),
            dependent: file::File::new(next.dependent, self.ast),
        })
    }
}
impl<'ast> ExactSizeIterator for Iter<'ast> {
    fn len(&self) -> usize {
        self.indexes
            .as_ref()
            .map_or(self.slice.len() - self.cursor, ExactSizeIterator::len)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct DependenciesInner {
    pub(super) direct: Vec<DependencyInner>,
    pub(super) transitive: Vec<DependencyInner>,
    pub(super) public: Vec<usize>,
    pub(super) weak: Vec<usize>,
    pub(super) unused: Vec<usize>,
}

impl DependenciesInner {
    pub(crate) fn len(&self) -> usize {
        self.direct.len()
    }
    pub(crate) fn new(
        direct: Vec<DependencyInner>,
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
            unused: Vec::new(),
        })
    }
}
