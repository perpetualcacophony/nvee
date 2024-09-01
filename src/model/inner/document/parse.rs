use crate::{model::item, set::KeyEq, Item, Key, Parse, Set};

use super::Document;

#[derive(Debug)]
pub enum Error {
    DuplicateKey(Key),
    ParseItem(item::ParseError),
}

impl From<item::ParseError> for Error {
    fn from(value: item::ParseError) -> Self {
        Self::ParseItem(value)
    }
}

impl crate::Sealed for Document {}
impl Parse for Document {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        let mut items = Set::new();

        while let Ok(item) = input.parse::<Item>() {
            let key = item.key().to_owned();
            if !items.insert(item) {
                return Err(Error::DuplicateKey(key));
            }
        }

        Ok(Self {
            basename: None,
            items,
        })
    }
}

#[cfg(test)]
pub(super) mod tests {
    use crate::{Document, Item, Set, Table};

    pub const EXAMPLE: &str = include_str!("example.nvee");

    macro_rules! table {
        ([$($name:literal).+] $($($key:literal).+ = $value:expr)+) => {
            {
            let mut fields = crate::Set::new();

            $(
                fields.insert(crate::model::field::CONSTRUCTOR(([$($key),+].as_slice(), $value)));
            )+

            Table::new(
                crate::model::key::CONSTRUCTOR([$($name),+].as_slice()),
                fields,
            )
            }
        };
    }

    #[test]
    fn example() {
        use crate::Value;

        let mut items = Set::new();

        items.insert(Item::Table(table! {
            ["db"]
            "url" = Value::String("https://example.com".to_owned())
            "port" = Value::Integer(2020)
        }));

        pretty_assertions::assert_eq!(
            crate::test_utils::parse_str::<Document>(EXAMPLE),
            Document {
                basename: None,
                items
            }
        )
    }
}
