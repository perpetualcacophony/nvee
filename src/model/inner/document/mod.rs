use crate::{Item, Parse, Set};

mod parse;

use super::Key;

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

    pub fn vars(&self) -> impl Iterator<Item = String> {
        use std::borrow::Cow;

        let format_var =
            |(k, v): (Cow<Key>, &super::Value)| format!("{}={}", k.var_name(), v.var());

        let vec: Vec<String> = if let Some(base) = self.base_key() {
            self.items
                .iter()
                .flat_map(Item::vars)
                .map(|(k, v)| (Cow::Owned(base.chain(&k)), v))
                .map(format_var)
                .collect()
        } else {
            self.items
                .iter()
                .flat_map(Item::vars)
                .map(format_var)
                .collect()
        };

        vec.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;

    use super::{parse, Document};

    #[test]
    fn vars() {
        let document = Document::parse_str(parse::tests::EXAMPLE).expect("parsing should not fail");
        let vars = document.vars().collect::<Vec<_>>();

        assert!(vars.contains(&"DB_URL=https://example.com".to_owned()));
        assert!(vars.contains(&"DB_PORT=2020".to_owned()));
    }

    #[test]
    fn vars_basename() {
        let mut document =
            Document::parse_str(parse::tests::EXAMPLE).expect("parsing should not fail");
        document.set_basename("example".to_owned());
        let vars = document.vars().collect::<Vec<_>>();

        assert!(vars.contains(&"EXAMPLE_DB_URL=https://example.com".to_owned()));
        assert!(vars.contains(&"EXAMPLE_DB_PORT=2020".to_owned()));
    }
}
