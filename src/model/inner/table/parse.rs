use std::collections::HashSet as Set;

use crate::{
    model::{field, key},
    Field, Key, Parse,
};

use super::Table;

pub enum Error {
    Name(Option<key::ParseError>),
    ParseField(field::ParseError),
    DuplicateKey(Key),
}

impl From<key::ParseError> for Error {
    fn from(value: key::ParseError) -> Self {
        Self::Name(Some(value))
    }
}

impl From<field::ParseError> for Error {
    fn from(value: field::ParseError) -> Self {
        Self::ParseField(value)
    }
}

impl Error {
    fn name() -> Self {
        Self::Name(None)
    }
}

impl Parse for Table {
    type Err = Error;

    fn parse(input: &mut crate::Parser<'_>) -> Result<Self, Self::Err> {
        for _ in 0..2 {
            if input.next_char() != Some('[') {
                return Err(Error::name());
            }
        }

        let name = input.parse()?;

        let mut fields = Set::new();
        let mut keys = Set::new();

        while let Ok(field) = input.parse::<Field>() {
            if keys.contains(field.key()) {
                return Err(Error::DuplicateKey(field.key().to_owned()));
            }

            keys.insert(field.key().clone());
            fields.insert(field);
        }

        Ok(Self { name, fields })
    }
}
