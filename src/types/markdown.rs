use crate::traits::MarkdownElement;
use crate::types::header::Header;
use crate::types::link::Link;
use crate::types::list::List;
use crate::types::paragraph::Paragraph;
use crate::utils::ToUsize;
use crate::PRELIMINARY_REMARK;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default)]
// #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Markdown {
    pub elements: Vec<Box<dyn MarkdownElement>>,
}

impl<'a> Markdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(elements: Vec<Box<dyn MarkdownElement>>) -> Self {
        Self { elements }
    }

    /// Creates a new markdown file, pre-populating it with the remark notice.
    pub fn with_remark() -> Self {
        let elements = (&*PRELIMINARY_REMARK).clone();
        Self {
            elements: elements.to_vec(),
        }
    }

    pub fn header(&mut self, text: impl Into<String>, level: impl ToUsize) -> &mut Self {
        let header = Header::from(text, level);
        self.elements.push(Box::new(header));
        self
    }

    pub fn header1(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 1usize);
        self
    }

    pub fn header2(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 2usize);
        self
    }

    pub fn header3(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 3usize);
        self
    }

    pub fn header4(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 4usize);
        self
    }

    pub fn header5(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 5usize);
        self
    }

    pub fn header6(&mut self, text: impl Into<String>) -> &mut Self {
        self.header(text, 6usize);
        self
    }

    pub fn list(&mut self, list: List) -> &mut Self {
        self.elements.push(Box::new(list));
        self
    }

    pub fn link(&mut self, link: Link) -> &mut Self {
        self.elements.push(Box::new(link));
        self
    }

    pub fn paragraph(&mut self, text: impl Into<String>) -> &mut Self {
        self.elements.push(Box::new(Paragraph::from(text)));
        self
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Markdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in &self.elements {
            writeln!(f, "{}", element.render())?;
        }

        Ok(())
    }
}
