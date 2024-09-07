use crate::Field;
use crate::Set;
use std::fmt;

mod parse;
pub use parse::Error as ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<'a> {
    name: crate::Key<'a>,
    fields: Set<'a, Field<'a>>,
}

impl<'a> Table<'a> {
    pub fn name(&self) -> &crate::Key<'a> {
        &self.name
    }

    #[allow(dead_code)] // used in a test macro
    pub(crate) fn new(name: crate::Key<'a>, fields: Set<'a, Field<'a>>) -> Self {
        Self { name, fields }
    }

    pub fn fields(&self) -> Fields {
        Fields::from_table(self)
    }
}

impl fmt::Display for Table<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}]", self.name())?;

        for field in self.fields() {
            writeln!(f, "{}", field)?;
        }

        Ok(())
    }
}

pub struct Fields<'table> {
    table_name: &'table crate::Key<'table>,
    inner: <&'table Set<'table, Field<'table>> as IntoIterator>::IntoIter,
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
    type Item = Field<'table>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|field| field.with_parent(self.table_name))
    }
}

impl crate::set::KeyEq for Table<'_> {
    fn key(&self) -> &super::Key {
        self.name()
    }
}
