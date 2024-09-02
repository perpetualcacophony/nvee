use crate::{
    model::{field, table},
    Key, Parse, Set,
};

use super::Document;

#[derive(Debug)]
pub enum Error {
    DuplicateKey(Key),
    Field(field::ParseError),
    Table(table::ParseError),
}

impl From<field::ParseError> for Error {
    fn from(value: field::ParseError) -> Self {
        Self::Field(value)
    }
}

impl From<table::ParseError> for Error {
    fn from(value: table::ParseError) -> Self {
        Self::Table(value)
    }
}

impl crate::Sealed for Document {}
impl Parse for Document {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        let mut fields = Set::new();
        let mut tables = Set::new();

        while input.peek_char() != Some('[') {
            fields.insert(input.parse()?);

            while input.parse_char_with(|ch| matches!(ch, '\n')).is_some() {
                // consume newlines
            }
        }

        while input.peek_char().is_some() {
            tables.insert(input.parse()?);

            while input.parse_char_with(|ch| matches!(ch, '\n')).is_some() {
                // mmm tasty newlines
            }
        }

        Ok(Self {
            basename: None,
            fields,
            tables,
        })
    }
}

#[cfg(test)]
pub(super) mod tests {
    use crate::{Document, Set, Table};

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

        let mut tables = Set::new();

        tables.insert(table! {
            ["db"]
            "url" = Value::String("https://example.com".to_owned())
            "port" = Value::Integer(2020)
        });

        pretty_assertions::assert_eq!(
            crate::test_utils::parse_str::<Document>(EXAMPLE),
            Document {
                basename: None,
                tables,
                ..Default::default()
            }
        )
    }
}
