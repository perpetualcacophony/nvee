pub mod model;
pub use model::{Document, Field, Ident, Item, Key, Table, Value};

pub mod parser;
pub use parser::{Parse, Parser};

pub fn from_str<P: Parse>(s: &str) -> Result<P, P::Err> {
    Parser::new(s).parse()
}
