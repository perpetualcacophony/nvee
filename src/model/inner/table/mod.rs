use crate::Field;
use std::{collections::HashSet as Set, fmt};

mod parse;

pub struct Table {
    name: crate::Key,
    fields: Set<Field>,
}

impl Table {
    pub fn name(&self) -> &crate::Key {
        &self.name
    }

    pub fn fields(&self) -> Fields {
        Fields::from_table(self)
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
    inner: <&'table Set<Field> as IntoIterator>::IntoIter,
}

impl<'table> Fields<'table> {
    fn from_table(table: &'table Table) -> Self {
        Self {
            inner: table.fields.iter(),
        }
    }
}

impl<'table> Iterator for Fields<'table> {
    type Item = &'table Field;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
