use std::fmt::Display;

use serde::Serialize;

/// # Image
///
/// Starts with [Bang](type@crate::lexer::TokenKind::Bang) of length 1, and has two required parts.
///
/// [Alt](Image::alt) can contain any character and is surrounded by square brackets
/// [LeftSquareBracket](type@crate::lexer::TokenKind::LeftSquareBracket) and
/// [RightSquareBracket](type@crate::lexer::TokenKind::RightSquareBracket) respectively.
///
/// [Src](Image::src) can contain any character surrounded by parenthesis
/// [LeftParenthesis](type@crate::lexer::TokenKind::LeftParenthesis) and
/// [RightParenthesis](type@crate::lexer::TokenKind::RightParenthesis). Must support any number of
/// nested parenthesis.
///
/// Examples:
///
/// |                yamd                   | html equivalent                                   |
/// |---------------------------------------|---------------------------------------------------|
/// | `![alt](src)`                         | `<img src="src" alt="alt" />`                     |
/// | `![alt [nested squares\]](src)`       | `<img src="src" alt="alt [nested squares]" />`    |
/// | `![alt](src(with nested)paren)`       | `<img src="src(with nested)paren" alt="alt" />`   |
/// | `![alt](src(with(unclosed)nested`     | `<img src="url(with(unclosed" alt="alt" />`       |
///
/// Examples of things that are not valid Image:
///
/// |                yamd                   | html equivalent                               |
/// |---------------------------------------|-----------------------------------------------|
/// | `![alt]`                              | `<p>![alt]</p>`                               |
/// | `![alt](src with unclosed paren`      | `<p>![alt](src with unclosed paren</p>`       |
///

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Image {
    pub alt: String,
    pub src: String,
}

impl Image {
    pub fn new<S: Into<String>>(alt: S, src: S) -> Self {
        Self {
            alt: alt.into(),
            src: src.into(),
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "![{}]({})",
            self.alt.replace("]", "\\]").replace("\n\n", "\\\n\n"),
            self.src.replace("\n\n", "\\\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() {
        let image = Image::new("alt", "src");
        assert_eq!(image.to_string(), "![alt](src)");
    }

    #[test]
    fn image_with_nested_squares() {
        let image = Image::new("alt [nested squares]", "src");
        assert_eq!(image.to_string(), "![alt [nested squares\\]](src)");
    }

    #[test]
    fn image_with_nested_parentheses() {
        let image = Image::new("alt", "src(with nested)parentheses");
        assert_eq!(image.to_string(), "![alt](src(with nested)parentheses)");
    }

    #[test]
    fn image_with_terminator() {
        let image = Image::new("alt\n\n", "src\n\n");
        assert_eq!(image.to_string(), "![alt\\\n\n](src\\\n\n)");
    }
}
