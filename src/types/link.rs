use crate::traits::{AsFooter, MarkdownElement};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Link {
    pub text: String,
    pub url: String,
    pub footer: bool,
    pub inlined: bool,
}

impl Link {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(
        text: impl Into<String>,
        url: impl Into<String>,
        footer: bool,
        inlined: bool,
    ) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
            footer,
            inlined,
        }
    }
}

impl AsFooter for Link {
    fn as_footer(&self) -> Box<dyn MarkdownElement> {
        Box::new(format!("[{}]: {}", self.text, self.url))
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = if self.footer {
            format!("[{}][{}]", self.text, self.text)
        } else {
            format!("[{}]({})", self.text, self.url)
        };

        if self.inlined {
            write!(f, "{}", text)
        } else {
            writeln!(f, "{}", text)
        }
    }
}
