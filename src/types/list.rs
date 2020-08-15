use crate::traits::MarkdownElement;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

pub type ListItem = Box<dyn MarkdownElement>;

#[derive(Clone, Debug, Default)]
// #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct List {
    pub items: Vec<ListItem>,
    pub ordered: bool,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ordered() -> Self {
        Self {
            ordered: true,
            ..Default::default()
        }
    }

    pub fn ordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            ordered: true,
        }
    }

    pub fn unordered() -> Self {
        Self {
            ordered: false,
            ..Default::default()
        }
    }

    pub fn unordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            ordered: false,
        }
    }

    pub fn add(&mut self, item: ListItem) -> &mut Self {
        self.items.push(item);
        self
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, item) in self.items.iter().enumerate() {
            let checkmark = if self.ordered {
                format!("{}.", idx + 1)
            } else {
                "-".to_string()
            };
            writeln!(f, "{} {}", checkmark, item.render())?;
        }

        Ok(())
    }
}

impl From<&mut List> for List {
    fn from(value: &mut List) -> Self {
        value.clone()
    }
}
