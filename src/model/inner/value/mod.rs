use std::fmt;

mod parse;
pub use parse::Error as ParseError;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub enum Value {
    String(String),
    Integer(u64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Integer(int) => int.fmt(f),
        }
    }
}
