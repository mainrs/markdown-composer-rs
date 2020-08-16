pub trait BlockQuote {
    fn block_quote(&self) -> String;
    fn block_quote_multi_line(&self) -> String;
}

impl<T> BlockQuote for T where T: AsRef<str> {
    fn block_quote(&self) -> String {
        format!("> {}", self.as_ref())
    }

    fn block_quote_multi_line(&self) -> String {
        let mut lines = Vec::new();
        for line in self.as_ref().lines() {
            let quoted = format!("> {}", line);
            lines.push(quoted);
        }
        lines.join("\n")
    }
}

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

pub trait CodeBlock {
    fn code_block(&self) -> String;
    fn code_block_with_language<S: AsRef<str>>(&self, language: S) -> String;
}

impl<T> CodeBlock for T where T: AsRef<str> {
    fn code_block(&self) -> String {
        format!("```\n{}\n```", self.as_ref())
    }

    fn code_block_with_language<S: AsRef<str>>(&self, language: S) -> String {
        format!("```{}\n{}\n```", language.as_ref(), self.as_ref())
    }
}

pub trait Inline {
    fn inline(&self) -> String;
}

impl<T> Inline for T where T: AsRef<str> {
    fn inline(&self) -> String {
        format!("`{}`", self.as_ref())
    }
}

pub trait Italic {
    fn italic(&self) -> String;
}

impl<T> Italic for T where T: AsRef<str> {
    fn italic(&self) -> String {
        format!("*{}*", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::Bold;
    use super::Italic;
    use super::BlockQuote;
    use super::Inline;

    #[test]
    fn test_block_quote_single_line() {
        let text = "This is a single line block quote";
        let expected = "> This is a single line block quote";

        assert_eq!(expected, text.block_quote());
    }

    #[test]
    fn test_block_quote_multi_line() {
        let text = "This is a single line block quote\nThis is the second line";
        let expected = "> This is a single line block quote\n> This is the second line";

        assert_eq!(expected, text.block_quote_multi_line());
    }

    #[test]
    fn test_bold() {
        // &str
        let text = "text";
        assert_eq!("**text**", text.bold());

        // String
        let text = String::from("text");
        assert_eq!(String::from("**text**"), text.bold());
    }

    #[test]
    fn test_inline() {
        let text = "text";
        assert_eq!("`text`", text.inline());

        let text = String::from("text");
        assert_eq!(String::from("`text`"), text.inline());
    }

    #[test]
    fn test_italic() {
        let text = "text";
        assert_eq!("*text*", text.italic());

        let text = String::from("text");
        assert_eq!(String::from("*text*"), text.italic());
    }
}
