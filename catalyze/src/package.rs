use crate::{
    ast::{impl_traits, Accessor, Ast, Fqn, FullyQualifiedName, Nodes},
    file::{self},
};

use std::fmt::Debug;

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Inner {
    name: String,
    is_well_known: bool,
    files: Nodes<file::Key>,
    fqn: FullyQualifiedName,
}

pub struct Package<'ast>(Accessor<'ast, Key, Inner>);

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
