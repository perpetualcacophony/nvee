use std::fmt;

mod parse;
pub use parse::Error as ParseError;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Field {
    key: crate::Key,
    value: crate::Value,
}

impl Field {
    pub fn key(&self) -> &crate::Key {
        &self.key
    }

    pub fn value(&self) -> &crate::Value {
        &self.value
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{key} = {value}", key = self.key(), value = self.value())
    }
}
