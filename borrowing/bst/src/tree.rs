#![forbid(unsafe_code)]
use std::{borrow::Borrow, cmp, mem::replace};

use crate::node::Node;

pub struct AVLTreeMap<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn len(&self) -> usize {
        match &self.root {
            Some(node) => node.nodes_count,
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let entry = self.get_entry(key);
        if let Some((_k, v)) = entry {
            Some(v)
        } else {
            None
        }
    }

    fn get_entry<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Eq + Ord + ?Sized,
    {
        let mut curr_node = self.root.as_ref();

        while let Some(node) = curr_node {
            let curr_key = node.key();
            match key.cmp(curr_key.borrow()) {
                cmp::Ordering::Less => {
                    curr_node = node.left.as_ref();
                }
                cmp::Ordering::Equal => {
                    return Some((node.key(), node.value()));
                }
                cmp::Ordering::Greater => {
                    curr_node = node.right.as_ref();
                }
            }
        }
        None
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.get(key).is_some()
    }

    fn height(node: &Option<Box<Node<K, V>>>) -> usize {
        match node {
            Some(v) => v.height,
            None => 0,
        }
    }

    fn nodes_count(node: &Option<Box<Node<K, V>>>) -> usize {
        match node {
            Some(v) => v.nodes_count,
            None => 0,
        }
    }

    fn fix_height(node: &mut Box<Node<K, V>>) {
        node.height = std::cmp::max(
            AVLTreeMap::height(&node.right),
            AVLTreeMap::height(&node.left),
        ) + 1;
        node.nodes_count =
            AVLTreeMap::nodes_count(&node.left) + AVLTreeMap::nodes_count(&node.right) + 1;
    }

    fn bfactor(node: &Box<Node<K, V>>) -> i32 {
        let left = AVLTreeMap::height(&node.left) as i32;
        let right = AVLTreeMap::height(&node.right) as i32;
        right - left
    }

    fn rotate_left(node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut node = node;
        let mut tmp = node.right.unwrap();
        node.right = tmp.left;
        tmp.left = Some(node);
        AVLTreeMap::fix_height(tmp.left.as_mut().unwrap());
        AVLTreeMap::fix_height(&mut tmp);
        tmp
    }

    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut tmp = node.left.unwrap();
        node.left = tmp.right;
        tmp.right = Some(node);
        AVLTreeMap::fix_height(tmp.right.as_mut().unwrap());
        AVLTreeMap::fix_height(&mut tmp);
        tmp
    }

    fn balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        AVLTreeMap::fix_height(&mut node);
        let bf = AVLTreeMap::bfactor(&node);

        match bf {
            2 => {
                if AVLTreeMap::bfactor(node.right.as_ref().unwrap()) < 0 {
                    node.right = Some(AVLTreeMap::rotate_right(node.right.take().unwrap()));
                }
                AVLTreeMap::rotate_left(node)
            }

            -2 => {
                if AVLTreeMap::bfactor(node.left.as_ref().unwrap()) > 0 {
                    node.left = Some(AVLTreeMap::rotate_left(node.left.take().unwrap()));
                }
                AVLTreeMap::rotate_right(node)
            }
            _ => node,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut old = None;
        self.root = Some(AVLTreeMap::insert_rec(&mut self.root, key, value, &mut old));
        old
    }

    fn insert_rec(
        node: &mut Option<Box<Node<K, V>>>,
        key: K,
        value: V,
        old: &mut Option<V>,
    ) -> Box<Node<K, V>> {
        let n = node.take();
        match n {
            Some(mut n) => {
                match n.key().cmp(&key) {
                    cmp::Ordering::Greater => {
                        let mut left = n.left.take();
                        n.left = Some(AVLTreeMap::insert_rec(&mut left, key, value, old));
                    }
                    cmp::Ordering::Equal => {
                        old.replace(n.value);
                        n.value = value;
                        return n;
                    }
                    cmp::Ordering::Less => {
                        let mut right = n.right.take();
                        n.right = Some(AVLTreeMap::insert_rec(&mut right, key, value, old));
                    }
                };

                AVLTreeMap::balance(n)
            }
            None => Box::new(Node::new(key, value)),
        }
    }
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let mut deleted = None;
        self.root = AVLTreeMap::remove_rec(&mut self.root, key, &mut deleted);

        if let Some((_k, v)) = deleted {
            Some(v)
        } else {
            None
        }
    }

    fn remove_rec<Q: ?Sized>(
        node: &mut Option<Box<Node<K, V>>>,
        key: &Q,
        ret: &mut Option<(K, V)>,
    ) -> Option<Box<Node<K, V>>>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let node = node.take();
        match node {
            Some(mut node) => {
                match node.key().borrow().cmp(key) {
                    cmp::Ordering::Less => {
                        node.right = AVLTreeMap::remove_rec(&mut node.right, key, ret);
                    }
                    cmp::Ordering::Equal => {
                        let right = node.right.take();

                        if right.is_none() {
                            let _ = replace(ret, Some((node.key, node.value)));
                            return node.left;
                        }

                        node.right =
                            AVLTreeMap::find_and_remove_min(right.unwrap(), &mut node, ret);
                    }
                    cmp::Ordering::Greater => {
                        node.left = AVLTreeMap::remove_rec(&mut node.left, key, ret);
                    }
                };

                Some(AVLTreeMap::balance(node))
            }
            None => None,
        }
    }

    fn find_and_remove_min(
        mut node: Box<Node<K, V>>,
        tmp: &mut Box<Node<K, V>>,
        deleted: &mut Option<(K, V)>,
    ) -> Option<Box<Node<K, V>>> {
        if let Some(left) = node.left {
            node.left = AVLTreeMap::find_and_remove_min(left, tmp, deleted);
            Some(AVLTreeMap::balance(node))
        } else {
            let deleted_k = replace(&mut tmp.key, node.key);
            let deleted_v = replace(&mut tmp.value, node.value);
            deleted.replace((deleted_k, deleted_v));

            let t = node.right;
            node.right = None;
            t
        }
    }

    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        return self.get_entry(key);
    }

    pub fn nth_key_value(&self, k: usize) -> Option<(&K, &V)> {
        if k > self.len() {
            return None;
        }
        let mut item = None;
        AVLTreeMap::find_nth_rec(&self.root, &mut 0, k, &mut item);
        return item;
    }

    fn find_nth_rec<'a>(
        node: &'a Option<Box<Node<K, V>>>,
        cur_idx: &mut usize,
        searched_idx: usize,
        item: &mut Option<(&'a K, &'a V)>,
    ) {
        if let Some(node) = node {
            AVLTreeMap::find_nth_rec(&node.left, cur_idx, searched_idx, item);
            if *cur_idx == searched_idx {
                item.replace((&node.key, &node.value));
            }
            *cur_idx += 1;
            AVLTreeMap::find_nth_rec(&node.right, cur_idx, searched_idx, item);
        } else {
            return;
        }
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut deleted = None;
        self.root = AVLTreeMap::remove_rec(&mut self.root, key, &mut deleted);
        deleted
    }
}
