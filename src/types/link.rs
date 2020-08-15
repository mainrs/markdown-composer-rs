#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Link {
    pub text: String,
    pub url: String,
    pub inlined: bool,
}

impl Link {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(text: impl Into<String>, url: impl Into<String>, inlined: bool) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
            inlined,
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inlined {
            write!(f, "[{}]({})", self.text, self.url)
        } else {
            writeln!(f, "[{}]({})", self.text, self.url)
        }
    }
}
