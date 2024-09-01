use std::borrow::Cow;

use crate::{Item, Parse, Set};

mod parse;
pub use parse::Error as ParseError;

use super::{Key, Value};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Document {
    basename: Option<String>,
    items: Set<Item>,
}

impl Document {
    pub fn base_key(&self) -> Option<Key> {
        self.basename
            .as_deref()
            .and_then(|s| Key::parse_str(s).ok())
    }

    pub fn set_basename(&mut self, value: String) {
        self.basename = Some(value)
    }

    pub fn vars(&self) -> impl Iterator<Item = (Cow<Key>, &Value)> {
        let vec: Vec<(Cow<Key>, &Value)> = if let Some(base) = self.base_key() {
            self.items
                .iter()
                .flat_map(Item::vars)
                .map(|(k, v)| (Cow::Owned(base.chain(&k)), v))
                .collect()
        } else {
            self.items.iter().flat_map(Item::vars).collect()
        };

        vec.into_iter()
    }

    /**
    Sets the environment variables specified in this `Document`.

    If a variable is already set, this function will not override it.
    Instead, this function will return the indexes of all preexisting variables.

    # Safety
    This function has the same safety issues as the underlying [`std::env::set_var`].

    To summarize:
    * **Windows:** This function is always safe to call, including on multi-threaded programs.. Consider using [`set_vars_windows`](Document::set_vars_windows) instead.

    * **Single-threaded programs:** This function is always safe to call.

    * **Multi-threaded programs:** This function is unsafe on multi-threaded programs, as concurrently setting environment variables
      on non-Windows platforms is inherently not thread-safe. However, this function is likely
      safe to call if called and completed before any other threads are spawned, such as in the
      first lines of the `main` function.
    */
    pub unsafe fn set_vars(&self) -> Vec<String> {
        let mut already_set = Vec::new();

        for (key, value) in self.vars() {
            let key = key.var_name();

            if std::env::var(&key).is_ok() {
                already_set.push(key);
            } else {
                std::env::set_var(key, value.var())
            }
        }

        already_set
    }

    #[cfg(target_family = "windows")]
    /**
    Windows-exclusive convenience function wrapping [`set_vars`](Document::set_vars).

    This function is safe to call, as [`std::env::set_var`] is always safe to call on Windows.
    */
    pub fn set_vars_windows(&self) {
        unsafe { self.set_vars() }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;

    use super::{parse, Document};

    use std::env;

    impl Document {
        fn with_vars(&self, f: impl FnOnce()) {
            temp_env::with_vars(
                self.vars()
                    .map(|(k, v)| (k.var_name(), Some(v.var())))
                    .collect::<Vec<_>>(),
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
