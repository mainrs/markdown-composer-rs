use crate::traits::MarkdownElement;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// An item inside a markdown list.
pub type ListItem = Box<dyn MarkdownElement>;

/// A markdown list.
#[derive(Clone, Debug, Default)]
// #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct List {
    /// The items of the list.
    pub items: Vec<ListItem>,
    /// `True` if the list should be ordered.
    ///
    /// Ordered lists use numbers instead of dots as the enumeration character.
    pub ordered: bool,
}

impl List {
    /// Creates a new default `List`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty ordered `List`.
    pub fn ordered() -> Self {
        Self {
            ordered: true,
            ..Default::default()
        }
    }

    /// Creates a new ordered `List` with the given items.
    pub fn ordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            ordered: true,
        }
    }

    /// Creates a new empty unordered `List`.
    pub fn unordered() -> Self {
        Self {
            ordered: false,
            ..Default::default()
        }
    }

    /// Creates a new unordered list with the given items.
    pub fn unordered_with(items: Vec<ListItem>) -> Self {
        Self {
            items,
            ordered: false,
        }
    }

    /// Adds an item to the list.
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

// INVESTIGATE: Is this good design? Makes the API a little bit easier to use by
// not having to explicitly clone.
impl From<&mut List> for List {
    fn from(value: &mut List) -> Self {
        value.clone()
    }
}
