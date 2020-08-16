use crate::traits::{AsFooter, MarkdownElement};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Image {
    pub footer: bool,
    pub text: String,
    pub url: String,
}

impl Image {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(text: impl Into<String>, url: impl Into<String>, footer: bool) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
            footer,
        }
    }
}

impl AsFooter for Image {
    fn as_footer(&self) -> Box<dyn MarkdownElement> {
        Box::new(format!("[{}]: {}", self.text, self.url))
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.footer {
            writeln!(f, "![{}][{}]", self.text, self.text)
        } else {
            writeln!(f, "![{}]({})", self.text, self.url)
        }
    }
}
