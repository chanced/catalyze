use crate::{
    ast::{impl_traits, Accessor, Ast, Fqn, FullyQualifiedName},
    file::{self},
};

use std::fmt::Debug;

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(PartialEq, Default, Clone, Debug)]
pub(crate) struct Inner {
    pub(crate) fqn: FullyQualifiedName,
    pub(crate) name: String,
    pub(crate) is_well_known: bool,
    pub(crate) files: Vec<file::Key>,
    pub(crate) is_hydrated: bool,
}
impl Inner {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            is_well_known: name.as_ref() == Package::WELL_KNOWN,
            files: Vec::default(),
            fqn: FullyQualifiedName::from_package_name(name),
            is_hydrated: false,
        }
    }
}

pub struct Package<'ast>(Accessor<'ast, Key, Inner>);

impl<'ast> Package<'ast> {
    pub const WELL_KNOWN: &'static str = "google.protobuf";
}

impl_traits!(Package, Key, Inner);

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
