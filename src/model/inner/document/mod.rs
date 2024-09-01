use crate::{Item, Parse, Set};

use super::Key;

pub struct Document {
    filename: Option<String>,
    items: Set<Item>,
}

impl Document {
    pub fn base_key(&self) -> Option<Key> {
        self.filename
            .as_deref()
            .and_then(|s| Key::parse_str(s).ok())
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
