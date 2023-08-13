#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct Node<K, V> {
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub key: K,
    pub value: V,
    pub height: usize,
    pub nodes_count: usize,
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            key,
            value,
            height: 1,
            nodes_count: 1,
        }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}
