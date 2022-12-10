use {
    serde::{Deserialize, Serialize},
    std::{iter, vec::IntoIter},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NonEmptyVec<T> {
    first: T,
    list: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub const fn new(first: T) -> Self {
        Self {
            first,
            list: Vec::new(),
        }
    }

    pub const fn first(&self) -> &T {
        &self.first
    }

    pub fn last(&self) -> &T {
        &self.list.last().unwrap_or(&self.first)
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut vec = vec![self.first];
        vec.extend(self.list);
        vec
    }

    pub fn push(mut self, next: T) -> Self {
        self.list.push(next);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.first).chain(&self.list)
    }

    pub fn len(&self) -> usize {
        1 + self.list.len()
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;
    type IntoIter = iter::Chain<iter::Once<T>, IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.first).chain(self.list.into_iter())
    }
}

impl<T> Extend<T> for NonEmptyVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.list.extend(iter)
    }
}
