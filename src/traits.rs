use dyn_clonable::clonable;
use std::fmt;

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
