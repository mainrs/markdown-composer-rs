use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct CheckmarkItem<'a> {
    pub checked: bool,
    pub text: &'a str,
}

impl<'a> CheckmarkItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(text: &'a str, checked: bool) -> Self {
        Self { text, checked }
    }
}

impl fmt::Display for CheckmarkItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let checkmark = if self.checked { "x" } else { " " };
        write!(f, "[{}] {}", checkmark, self.text)
    }
}

pub trait Strikethrough {
    fn strikethrough(&self) -> String;
}

impl<T> Strikethrough for T
where
    T: AsRef<str>,
{
    fn strikethrough(&self) -> String {
        format!("~{}~", self.as_ref())
    }
}
