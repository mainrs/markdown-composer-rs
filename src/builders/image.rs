use crate::types::image::Image;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImageBuilder {
    text: String,
    url: String,
    footer: bool,
}

impl ImageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn footer(mut self, value: bool) -> Self {
        self.footer = value;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn build(self) -> Image {
        Image::from(self.text, self.url, self.footer)
    }
}

impl Image {
    pub fn builder() -> ImageBuilder {
        ImageBuilder::new()
    }
}
