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
mod tests {
    fn from_str(s: &str) -> super::Ident {
        super::Ident(s.to_owned())
    }

    fn parse_str(s: &str) -> Result<super::Ident, super::ParseError> {
        use crate::Parse;
        super::Ident::parse_str(s)
    }

    #[test]
    fn valid() {
        fn assert_valid(iter: impl IntoIterator<Item = &'static str>) {
            for s in iter {
                assert_eq!(parse_str(s).expect("parsing should not fail"), from_str(s))
            }
        }

        assert_valid(["amber", "beep_boop", "kv2", "2kv", "55555"]);
    }

    #[test]
    #[should_panic]
    fn invalid() {
        fn assert_invalid(iter: impl IntoIterator<Item = &'static str>) {
            for s in iter {
                parse_str(s).unwrap();
            }
        }

        assert_invalid(["", ".", "...", "???"]);
    }
}
