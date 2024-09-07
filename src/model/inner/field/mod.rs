use std::fmt;

mod parse;
pub use parse::Error as ParseError;

#[cfg(test)]
pub use parse::CONSTRUCTOR;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Field<'kv> {
    key: crate::Key<'kv>,
    value: crate::Value<'kv>,
}

impl<'kv> Field<'kv> {
    pub fn key(&self) -> &crate::Key<'kv> {
        &self.key
    }

    pub fn value(&self) -> crate::Value<'kv> {
        self.value
    }

    pub fn to_kv(self) -> (crate::Key<'kv>, crate::Value<'kv>) {
        (self.key, self.value)
    }

    pub fn with_parent(&'kv self, key: &'kv crate::Key<'kv>) -> Self {
        Self {
            key: key.chain(&self.key),
            value: self.value,
        }
    }
}

impl fmt::Display for Field<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{key} = {value}", key = self.key(), value = self.value())
    }
}

impl crate::set::KeyEq for Field<'_> {
    fn key(&self) -> &super::Key {
        self.key()
    }
}
