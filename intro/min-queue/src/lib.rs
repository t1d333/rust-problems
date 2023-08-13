#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    s1: VecDeque<(T, T)>,
    s2: VecDeque<(T, T)>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        MinQueue {
            s1: VecDeque::new(),
            s2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        let mut min = val.clone();
        if !self.s1.is_empty() && self.s1.back().unwrap().1 < min {
            min = self.s1.back().unwrap().1.clone();
        }
        self.s1.push_back((val, min));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.s2.is_empty() {
            while !self.s1.is_empty() {
                let el = self.s1.pop_back().unwrap().0;
                let mut min = el.clone();
                if !self.s2.is_empty() {
                    let tmp = self.s2.back().unwrap().1.clone();
                    if tmp < min {
                        min = tmp;
                    }
                }
                self.s2.push_back((el, min));
            }
        }

        if let Some(res) = self.s2.pop_back() {
            Some(res.0)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        if self.s2.is_empty() {
            Some(&self.s1.front().unwrap().0)
        } else {
            Some(&self.s2.back().unwrap().0)
        }
    }

    pub fn min(&self) -> Option<&T> {
        if !self.s1.is_empty() && !self.s2.is_empty() {
            return Some(std::cmp::min(
                &self.s1.back().unwrap().1,
                &self.s2.back().unwrap().1,
            ));
        }

        if !self.s1.is_empty() {
            return Some(&self.s1.back().unwrap().1);
        }

        if !self.s2.is_empty() {
            return Some(&self.s2.back().unwrap().1);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.s1.len() + self.s2.len()
    }

    pub fn is_empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}
