use crate::{
    model::{field, table},
    Parse,
};

use super::Item;

#[derive(Debug)]
pub enum Error {
    Field(field::ParseError),
    Table(table::ParseError),
}

impl From<field::ParseError> for Error {
    fn from(value: field::ParseError) -> Self {
        Self::Field(value)
    }
}

impl From<table::ParseError> for Error {
    fn from(value: table::ParseError) -> Self {
        Self::Table(value)
    }
}

impl Parse for Item {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        if input.peek_char() == Some('[') {
            Ok(Self::Table(input.parse()?))
        } else {
            Ok(Self::Field(input.parse()?))
        }
    }
}
