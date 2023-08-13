#![forbid(unsafe_code)]
use std::rc::Rc;

pub struct PRef<T> {
    data: Rc<T>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Clone for PRef<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

pub struct PStackIter<T> {
    stack: PStack<T>,
}

impl<T> PStackIter<T> {
    pub fn new(stack: PStack<T>) -> Self {
        Self { stack }
    }
}

impl<T> Iterator for PStackIter<T> {
    type Item = PRef<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.data.is_none() {
            None
        } else {
            let (tmp, new) = self.stack.pop().unwrap();
            self.stack = new;
            Some(tmp)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PStack<T> {
    data: Option<PRef<T>>,
    prev: Option<Rc<PStack<T>>>,
    len: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.as_ref().map(|r| PRef {
                    data: r.data.clone(),
                }),
            prev: self.prev.clone(),
            len: self.len,
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            prev: None,
            len: 0,
        }
    }

    pub fn push(&self, value: T) -> Self {
        let mut new = PStack::new();
        new.len = self.len + 1;
        new.data = Some(PRef {
            data: Rc::new(value),
        });

        new.prev = Some(Rc::new(self.clone()));
        new
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        match &self.data {
            Some(data) => {
                let tmp = PRef {
                    data: data.data.clone(),
                };
                let new = match &self.prev {
                    Some(stack) => PStack {
                        data: stack.data.clone(),
                        prev: stack.prev.clone(),
                        len: stack.len(),
                    },
                    None => PStack::new(),
                };

                Some((tmp, new))
            }
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        PStackIter::new(self.clone())
    }
}
