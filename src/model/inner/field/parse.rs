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

impl crate::Sealed for Field {}
impl Parse for Field {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        let key = input.parse()?;
        input.parse::<Separator>().map_err(|_| Error::Separator)?;
        let value = input.parse()?;

        Ok(Self { key, value })
    }
}

struct Separator;

impl crate::Sealed for Separator {}
impl Parse for Separator {
    type Err = ();

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        if input.next_char() != Some(' ') {
            Err(())
        } else if input.next_char() != Some('=') {
            Err(())
        } else if input.next_char() != Some(' ') {
            Err(())
        } else {
            Ok(Self)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn valid() {
        crate::test_valid(
            |(key, value)| super::Field {
                key: crate::model::key::CONSTRUCTOR(key),
                value,
            },
            [
                ("preen = 100", (["preen"].as_slice(), Value::Integer(100))),
                (
                    r#"beep.boop = "top""#,
                    (["beep", "boop"].as_slice(), Value::String("top".to_owned())),
                ),
            ],
        );
    }

    test_invalid! {
        super::Field: "", ".", " ", "...", "???", " = ", "beep. = 100", "mop=340"
    }
}
