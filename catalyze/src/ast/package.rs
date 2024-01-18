use super::{
    access::NodeKeys,
    file::{self, File},
    impl_traits_and_methods, FullyQualifiedName, Resolver,
};

use std::fmt::Debug;

pub const WELL_KNOWN: &str = "google.protobuf";

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommentsInner {
    comments: super::Comments,
    defined_in: file::Key,
}

pub struct Comments<'ast> {
    pub comments: super::Comments,
    pub defined_in: File<'ast>,
}

impl<'ast> Comments<'ast> {
    pub fn defined_in(&self) -> File<'ast> {
        self.defined_in
    }
    pub fn comments(&self) -> &super::Comments {
        &self.comments
    }
}
#[derive(PartialEq, Default, Clone, Debug)]
pub(super) struct Inner {
    key: Key,

    fqn: FullyQualifiedName,
    comments: Vec<CommentsInner>,
    name: Box<str>,
    is_well_known: bool,
    files: Vec<file::Key>,
}

impl Inner {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            key: Key::default(),
            name: name.as_ref().into(),
            is_well_known: name.as_ref() == WELL_KNOWN,
            files: Vec::default(),
            fqn: FullyQualifiedName::from_package_name(name),
            comments: Vec::default(),
        }
    }
    pub(super) fn fqn(&self) -> &FullyQualifiedName {
        &self.fqn
    }
    pub(super) fn add_file(&mut self, file: file::Key) {
        self.files.push(file);
    }
    pub(super) fn add_comments(&mut self, comments: super::Comments, defined_in: file::Key) {
        self.comments.push(CommentsInner {
            comments,
            defined_in,
        });
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::node::Key> {
        self.files.iter().copied().map(Into::into)
    }
}

pub struct Package<'ast>(Resolver<'ast, Key, Inner>);

impl_traits_and_methods!(Package, Key, Inner);

// impl Debug for Package {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         fmt.debug_struct("Package")
//             .field("name", &self.0.name)
//             .field("is_well_known", &self.0.is_well_known)
//             .field("files", &self.0.files)
//             .finish()
//     }
// }
// impl Package {
//     pub fn name(&self) -> &str {
//         self.0.name.as_ref()
//     }

//     pub fn is_well_known(&self) -> bool {
//         self.0.is_well_known
//     }

//     pub fn files(&self) -> &[File] {
//         &self.0.files
//     }
// }
