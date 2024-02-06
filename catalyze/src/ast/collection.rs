use std::ops::{Deref, Index};

use ahash::HashMapExt;

use crate::HashMap;

use super::{node, package, FullyQualifiedName, Name};

#[derive(Debug, Clone)]
pub(super) struct Collection<K> {
    list: Vec<K>,
    by_name: HashMap<Name, K>,
    by_fqn: HashMap<FullyQualifiedName, K>,
}

impl<K> Collection<K>
where
    K: Copy,
{
    fn from_vec(nodes: Vec<node::Ident<K>>) -> Self {
        let mut list = Vec::with_capacity(nodes.len());
        let mut by_name = HashMap::with_capacity(nodes.len());
        let mut index = HashMap::with_capacity(nodes.len());
        for node in nodes.into_iter() {
            list.push(node.key);
            by_name.insert(node.name, node.key);
            index.insert(node.fqn, node.key);
        }
        Self {
            list,
            by_name,
            by_fqn: index,
        }
    }

    pub(crate) fn push(&mut self, node: node::Ident<K>) {
        self.by_fqn.entry(node.fqn.clone()).or_insert_with(|| {
            self.list.push(node.key);
            self.by_name.insert(node.name.clone(), node.key);
            node.key
        });
    }
}
impl<K> Collection<K>
where
    K: slotmap::Key + Copy,
{
    pub(super) fn get_by_name(&self, name: &str) -> Option<K> {
        self.by_name.get(name).copied()
    }

    pub(super) fn get(&self, index: usize) -> Option<K> {
        self.list.get(index).copied()
    }

    pub(super) fn from_slice(nodes: &[node::Ident<K>]) -> Self {
        let mut list = Vec::with_capacity(nodes.len());
        let mut by_name = HashMap::with_capacity(nodes.len());
        let mut by_fqn = HashMap::with_capacity(nodes.len());
        for node in nodes.iter() {
            list.push(node.key);
            by_name.insert(node.name.clone(), node.key);
            by_fqn.insert(node.fqn.clone(), node.key);
        }
        Self {
            list,
            by_name,
            by_fqn,
        }
    }
}
impl<K> From<Vec<node::Ident<K>>> for Collection<K>
where
    K: Copy,
{
    fn from(v: Vec<node::Ident<K>>) -> Self {
        Self::from_vec(v)
    }
}
impl Index<usize> for Collection<package::PackageKey> {
    type Output = package::PackageKey;
    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}
impl<K> Default for Collection<K> {
    fn default() -> Self {
        Self {
            list: Vec::default(),
            by_name: HashMap::default(),
            by_fqn: HashMap::default(),
        }
    }
}
impl<K> PartialEq for Collection<K>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list
    }
}
impl<K> Eq for Collection<K> where K: PartialEq {}

impl<K> Deref for Collection<K> {
    type Target = [K];
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
