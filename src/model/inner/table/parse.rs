use crate::Set;

use crate::{
    model::{field, key},
    Field, Key, Parse,
};

use super::Table;

#[derive(Debug)]
pub enum Error {
    Name(Option<key::ParseError>),
    ParseField(field::ParseError),
    DuplicateKey(Key),
}

impl From<key::ParseError> for Error {
    fn from(value: key::ParseError) -> Self {
        Self::Name(Some(value))
    }
}

impl From<field::ParseError> for Error {
    fn from(value: field::ParseError) -> Self {
        Self::ParseField(value)
    }
}

impl Error {
    fn name() -> Self {
        Self::Name(None)
    }
}

impl crate::Sealed for Table {}
impl Parse for Table {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        if input.next_char() != Some('[') {
            return Err(Error::name());
        }

        let name = input.parse()?;

        if input.next_char() != Some(']') {
            return Err(Error::name());
        }

        input.next_char();

        let mut fields = Set::new();

        while let Ok(field) = input.parse::<Field>() {
            let key = field.key().to_owned();
            if !fields.insert(field) {
                return Err(Error::DuplicateKey(key));
            }
            input.next_char();
        }

        Ok(Self { name, fields })
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use crate::Value;

    macro_rules! construct {
        ([$($name:literal).+] $($($key:literal).+ = $value:expr)+) => {
            {
            let mut fields = super::Set::new();

            $(
                fields.insert(crate::model::field::CONSTRUCTOR(([$($key),+].as_slice(), $value)));
            )+

            Table {
                name: crate::model::key::CONSTRUCTOR([$($name),+].as_slice()),
                fields,
            }
            }
        };
    }

    #[test]
    fn valid() {
        crate::test_valid(
            |input: Table| input,
            [
                (
                    "[mongodb]\nusername = \"kate\"\nport = 999",
                    construct! {
                        ["mongodb"]
                        "username" = Value::String("kate".to_owned())
                        "port" = Value::Integer(999)
                    },
                ),
                (
                    "[mongo.db] username = \"kate\" port.alt = 999",
                    construct! {
                        ["mongo"."db"]
                        "username" = Value::String("kate".to_owned())
                        "port"."alt" = Value::Integer(999)
                    },
                ),
            ],
        );
    }

    test_invalid! {
        super::Key: "", " ", ".", "bip.", ".leading",
    }
}
