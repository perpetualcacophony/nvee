use std::num::ParseIntError;

use crate::Parse;

use super::Value;

#[derive(Debug)]
pub enum Error {
    EmptyInput,
    ParseInt(ParseIntError),
    UnclosedString,
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}

impl crate::Sealed for Value<'_> {}
impl<'p> Parse<'p> for Value<'p> {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'p>) -> Result<Self, Self::Err> {
        if input.peek_char() == Some('"') {
            input.next_char();

            let s = input.parse_while(|ch| ch != &'"').unwrap_or_default();
            input.next_char();
            Ok(Self::String(s))
        } else {
            Ok(Self::Integer(input.parse()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    fn string(s: &str) -> Value {
        Value::String(s)
    }

    fn integer(int: u64) -> Value<'static> {
        Value::Integer(int)
    }

    #[test]
    fn valid() {
        crate::test_valid(
            |input| input,
            [
                (r#""preen""#, string("preen")),
                ("105", integer(105)),
                ("999", integer(999)),
            ],
        );
    }

    test_invalid! {
        Value: "", "preen", " "
    }
}
