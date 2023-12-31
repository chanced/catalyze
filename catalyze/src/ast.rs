use crate::{
    extension::Extension, file::File, fqn::FullyQualifiedName, node::Node, package::Package,
};
use std::collections::HashMap;

mod hydrate;

pub struct Ast {
    files: Vec<File>,
    package_list: Vec<Package>,
    defined_extensions: Vec<Extension>,
    nodes: HashMap<FullyQualifiedName, Node>,
}
