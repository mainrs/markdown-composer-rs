pub trait Bold {
    fn bold(&self) -> String;
}

impl<T> Bold for T
where
    T: AsRef<str>,
{
    fn bold(&self) -> String {
        format!("**{}**", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::Bold;
    #[test]
    fn test_bold() {
        // &str
        let text = "text";
        assert_eq!("**text**", text.bold());

        // String
        let text = String::from("text");
        assert_eq!(String::from("**text**"), text.bold());
    }
}
