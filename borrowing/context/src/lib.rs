#![forbid(unsafe_code)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Context {
    map: HashMap<String, Box<dyn Any>>,
    singletone_map: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            singletone_map: HashMap::new(),
        }
    }

    pub fn insert<K: ToString, Q: Any>(&mut self, key: K, obj: Q) {
        self.map.insert(key.to_string(), Box::new(obj));
    }

    pub fn get<Q: Any>(&self, key: &str) -> &Q {
        self.map.get(key).unwrap().downcast_ref().unwrap()
    }

    pub fn insert_singletone<Q: Any>(&mut self, obj: Q) {
        self.singletone_map.insert(obj.type_id(), Box::new(obj));
    }

    pub fn get_singletone<Q: Any>(&self) -> &Q {
        self.singletone_map
            .get(&TypeId::of::<Q>())
            .unwrap()
            .downcast_ref()
            .unwrap()
    }
}
