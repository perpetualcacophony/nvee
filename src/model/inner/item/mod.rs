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
