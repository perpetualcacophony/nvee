pub mod model;
pub use model::{Document, Field, Ident, Item, Key, Table, Value};

pub mod parser;
pub(crate) use parser::Sealed;
pub use parser::{Parse, Parser};
