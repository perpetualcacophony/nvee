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
            _ => segments.push(input.parse()?),
        }

        loop {
            if input.peek_char() == Some(' ') || input.peek_char().is_none() {
                break;
            }

            input.next_char();
            segments.push(input.parse()?);
        }

        Ok(Self { segments })
    }
}

#[derive(Debug)]
pub struct Error {
    meta: Meta,
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn valid() {
        crate::test_valid(
            |slice| super::Key {
                segments: slice
                    .iter()
                    .copied()
                    .map(crate::model::ident::CONSTRUCTOR)
                    .collect(),
            },
            [
                ("preen", ["preen"].as_slice()),
                ("beep.boop", ["beep", "boop"].as_slice()),
                (
                    "bop2.bip2.3dm.4g_nine",
                    ["bop2", "bip2", "3dm", "4g_nine"].as_slice(),
                ),
            ],
        );
    }

    test_invalid! {
        super::Key: "", " ", ".", "bip.", ".leading",
    }
}
