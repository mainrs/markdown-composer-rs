//! Contains extension traits that can used to transform text elements into special markdown formats.
//!
//! The module does add support for the following transformations:
//! - [Block quotes](trait.BlockQuote.html)
//! - [Bold](trait.Bold.html)
//! - [Code blocks](trait.CodeBlock.html)
//! - [Inline code](trait.Inline.html)
//! - [Italic](trait.Italic.html)

/// An extension trait for block quote transformation.
pub trait BlockQuote {
    /// Transforms the given text into a block quote.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_composer::transforms::BlockQuote;
    ///
    /// let text = "To quote";
    /// let quoted = text.block_quote();
    /// assert_eq!(quoted, "> To quote");
    /// ```
    ///
    /// # Note
    ///
    /// Multiline quotes should be created using [block_quote_multi_line](trait.BlockQuote.html#tymethod.block_quote_multi_line) function.
    fn block_quote(&self) -> String;

    /// Transforms the given text into a multiline block quote.
    ///
    /// This method does take newlines into account and splits the text after them to create a block quote that spans over multiple lines instead of a single one.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_composer::transforms::BlockQuote;
    ///
    /// let text = "To quote\nor not to quote";
    /// let quoted = text.block_quote_multi_line();
    /// assert_eq!(quoted, "> To quote\n> or not to quote");
    /// ```
    fn block_quote_multi_line(&self) -> String;
}

impl<T> BlockQuote for T
where
    T: AsRef<str>,
{
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

/// An extension trait for bold transformations.
pub trait Bold {
    /// Transforms the given text into its bold version.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_composer::transforms::Bold;
    ///
    /// let text = "To bold";
    /// let bold = text.bold();
    /// assert_eq!(bold, "**To bold**");
    /// ```
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

/// An extension trait for code block transformations.
pub trait CodeBlock {
    /// Transforms the given text into a code block.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_composer::transforms::CodeBlock;
    ///
    /// let text = "print(\"Hello world!\")";
    /// let code_block = text.code_block();
    /// assert_eq!(code_block, "```\nprint(\"Hello world!\")\n```");
    /// ```
    fn code_block(&self) -> String;

    /// Transforms the given text into a code block, allowing to specify the language to use for highlighting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_composer::transforms::CodeBlock;
    ///
    /// let text = "print(\"Hello world!\")";
    /// let code_block = text.code_block_with_language("python");
    /// assert_eq!(code_block, "```python\nprint(\"Hello world!\")\n```");
    /// ```
    fn code_block_with_language<S: AsRef<str>>(&self, language: S) -> String;
}

impl<T> CodeBlock for T
where
    T: AsRef<str>,
{
    fn code_block(&self) -> String {
        format!("```\n{}\n```", self.as_ref())
    }

    fn code_block_with_language<S: AsRef<str>>(&self, language: S) -> String {
        format!("```{}\n{}\n```", language.as_ref(), self.as_ref())
    }
}

/// An extension trait for inline transformations.
pub trait Inline {
    /// Transforms the given text into an inline code block.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_composer::transforms::Inline;
    ///
    /// let text = "Inline text";
    /// let inline = text.inline();
    /// assert_eq!(inline, "`Inline text`");
    /// ```
    fn inline(&self) -> String;
}

impl<T> Inline for T
where
    T: AsRef<str>,
{
    fn inline(&self) -> String {
        format!("`{}`", self.as_ref())
    }
}

/// An extension trait for italic transformations.
pub trait Italic {
    /// Transforms the given text into its italic variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use markdown_composer::transforms::Italic;
    ///
    /// let text = "Italic text";
    /// let inline = text.italic();
    /// assert_eq!(inline, "*Italic text*");
    /// ```
    fn italic(&self) -> String;
}

impl<T> Italic for T
where
    T: AsRef<str>,
{
    fn italic(&self) -> String {
        format!("*{}*", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockQuote, Bold, Inline, Italic};

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
