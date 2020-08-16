use crate::types::link::Link;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkBuilder {
    text: String,
    url: String,
    footer: bool,
    inlined: bool,
}

impl LinkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn footer(mut self, value: bool) -> Self {
        self.footer = value;
        self
    }

    pub fn inlined(mut self) -> Self {
        self.inlined = true;
        self
    }

    pub fn build(self) -> Link {
        Link::from(self.text, self.url, self.footer, self.inlined)
    }
}

impl Link {
    pub fn builder() -> LinkBuilder {
        LinkBuilder::new()
    }
}
