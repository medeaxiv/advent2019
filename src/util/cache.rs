use std::collections::{BTreeMap, HashMap};

use ahash::AHashMap;

pub trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V);
}

pub struct NoCache;

impl<K, V> Cache<K, V> for NoCache {
    fn get(&self, _key: &K) -> Option<&V> {
        None
    }

    fn get_mut(&mut self, _key: &K) -> Option<&mut V> {
        None
    }

    fn insert(&mut self, _key: K, _value: V) {
        // Noop
    }
}

impl<K, V> Cache<K, V> for HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Self::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) {
        Self::insert(self, key, value);
    }
}

impl<K, V> Cache<K, V> for AHashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Self::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) {
        Self::insert(self, key, value);
    }
}

impl<K, V> Cache<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Self::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) {
        Self::insert(self, key, value);
    }
}

impl<V> Cache<usize, V> for Vec<Option<V>>
where
    V: Eq,
{
    fn get(&self, key: &usize) -> Option<&V> {
        self.as_slice().get(*key).and_then(|value| value.as_ref())
    }

    fn get_mut(&mut self, key: &usize) -> Option<&mut V> {
        self.as_mut_slice()
            .get_mut(*key)
            .and_then(|value| value.as_mut())
    }

    fn insert(&mut self, key: usize, value: V) {
        self[key] = Some(value);
    }
}
