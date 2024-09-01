pub enum Item {
    Field(crate::Field),
    Table(crate::Table),
}

impl crate::set::KeyEq for Item {
    fn key(&self) -> &super::Key {
        match self {
            Self::Field(f) => f.key(),
            Self::Table(t) => t.key(),
        }
    }
}

impl crate::Sealed for Item {}

impl Item {
    pub fn vars(
        &self,
    ) -> Box<dyn Iterator<Item = (std::borrow::Cow<super::Key>, &crate::Value)> + '_> {
        match self {
            Self::Field(field) => {
                Box::new([(std::borrow::Cow::Borrowed(field.key()), field.value())].into_iter())
            }
            Self::Table(table) => {
                Box::new(table.vars().map(|(k, v)| (std::borrow::Cow::Owned(k), v)))
            }
        }
    }
}
