use ahash::HashMap;

use super::{
    access::{AccessFqn, AccessKey, AccessName, AccessNodeKeys},
    collection::Collection,
    file::{File, FileIdent, FileKey, Files},
    impl_traits_and_methods,
    location::Comments,
    resolve::{Get, Resolver},
    FullyQualifiedName, Name,
};

use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    hash::Hash,
};

pub const WELL_KNOWN: &str = "google.protobuf";

slotmap::new_key_type! {
    pub(super) struct PackageKey;
}

pub(super) type PackageTable = super::table::Table<PackageKey, PackageInner, PackageIndex>;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct PackageIndex {
    pub(super) by_fqn: HashMap<FullyQualifiedName, PackageKey>,
    pub(super) by_name: HashMap<Name, PackageKey>,
}

impl Borrow<HashMap<FullyQualifiedName, PackageKey>> for PackageIndex {
    fn borrow(&self) -> &HashMap<FullyQualifiedName, PackageKey> {
        &self.by_fqn
    }
}
impl BorrowMut<HashMap<FullyQualifiedName, PackageKey>> for PackageIndex {
    fn borrow_mut(&mut self) -> &mut HashMap<FullyQualifiedName, PackageKey> {
        &mut self.by_fqn
    }
}

impl Borrow<HashMap<Name, PackageKey>> for PackageIndex {
    fn borrow(&self) -> &HashMap<Name, PackageKey> {
        &self.by_name
    }
}
impl BorrowMut<HashMap<Name, PackageKey>> for PackageIndex {
    fn borrow_mut(&mut self) -> &mut HashMap<Name, PackageKey> {
        &mut self.by_name
    }
}
pub struct Package<'ast>(pub(super) Resolver<'ast, PackageKey, PackageInner>);

impl_traits_and_methods!(Package, PackageKey, PackageInner);
impl<'ast> Package<'ast> {
    pub fn name(&self) -> &str {
        &self.0.name
    }
    pub fn fqn(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
    pub fn is_well_known(self) -> bool {
        self.0.is_well_known
    }
    pub fn comments(&self) -> impl Iterator<Item = PackageComments> {
        self.0
            .files_with_package_comments
            .iter()
            .copied()
            .map(|key| {
                let file = self.ast().get(key);
                PackageComments {
                    comments: file.package_comments.as_ref().unwrap(),
                    defined_in: (key, self.ast()).into(),
                }
            })
    }
    pub fn files(&'ast self) -> Files<'ast> {
        (&self.0.files, self.ast()).into()
    }
}

impl AccessName for Package<'_> {
    fn name(&self) -> &str {
        &self.0.name
    }
}
impl AccessFqn for Package<'_> {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}

pub struct PackageComments<'ast> {
    pub comments: &'ast Comments,
    pub defined_in: File<'ast>,
}

impl<'ast> PackageComments<'ast> {
    pub fn defined_in(&self) -> File<'ast> {
        self.defined_in
    }
    pub fn comments(&self) -> &Comments {
        &self.comments
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub(super) struct PackageInner {
    pub(super) key: PackageKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) files_with_package_comments: Vec<FileKey>,
    pub(super) name: Name,
    pub(super) is_well_known: bool,
    pub(super) files: Collection<FileKey>,
}
impl AccessFqn for PackageInner {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.fqn
    }
}
impl AccessKey for PackageInner {
    type Key = PackageKey;

    fn key(&self) -> Self::Key {
        self.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}
impl AccessNodeKeys for PackageInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        self.files.iter().copied().map(Into::into)
    }
}
impl PackageInner {
    pub fn new(name: &str) -> Self {
        let fqn = FullyQualifiedName::for_package(name.into());
        Self {
            key: PackageKey::default(),
            name: name.into(),
            is_well_known: name == WELL_KNOWN,
            files: Collection::default(),
            fqn,
            files_with_package_comments: Vec::new(),
        }
    }
    pub(super) fn hydrate(&mut self, name: Name) {
        if !self.name.is_empty() {
            return;
        }
        self.is_well_known = name == WELL_KNOWN;
        self.name = name;
    }
    pub(super) fn add_file(&mut self, file: FileIdent) {
        self.files.push(file);
    }
    pub(super) fn add_file_with_comments(&mut self, file: FileKey) {
        self.files_with_package_comments.push(file);
    }
}
