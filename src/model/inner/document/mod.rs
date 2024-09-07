use crate::{Field, Parse, Set, Table};

mod parse;
pub use parse::Error as ParseError;

use super::Key;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Document<'a> {
    basename: Option<String>,
    fields: Set<'a, Field<'a>>,
    tables: Set<'a, Table<'a>>,
}

impl<'a> Document<'a> {
    pub fn set_basename(&mut self, value: String) {
        self.basename = Some(value)
    }

    pub fn fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter()
    }

    pub fn tables(&self) -> impl Iterator<Item = &Table> {
        self.tables.iter()
    }

    pub fn vars(self) -> impl Iterator<Item = (String, String)> {
        let Self {
            basename,
            fields,
            tables,
        } = self;

        let base = basename.as_deref().and_then(|s| Key::parse_str(s).ok());

        let vec: Vec<(_, _)> = {
            let chained = fields
                .into_iter()
                .chain(tables.iter().flat_map(Table::fields));

            if let Some(base) = base {
                chained
                    .map(move |field| {
                        (
                            base.chain(field.key()).to_string(),
                            field.value().to_string(),
                        )
                    })
                    .collect()
            } else {
                chained
                    .map(|field| (field.key().to_string(), field.value().to_string()))
                    .collect()
            }
        };

        vec.into_iter()
    }

    /**
    Sets the environment variables specified in this `Document`.

    If a variable is already set, this method will not override it.
    Instead, this method will return the indexes of all preexisting variables.

    # Safety
    This function has the same safety issues as the underlying function [`std::env::set_var`].

    To summarize:
    * **Windows:** This method is always safe to call, including on multi-threaded programs. Consider using [`set_vars_windows`](Document::set_vars_windows) instead.

    * **Single-threaded programs:** This method is always safe to call.

    * **Multi-threaded programs:** This method is unsafe on multi-threaded programs, as concurrently setting environment variables
      on non-Windows platforms is inherently not thread-safe. However, this method is likely
      safe to call if called and completed before any other threads are spawned, such as in the
      first lines of the `main` function.
    */
    pub unsafe fn set_vars(self) -> Vec<String> {
        let mut already_set = Vec::new();

        for (key, value) in self.vars() {
            if std::env::var(&key).is_ok() {
                already_set.push(key);
            } else {
                std::env::set_var(key, value)
            }
        }

        already_set
    }

    /**
    Windows-exclusive convenience wrapper for [`set_vars`](Document::set_vars).

    This method is safe to call, as [`std::env::set_var`] is always safe to call on Windows.
    */
    pub fn set_vars_windows(self) {
        #[cfg(not(target_family = "windows"))]
        panic!("you can only use this method on windows!");

        #[allow(unreachable_code)]
        unsafe {
            self.set_vars();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;

    use super::{parse, Document};

    use std::env;

    impl Document<'_> {
        fn with_vars(self, f: impl FnOnce()) {
            temp_env::with_vars(
                self.vars().map(|(k, v)| (k, Some(v))).collect::<Vec<_>>(),
                f,
            )
        }
    }

    #[test]
    fn set_vars() {
        let document = Document::parse_str(parse::tests::EXAMPLE).expect("parsing should not fail");

        document.with_vars(|| {
            pretty_assertions::assert_eq!(env::var("DB_URL").as_deref(), Ok("https://example.com"));

            pretty_assertions::assert_eq!(env::var("DB_PORT").as_deref(), Ok("2020"));
        });
    }

    #[test]
    fn vars_basename() {
        let mut document =
            Document::parse_str(parse::tests::EXAMPLE).expect("parsing should not fail");
        document.set_basename("example".to_owned());

        document.with_vars(|| {
            pretty_assertions::assert_eq!(
                env::var("EXAMPLE_DB_URL").as_deref(),
                Ok("https://example.com")
            );

            pretty_assertions::assert_eq!(env::var("EXAMPLE_DB_PORT").as_deref(), Ok("2020"));
        });
    }
}
