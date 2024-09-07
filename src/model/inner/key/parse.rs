use crate::{model::ident, Parse};

use super::Key;

impl crate::Sealed for Key<'_> {}
impl<'p> Parse<'p> for Key<'p> {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'p>) -> Result<Self, Self::Err> {
        if input.peek_char() == Some(Self::SEPARATOR) {
            return Err(Error::LeadingSeparator);
        }

        let mut segments = vec![input.parse()?];

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
    pub const CONSTRUCTOR: fn(&'static [&'static str]) -> super::Key<'static> =
        |input| super::Key::<'static> {
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
