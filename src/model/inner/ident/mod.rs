use std::ops::Deref;

use crate::Parse;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct Ident(String);

impl Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct ParseError {
    index: usize,
    meta: ParseErrorMeta,
}

#[derive(Debug)]
pub enum ParseErrorMeta {
    IllegalCharacter(char),
    EmptyInput,
}

impl crate::Sealed for Ident {}
impl Parse for Ident {
    type Err = ParseError;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        let mut s = String::new();

        let mut index = 0;

        while let Some(next) = input.peek_char() {
            match next {
                ' ' | '.' => break,
                legal @ ('a'..='z' | '0'..='9' | '_') => {
                    s.push(legal);
                    input.next_char();
                    index += 1;
                }
                illegal => {
                    return Err(ParseError {
                        index: index + 1,
                        meta: ParseErrorMeta::IllegalCharacter(illegal),
                    })
                }
            }
        }

        if s.is_empty() {
            Err(ParseError {
                index: 0,
                meta: ParseErrorMeta::EmptyInput,
            })
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
pub use tests::CONSTRUCTOR;

#[cfg(test)]
mod tests {
    pub const CONSTRUCTOR: fn(&'static str) -> super::Ident = |s| super::Ident(s.to_owned());

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
