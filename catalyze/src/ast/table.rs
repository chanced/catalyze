use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Index, IndexMut},
    path::Path,
};

use slotmap::SlotMap;

use crate::HashMap;

use super::{access, file::SetPath, FullyQualifiedName, Name};

pub(super) trait AsFqnIndex<K> {
    fn as_fqn_index(&self) -> &HashMap<FullyQualifiedName, K>;
    fn as_fqn_index_mut(&mut self) -> &mut HashMap<FullyQualifiedName, K>;
}

#[derive(Debug, Clone)]
pub(super) struct Table<K, V, I>
where
    K: slotmap::Key,
{
    pub(super) map: SlotMap<K, V>,
    pub(super) index: I,
    pub(super) order: Vec<K>,
}

impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
{
    pub(super) fn len(&self) -> usize {
        self.map.len()
    }
    pub(super) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
    V: Default,
{
    pub(super) fn insert_default(&mut self) -> (K, &mut V) {
        let key = self.map.insert(V::default());
        self.order.push(key);
        (key, &mut self.map[key])
    }
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

impl<K, V, I> Default for Table<K, V, I>
where
    K: slotmap::Key,
    I: Default,
{
    fn default() -> Self {
        Self {
            map: SlotMap::with_key(),
            index: I::default(),
            order: Vec::default(),
        }
    }
}
impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
    V: access::AccessFqn,
    I: Borrow<HashMap<FullyQualifiedName, K>> + BorrowMut<HashMap<FullyQualifiedName, K>>,
{
    pub(super) fn get_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&V> {
        self.index.borrow().get(fqn).map(|key| &self.map[*key])
    }
    pub(super) fn get_mut_by_fqn(&mut self, fqn: &FullyQualifiedName) -> Option<&mut V> {
        self.index.borrow().get(fqn).map(|key| &mut self.map[*key])
    }

    pub(crate) fn get_fqn(&self, key: K) -> &FullyQualifiedName {
        self.get(key).unwrap().fqn()
    }
}

impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
    V: Default + access::AccessKey<Key = K>,
    I: Borrow<HashMap<Name, K>>,
{
    pub(super) fn get_by_path(&self, path: &Path) -> Option<&V> {
        self.index
            .borrow()
            .get(path.as_os_str().to_str()?)
            .map(|key| &self.map[*key])
    }
    pub(super) fn get_by_name(&self, name: &str) -> Option<&V> {
        let path = Path::new(name);
        self.index.borrow().get(name).map(|key| &self.map[*key])
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
    pub(crate) fn keys(&self) -> &[K] {
        &self.order
    }
}

impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
    V: Default + SetPath + access::AccessKey<Key = K>,
    I: Default + BorrowMut<HashMap<Name, K>> + Borrow<HashMap<Name, K>>,
{
    pub(super) fn get_or_insert_key_by_path(&mut self, path: &Path) -> K {
        self.get_or_insert_key_and_value_by_path(path).0
    }

    pub(super) fn get_or_insert_key_and_value_by_path(&mut self, path: &Path) -> (K, &mut V) {
        let path = path.as_os_str().to_str().unwrap();
        let index: &mut HashMap<Name, K> = self.index.borrow_mut();
        let name: Name = path.into();
        let key = *index.entry(name.clone()).or_insert_with(|| {
            let mut value = V::default();
            value.set_path(name.into());
            let key = self.map.insert(value);
            self.order.push(key);
            key
        });
        let value = &mut self.map[key];

        value.set_key(key);

        (key, value)
    }
}
impl<K, V, I> Table<K, V, I>
where
    K: slotmap::Key,
    V: From<FullyQualifiedName> + access::AccessKey<Key = K>,
    I: Default + BorrowMut<HashMap<FullyQualifiedName, K>>,
{
    pub(super) fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            index: Default::default(),
            order: Vec::new(),
        }
    }

    pub(super) fn get_or_insert_key(&mut self, path: FullyQualifiedName) -> K {
        self.get_or_insert_key_and_value(path).0
    }

    pub(super) fn get_or_insert(&mut self, fqn: FullyQualifiedName) -> &mut V {
        self.get_or_insert_key_and_value(fqn).1
    }
    pub(super) fn get_or_insert_key_and_value(&mut self, fqn: FullyQualifiedName) -> (K, &mut V) {
        let key = *self
            .index
            .borrow_mut()
            .entry(fqn.clone())
            .or_insert_with(|| {
                let key = self.map.insert(fqn.into());
                self.order.push(key);
                key
            });
        let value = &mut self.map[key];

        value.set_key(key);

        (key, value)
    }
}

impl<K, V, I> Index<K> for Table<K, V, I>
where
    K: slotmap::Key,
{
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        &self.map[key]
    }
}
impl<K, V, I> IndexMut<K> for Table<K, V, I>
where
    K: slotmap::Key,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        &mut self.map[key]
    }
}
