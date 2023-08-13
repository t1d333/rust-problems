#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    data: HashMap<K, V>,
    cap: usize,
    len: usize,
    key_to_used: BTreeMap<K, usize>,
    keys_priory: BTreeMap<usize, K>,
    timer: usize,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            data: HashMap::new(),
            cap: capacity,
            len: 0,
            timer: 0,
            key_to_used: BTreeMap::new(),
            keys_priory: BTreeMap::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(v) = self.data.get(key) {
            let time = self.key_to_used.insert(key.clone(), self.timer).unwrap();
            self.keys_priory.remove(&time);
            self.keys_priory.insert(self.timer, key.clone());
            self.timer += 1;
            return Some(v);
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut res = None;
        if self.data.contains_key(&key) {
            res = self.data.insert(key.clone(), value);
        } else {
            if self.len == self.cap {
                let t = self.keys_priory.keys().cloned().next().unwrap();
                let k = self.keys_priory.remove(&t).unwrap();
                self.data.remove(&k);
                self.key_to_used.remove(&k);
                self.len -= 1;
            }

            self.data.insert(key.clone(), value);
            self.len += 1;
        }

        if let Some(t) = self.key_to_used.insert(key.clone(), self.timer) {
            self.keys_priory.remove(&t);
        }

        self.keys_priory.insert(self.timer, key);
        self.timer += 1;
        res
    }
}
