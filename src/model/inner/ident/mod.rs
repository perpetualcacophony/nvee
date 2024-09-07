use std::ops::Deref;

use crate::Parse;

pub const CHAR_LEGAL: fn(char) -> bool = |ch| matches!(ch, 'a'..='z' | '0'..='9' | '_');

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
pub struct Ident<'s>(&'s str);

impl<'s> Ident<'s> {
    pub fn into_owned(self) -> IdentOwned {
        IdentOwned(self.0.to_owned())
    }
}

impl Deref for Ident<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug)]
pub struct ParseError;

impl crate::Sealed for Ident<'_> {}
impl<'p> Parse<'p> for Ident<'p> {
    type Err = ParseError;

    fn parse(input: &mut crate::Parser<'p>) -> Result<Self, Self::Err> {
        input
            .parse_while(|ch| matches!(ch, 'a'..='z' | '0'..='9' | '_'))
            .map(Self)
            .ok_or(ParseError)
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct IdentOwned(String);

impl IdentOwned {
    pub fn as_ref(&self) -> Ident {
        Ident(&self.0)
    }
}

#[cfg(test)]
pub use tests::CONSTRUCTOR;

#[cfg(test)]
mod tests {
    pub const CONSTRUCTOR: fn(&'static str) -> super::Ident = super::Ident;

    #[test]
    fn valid() {
        fn test_valid(iter: impl IntoIterator<Item = &'static str>) {
            crate::test_valid(CONSTRUCTOR, iter.into_iter().map(|s| (s, s)))
        }

        test_valid(["amber", "beep_boop", "kv2", "2kv", "55555"]);
    }

    test_invalid! {
        super::Ident: "", ".", " ", "...", "???"
    }
}
