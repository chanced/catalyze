use std::ops::{Deref, Index};

use ahash::HashMapExt;

use crate::HashMap;

use super::{node, package, FullyQualifiedName, Name};

#[derive(Debug, Clone)]
pub(super) struct Collection<K> {
    list: Vec<K>,
    by_name: HashMap<Name, usize>,
    by_fqn: HashMap<FullyQualifiedName, usize>,
}

impl<K> Collection<K>
where
    K: Copy,
{
    fn from_vec(nodes: Vec<node::Ident<K>>) -> Self {
        let mut list = Vec::with_capacity(nodes.len());
        let mut by_name = HashMap::with_capacity(nodes.len());
        let mut index = HashMap::with_capacity(nodes.len());
        for (idx, node) in nodes.into_iter().enumerate() {
            list.push(node.key);
            by_name.insert(node.name, idx);
            index.insert(node.fqn, idx);
        }
        Self {
            list,
            by_name,
            by_fqn: index,
        }
    }
}
impl<K> Collection<K>
where
    K: slotmap::Key + Copy,
{
    pub(super) fn get_by_name(&self, name: &str) -> Option<K> {
        self.by_name.get(name).copied().map(|idx| self.list[idx])
    }

    pub(super) fn get(&self, index: usize) -> Option<K> {
        self.list.get(index).copied()
    }

    pub(super) fn from_slice(nodes: &[node::Ident<K>]) -> Self {
        let mut list = Vec::with_capacity(nodes.len());
        let mut by_name = HashMap::with_capacity(nodes.len());
        let mut by_fqn = HashMap::with_capacity(nodes.len());
        for (idx, node) in nodes.into_iter().enumerate() {
            list.push(node.key);
            by_name.insert(node.name.clone(), idx);
            by_fqn.insert(node.fqn.clone(), idx);
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
impl Index<usize> for Collection<package::Key> {
    type Output = package::Key;
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
