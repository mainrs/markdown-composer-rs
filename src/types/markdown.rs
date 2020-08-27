use crate::{
    traits::{AsFooter, MarkdownElement},
    types::{header::Header, link::Link, list::List, paragraph::Paragraph},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;
use tousize::ToUsize;

#[derive(Default)]
// #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Markdown {
    pub elements: Vec<Box<dyn MarkdownElement>>,
    pub footers: Vec<Box<dyn MarkdownElement>>,
}

impl<'a> Markdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(
        elements: Vec<Box<dyn MarkdownElement>>,
        footers: Vec<Box<dyn MarkdownElement>>,
    ) -> Self {
        Self { elements, footers }
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
        if link.footer {
            self.footers.push(link.as_footer());
        }
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

        for footer in &self.footers {
            writeln!(f, "{}", footer.render())?;
        }

        Ok(())
    }
}
