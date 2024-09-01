use crate::Field;
use crate::Set;
use std::fmt;

mod parse;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn vars(&self) -> impl Iterator<Item = (crate::Key, &crate::Value)> + '_ {
        let base = self.name();
        self.fields()
            .map(|field| (base.chain(field.key()), field.value()))
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

impl crate::set::KeyEq for Table {
    fn key(&self) -> &super::Key {
        self.name()
    }
}
