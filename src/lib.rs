#[cfg(test)]
#[macro_use]
mod test_utils {
    use std::fmt::Debug;

    pub fn parse_str<'p, P: crate::Parse<'p, Err: Debug>>(s: &'p str) -> P {
        P::parse_str(s).unwrap_or_else(|err| {
            use colored::Colorize;

            panic!(
                "{}\noriginal error: {:?}",
                format!(
                    "parsing '{s}' {should_not} fail",
                    should_not = "should not".underline()
                )
                .yellow(),
                err
            )
        })
    }

    fn assert_valid<'p>(s: &'p str, parsed: impl crate::Parse<'p, Err: Debug> + Debug + PartialEq) {
        pretty_assertions::assert_eq!(parsed, parse_str(s))
    }

    pub fn test_valid<'p, Arg, P: crate::Parse<'p, Err: Debug> + Debug + PartialEq>(
        constructor: impl Fn(Arg) -> P,
        iter: impl IntoIterator<Item = (&'static str, Arg)>,
    ) {
        for (s, arg) in iter {
            assert_valid(s, constructor(arg))
        }
    }

    pub fn test_invalid<'p, P: crate::Parse<'p> + Debug>(
        iter: impl IntoIterator<Item = &'static str>,
    ) {
        for s in iter {
            if P::parse_str(s).is_ok() {
                use colored::Colorize;

                panic!(
                    "{}",
                    format!("parsing '{s}' {should} fail", should = "should".underline()).yellow()
                )
            }
        }
    }

    macro_rules! test_invalid {
        ($type:path: $($str:literal),+$(,)?) => {
            #[test]
            fn invalid() {
                crate::test_invalid::<$type>([$($str),+])
            }
        };
    }
}

#[cfg(test)]
use test_utils::{test_invalid, test_valid};

pub mod model;
pub use model::{Document, Field, Ident, Key, Table, Value};

pub mod parser;
pub(crate) use parser::Sealed;
pub use parser::{Parse, Parser};

pub mod set;
pub use set::Set;

#[derive(Debug)]
pub enum Error {
    ParseDocument(model::document::ParseError),
    Io(std::io::Error),
}

impl From<model::document::ParseError> for Error {
    fn from(value: model::document::ParseError) -> Self {
        Self::ParseDocument(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<(), Error> {
    let txt = std::fs::read_to_string(&path)?;
    let mut doc = Document::parse_str(&txt)?;

    if let Some(s) = path.as_ref().file_stem() {
        if let Some(s) = s.to_str() {
            doc.set_basename(s.to_owned());
        }
    }

    #[cfg(not(target_family = "windows"))]
    unsafe {
        doc.set_vars();
    }

    #[cfg(target_family = "windows")]
    doc.set_vars_windows();

    Ok(())
}

pub fn dotnvee() -> Result<(), Error> {
    return from_path(std::path::Path::new(".nvee"));
}
