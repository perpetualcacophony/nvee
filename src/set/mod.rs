use std::{collections::HashSet, hash::Hash};

use crate::Key;

#[derive(Clone, Debug)]
pub struct Set<Item> {
    inner: HashSet<HashByKey<Item>>,
}

impl<T> Set<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<Item> Set<Item>
where
    Item: KeyEq,
{
    pub fn get(&self, key: Key) -> Option<&Item> {
        self.inner
            .get(&HashByKey::Dummy(key))
            .map(|wrapper| wrapper.item().expect("items in set should be wrappers"))
    }

    pub fn contains(&self, key: Key) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&mut self, item: Item) -> bool {
        self.inner.insert(HashByKey::Item(item))
    }

    pub fn iter(&self) -> Iter<Item> {
        Iter {
            inner: self.inner.iter(),
        }
    }
}

impl<'set, Item> IntoIterator for &'set Set<Item>
where
    Item: KeyEq,
{
    type Item = &'set Item;
    type IntoIter = Iter<'set, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'set, Item> {
    inner: std::collections::hash_set::Iter<'set, HashByKey<Item>>,
}

impl<'set, Item> Iterator for Iter<'set, Item> {
    type Item = &'set Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(HashByKey::item)
    }
}

pub trait KeyEq: crate::Sealed {
    fn key(&self) -> &Key;

    fn key_eq(&self, other: &impl KeyEq) -> bool {
        self.key() == other.key()
    }
}

#[derive(Clone, Debug)]
enum HashByKey<T> {
    Item(T),
    Dummy(Key),
}

impl<T> HashByKey<T> {
    fn item(&self) -> Option<&T> {
        if let Self::Item(t) = self {
            Some(t)
        } else {
            None
        }
    }
}

impl<T> From<T> for HashByKey<T> {
    fn from(value: T) -> Self {
        Self::Item(value)
    }
}

impl<T: KeyEq> crate::Sealed for HashByKey<T> {}
impl<T: KeyEq> KeyEq for HashByKey<T> {
    fn key(&self) -> &crate::Key {
        match self {
            Self::Dummy(key) => key,
            Self::Item(item) => item.key(),
        }
    }
}

impl<T> PartialEq for HashByKey<T>
where
    Self: KeyEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<T> Eq for HashByKey<T> where Self: KeyEq {}

impl<T> Hash for HashByKey<T>
where
    Self: KeyEq,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}
