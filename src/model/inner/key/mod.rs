use std::fmt;

use crate::Ident;

mod parse;
pub use parse::Error as ParseError;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Key {
    segments: Vec<Ident>,
}

impl Key {
    pub const SEPARATOR: char = '.';

    pub fn last_segment(&self) -> &Ident {
        self.segments
            .last()
            .expect("must have at least one segment")
    }

    pub fn first_segment(&self) -> &Ident {
        self.segments
            .first()
            .expect("must have at least one segment")
    }

    pub fn segments(&self) -> Segments {
        Segments::from_path(self)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;

        self.first_segment().fmt(f)?;

        for segment in self.segments().skip(1) {
            f.write_char(Self::SEPARATOR)?;
            f.write_str(segment)?;
        }

        Ok(())
    }
}

pub struct Segments<'path> {
    inner: std::slice::Iter<'path, Ident>,
}

impl<'path> Segments<'path> {
    fn from_path(path: &'path Key) -> Self {
        Self {
            inner: path.segments.iter(),
        }
    }
}

impl<'path> Iterator for Segments<'path> {
    type Item = &'path Ident;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
