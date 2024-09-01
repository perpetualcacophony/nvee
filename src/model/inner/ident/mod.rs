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

pub struct ParseError {
    index: usize,
    meta: ParseErrorMeta,
}

pub enum ParseErrorMeta {
    IllegalCharacter(char),
    EmptyInput,
}

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
