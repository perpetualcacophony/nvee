use crate::Field;
use crate::Set;
use std::fmt;

mod parse;
pub use parse::Error as ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    name: crate::Key,
    fields: Set<Field>,
}

impl Table {
    pub fn name(&self) -> &crate::Key {
        &self.name
    }

    #[allow(dead_code)] // used in a test macro
    pub(crate) fn new(name: crate::Key, fields: Set<Field>) -> Self {
        Self { name, fields }
    }

    pub fn fields(&self) -> Fields {
        Fields::from_table(self)
    }

    pub fn into_fields(self) -> IntoFields {
        IntoFields {
            table_name: self.name,
            inner: self.fields.into_iter(),
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}]", self.name())?;

        for field in self.fields() {
            writeln!(f, "{}", field)?;
        }

        Ok(())
    }
}

pub struct Fields<'table> {
    table_name: &'table crate::Key,
    inner: <&'table Set<Field> as IntoIterator>::IntoIter,
}

impl<'table> Fields<'table> {
    fn from_table(table: &'table Table) -> Self {
        Self {
            table_name: table.name(),
            inner: table.fields.iter(),
        }
    }
}

impl<'table> Iterator for Fields<'table> {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|field| field.with_parent(self.table_name))
    }
}

pub struct IntoFields {
    table_name: crate::Key,
    inner: <Set<Field> as IntoIterator>::IntoIter,
}

impl Iterator for IntoFields {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|field| field.with_parent(&self.table_name))
    }
}

impl crate::set::KeyEq for Table {
    fn key(&self) -> &super::Key {
        self.name()
    }
}
