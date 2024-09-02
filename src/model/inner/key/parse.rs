use crate::{model::ident, Parse};

use super::Key;

impl crate::Sealed for Key {}
impl Parse for Key {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        if input.peek_char() == Some(Self::SEPARATOR) {
            return Err(Error::LeadingSeparator);
        }

        let mut segments = vec![input.parse().map_err(|_| Error::EmptyInput)?];

        loop {
            if !input.parse_char(Self::SEPARATOR) {
                break;
            } else {
                segments.push(input.parse()?);
            }
        }

        Ok(Self { segments })
    }
}

#[derive(Debug)]
pub enum Error {
    EmptyInput,
    Ident,
    LeadingSeparator,
}

impl From<ident::ParseError> for Error {
    fn from(_: ident::ParseError) -> Self {
        Self::Ident
    }
}

#[cfg(test)]
pub use tests::CONSTRUCTOR;

#[cfg(test)]
mod tests {
    pub const CONSTRUCTOR: fn(&'static [&'static str]) -> super::Key = |input| super::Key {
        segments: input
            .iter()
            .copied()
            .map(crate::model::ident::CONSTRUCTOR)
            .collect(),
    };

    #[test]
    fn valid() {
        crate::test_valid(
            CONSTRUCTOR,
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
