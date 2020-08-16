use dyn_clonable::clonable;
use std::fmt;

/// A [MarkdownElement](trait.MarkdownElement.html) that can be rendered as a
/// footer value.
pub trait AsFooter {
    /// Returns the `MarkdownElement` that can be used to render the footer.
    fn as_footer(&self) -> Box<dyn MarkdownElement>;
}

#[clonable]
pub trait MarkdownElement: Clone + fmt::Debug {
    fn render(&self) -> String;
}

impl<T> MarkdownElement for T
where
    T: Clone + fmt::Debug + fmt::Display,
{
    fn render(&self) -> String {
        format!("{}", self)
    }
}

impl<'a, T: 'a> From<T> for Box<dyn MarkdownElement + 'a>
where
    T: Clone + fmt::Debug + fmt::Display,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
