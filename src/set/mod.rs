use std::{collections::HashSet, hash::Hash};

use crate::Key;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Set<'item, Item: KeyEq> {
    inner: HashSet<HashByKey<'item, Item>>,
}

impl<T: KeyEq> Set<'_, T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: KeyEq> Default for Set<'_, T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<'a, Item> Set<'a, Item>
where
    Item: KeyEq,
{
    pub fn get(&'a self, key: Key<'a>) -> Option<&'a Item> {
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

impl<'set, Item> IntoIterator for &'set Set<'_, Item>
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
    inner: std::collections::hash_set::Iter<'set, HashByKey<'set, Item>>,
}

impl<'set, Item> Iterator for Iter<'set, Item> {
    type Item = &'set Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(HashByKey::item)
    }
}

pub struct IntoIter<'item, Item> {
    inner: std::collections::hash_set::IntoIter<HashByKey<'item, Item>>,
}

impl<Item> Iterator for IntoIter<'_, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(HashByKey::into_item)
    }
}

impl<'item, Item: KeyEq> IntoIterator for Set<'item, Item> {
    type IntoIter = IntoIter<'item, Item>;
    type Item = Item;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

pub trait KeyEq: crate::Sealed {
    fn key(&self) -> &Key;

    fn key_eq(&self, other: &impl KeyEq) -> bool {
        self.key() == other.key()
    }
}

#[derive(Clone, Debug)]
enum HashByKey<'k, T> {
    Item(T),
    Dummy(Key<'k>),
}

impl<T> HashByKey<'_, T> {
    fn item(&self) -> Option<&T> {
        if let Self::Item(t) = self {
            Some(t)
        } else {
            None
        }
    }

    fn into_item(self) -> Option<T> {
        if let Self::Item(t) = self {
            Some(t)
        } else {
            None
        }
    }
}

impl<T> From<T> for HashByKey<'_, T> {
    fn from(value: T) -> Self {
        Self::Item(value)
    }
}

impl<T: KeyEq> crate::Sealed for HashByKey<'_, T> {}
impl<T: KeyEq> KeyEq for HashByKey<'_, T> {
    fn key(&self) -> &crate::Key {
        match self {
            Self::Dummy(key) => key,
            Self::Item(item) => item.key(),
        }
    }
}

impl<T> PartialEq for HashByKey<'_, T>
where
    Self: KeyEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<T> Eq for HashByKey<'_, T> where Self: KeyEq {}

impl<T> Hash for HashByKey<'_, T>
where
    Self: KeyEq,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}
