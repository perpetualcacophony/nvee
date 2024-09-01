use crate::{model::ident, Parse};

use super::Key;

impl crate::Sealed for Key {}
impl Parse for Key {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        let mut segments = Vec::new();

        match input.peek_char() {
            Some('_') | None => {
                return Err(Error {
                    meta: Meta::EmptyInput,
                })
            }
            Some(Self::SEPARATOR) => {
                return Err(Error {
                    meta: Meta::LeadingSeparator,
                })
            }
            _ => (),
        }

        loop {
            if input.peek_char() == Some('_') {
                break;
            }

            segments.push(input.parse()?);
            input.next_char();
        }

        Ok(Self { segments })
    }
}

pub struct Error {
    meta: Meta,
}

pub enum Meta {
    EmptyInput,
    ParseIdent(ident::ParseError),
    LeadingSeparator,
}

impl From<ident::ParseError> for Meta {
    fn from(value: ident::ParseError) -> Self {
        Self::ParseIdent(value)
    }
}

impl From<ident::ParseError> for Error {
    fn from(value: ident::ParseError) -> Self {
        Self { meta: value.into() }
    }
}
