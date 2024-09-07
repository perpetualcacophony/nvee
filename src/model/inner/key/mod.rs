use std::fmt;

use crate::Ident;

mod parse;
pub use parse::Error as ParseError;

#[cfg(test)]
pub use parse::CONSTRUCTOR;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Key<'id> {
    segments: Vec<Ident<'id>>,
}

impl<'a> Key<'a> {
    pub const SEPARATOR: char = '.';

    pub fn last_segment(&self) -> &Ident<'a> {
        self.segments
            .last()
            .expect("must have at least one segment")
    }

    pub fn first_segment(&self) -> &Ident<'a> {
        self.segments
            .first()
            .expect("must have at least one segment")
    }

    pub fn segments(&self) -> Segments {
        Segments::from_path(self)
    }

    pub fn var_name(&self) -> String {
        let vec: Vec<String> = self.segments().map(|id| id.to_uppercase()).collect();
        vec.join("_")
    }

    pub fn chain(&'a self, next: &'a Self) -> Self {
        Self {
            segments: self.segments().chain(next.segments()).collect(),
        }
    }
}

impl fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.first_segment().fmt(f)?;

        for segment in self.segments().skip(1) {
            Self::SEPARATOR.fmt(f)?;
            segment.fmt(f)?;
        }

        Ok(())
    }
}

pub struct Segments<'path> {
    inner: std::slice::Iter<'path, Ident<'path>>,
}

impl<'path> Segments<'path> {
    fn from_path(path: &'path Key) -> Self {
        Self {
            inner: path.segments.iter(),
        }
    }
}

impl<'path> Iterator for Segments<'path> {
    type Item = Ident<'path>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }
}
