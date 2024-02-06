use super::{
    access::{AccessName, AccessNodeKeys},
    file::{self, File},
    impl_traits_and_methods, location,
    resolve::Resolver,
    FullyQualifiedName, Name,
};

use std::fmt::Debug;

pub const WELL_KNOWN: &str = "google.protobuf";

slotmap::new_key_type! {
    pub(super) struct PackageKey;
}

pub(super) type Table = super::table::Table<PackageKey, PackageInner>;

pub struct Package<'ast>(pub(super) Resolver<'ast, PackageKey, PackageInner>);

impl_traits_and_methods!(Package, PackageKey, PackageInner);
impl<'ast> Package<'ast> {
    pub fn name(&self) -> &str {
        &self.0.name
    }
    pub fn is_well_known(self) -> bool {
        self.0.is_well_known
    }
}

impl AccessName for Package<'_> {
    fn name(&self) -> &str {
        &self.0.name
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommentsInner {
    comments: location::Comments,
    defined_in: file::FileKey,
}

pub struct Comments<'ast> {
    pub comments: location::Comments,
    pub defined_in: File<'ast>,
}

impl<'ast> Comments<'ast> {
    pub fn defined_in(&self) -> File<'ast> {
        self.defined_in
    }
    pub fn comments(&self) -> &location::Comments {
        &self.comments
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub(super) struct PackageInner {
    pub(super) key: PackageKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) comments: Vec<CommentsInner>,
    pub(super) name: Name,
    pub(super) is_well_known: bool,
    pub(super) files: Vec<file::FileKey>,
}

impl PackageInner {
    pub fn new(name: &str) -> Self {
        let fqn = FullyQualifiedName::for_package(name.into());
        Self {
            key: PackageKey::default(),
            name: name.into(),
            is_well_known: name == WELL_KNOWN,
            files: Vec::default(),
            fqn,
            comments: Vec::default(),
        }
    }
    pub(super) fn hydrate(&mut self, name: Name) {
        if !self.name.is_empty() {
            return;
        }
        self.is_well_known = name == WELL_KNOWN;
        self.name = name;
    }
    pub(super) fn add_file(&mut self, file: file::FileKey) {
        self.files.push(file);
    }
    pub(super) fn add_comments(&mut self, comments: location::Comments, defined_in: file::FileKey) {
        self.comments.push(CommentsInner {
            comments,
            defined_in,
        });
    }
}

impl AccessNodeKeys for PackageInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        self.files.iter().copied().map(Into::into)
    }
}
