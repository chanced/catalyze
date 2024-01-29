use std::ops::{Index, IndexMut};

use slotmap::SlotMap;

use crate::HashMap;

use super::{access, FullyQualifiedName};

#[derive(Debug, Clone)]
pub(super) struct Table<K, V, I = HashMap<FullyQualifiedName, K>>
where
    K: slotmap::Key,
{
    pub(super) map: SlotMap<K, V>,
    pub(super) index: I,
    pub(super) order: Vec<K>,
}

trait WithCapacity {
    fn with_capacity(len: usize) -> Self;
}
impl<K, V> WithCapacity for HashMap<K, V> {
    fn with_capacity(capacity: usize) -> Self {
        ahash::HashMapExt::with_capacity(capacity)
    }
}

impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
    I: Default,
{
    pub(super) fn with_capacity(len: usize) -> Self {
        Self {
            map: SlotMap::with_capacity_and_key(len),
            index: I::default(),
            order: Vec::with_capacity(len),
        }
    }
}

impl<K, V, N> Default for Table<K, V, N>
where
    K: slotmap::Key,
    N: Default,
{
    fn default() -> Self {
        Self {
            map: SlotMap::with_key(),
            index: Default::default(),
            order: Vec::default(),
        }
    }
}
impl<K, V> Table<K, V, HashMap<FullyQualifiedName, K>>
where
    K: slotmap::Key,
    V: access::FullyQualifiedName,
{
    pub(super) fn get_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&V> {
        self.index.get(fqn).map(|key| &self.map[*key])
    }
    pub(super) fn get_mut_by_fqn(&mut self, fqn: &FullyQualifiedName) -> Option<&mut V> {
        self.index.get(fqn).map(|key| &mut self.map[*key])
    }

    pub(crate) fn get_fqn(&self, key: K) -> &FullyQualifiedName {
        self.get(key).unwrap().fqn()
    }
}

impl<K, V, N> Table<K, V, N>
where
    K: slotmap::Key,
{
    pub(crate) fn get(&self, key: K) -> Option<&V> {
        self.map.get(key)
    }
    pub(crate) fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub(crate) fn get_by_index(&self, index: usize) -> Option<&V> {
        self.order.get(index).map(|key| &self.map[*key])
    }
    pub(crate) fn get_mut_by_index(&mut self, index: usize) -> Option<&mut V> {
        self.order.get(index).map(|key| &mut self.map[*key])
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.order.iter().map(move |key| (*key, &self.map[*key]))
    }
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.map.iter_mut()
    }
    pub(crate) fn keys(&self) -> impl '_ + Iterator<Item = K> {
        self.order.iter().copied()
    }
}

impl<K, V> Table<K, V>
where
    K: slotmap::Key,
    V: From<FullyQualifiedName> + access::Key<Key = K>,
{
    pub(super) fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            index: HashMap::default(),
            order: Vec::new(),
        }
    }

    pub(super) fn get_or_insert_key(&mut self, fqn: FullyQualifiedName) -> K {
        self.get_or_insert_key_and_value(fqn).0
    }

    pub(super) fn get_or_insert(&mut self, fqn: FullyQualifiedName) -> &mut V {
        self.get_or_insert_key_and_value(fqn).1
    }
    pub(super) fn get_or_insert_key_and_value(&mut self, fqn: FullyQualifiedName) -> (K, &mut V) {
        let key = *self
            .index
            .entry(fqn.clone())
            .or_insert_with(|| self.map.insert(fqn.into()));
        let value = &mut self.map[key];

        if value.key() == K::default() {
            value.set_key(key);
        }

        (key, value)
    }
}

impl<K, V, N> Index<K> for Table<K, V, N>
where
    K: slotmap::Key,
{
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        &self.map[key]
    }
}
impl<K, V> IndexMut<K> for Table<K, V>
where
    K: slotmap::Key,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        &mut self.map[key]
    }
}
