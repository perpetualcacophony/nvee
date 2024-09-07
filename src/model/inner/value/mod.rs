use std::fmt;

mod parse;
pub use parse::Error as ParseError;

#[derive(Hash, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Value<'s> {
    String(&'s str),
    Integer(u64),
}

impl<'s> Value<'s> {
    pub fn var(&self) -> String {
        match self {
            Self::String(s) => (*s).to_owned(),
            Self::Integer(int) => int.to_string(),
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Integer(int) => int.fmt(f),
        }
    }
}
