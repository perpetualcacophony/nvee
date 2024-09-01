use std::num::ParseIntError;

use crate::Parse;

use super::Value;

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

impl crate::Sealed for Value {}
impl Parse for Value {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        if input.peek_char() == Some('"') {
            input.next_char();

            let mut s = String::new();

            loop {
                match input.next_char() {
                    Some('"') => break,
                    Some(other) => s.push(other),
                    _ => return Err(Error::UnclosedString),
                }
            }

            Ok(Self::String(s))
        } else {
            Ok(Self::Integer(input.parse()?))
        }
    }
}
