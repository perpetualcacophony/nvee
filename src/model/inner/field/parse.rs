use crate::{
    model::{key, value},
    Parse,
};

use super::Field;

#[derive(Debug)]
pub enum Error {
    Key(key::ParseError),
    Separator,
    Value(value::ParseError),
}

impl From<key::ParseError> for Error {
    fn from(value: key::ParseError) -> Self {
        Self::Key(value)
    }
}

impl From<value::ParseError> for Error {
    fn from(value: value::ParseError) -> Self {
        Self::Value(value)
    }
}

impl crate::Sealed for Field<'_> {}
impl<'p: 'kv, 'kv> Parse<'p> for Field<'kv> {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'p>) -> Result<Self, Self::Err> {
        let key = input.parse()?;
        input.parse::<Separator>()?;
        let value = input.parse()?;

        Ok(Self { key, value })
    }
}

struct Separator;

impl crate::Sealed for Separator {}
impl<'p> Parse<'p> for Separator {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'p>) -> Result<Self, Self::Err> {
        (input.parse_char(' ') && input.parse_char('=') && input.parse_char(' '))
            .then_some(Self)
            .ok_or(Error::Separator)
    }
}

#[cfg(test)]
pub use tests::CONSTRUCTOR;

#[cfg(test)]
mod tests {
    use crate::Value;

    pub const CONSTRUCTOR: fn((&'static [&'static str], Value<'static>)) -> super::Field<'static> =
        |(key, value)| super::Field {
            key: crate::model::key::CONSTRUCTOR(key),
            value,
        };

    #[test]
    fn valid() {
        crate::test_valid(
            CONSTRUCTOR,
            [
                ("preen = 100", (["preen"].as_slice(), Value::Integer(100))),
                (
                    r#"beep.boop = "top""#,
                    (["beep", "boop"].as_slice(), Value::String("top")),
                ),
            ],
        );
    }

    test_invalid! {
        super::Field: "", ".", " ", "...", "???", " = ", "beep. = 100", "mop=340"
    }
}
