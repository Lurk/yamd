use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Anchor
///
/// Anchor has two required parts.
///
/// [Text](Anchor::text) can contain any character and is surrounded by square brackets
/// [LeftSquareBracket](type@crate::lexer::TokenKind::LeftSquareBracket) and
/// [RightSquareBracket](type@crate::lexer::TokenKind::RightSquareBracket) respectively.
///
/// [URL](Anchor::url) can contain any character surrounded by parenthesis
/// [LeftParenthesis](type@crate::lexer::TokenKind::LeftParenthesis) and
/// [RightParenthesis](type@crate::lexer::TokenKind::RightParenthesis). Must support any number of
/// nested parenthesis.
///
/// Examples:
///
/// |                yamd                   | html equivalent                               |
/// |---------------------------------------|-----------------------------------------------|
/// | `[link](url)`                         | `<a href="url">link</a>`                      |
/// | `[link [nested squares\]](url)`       | `<a href="url">link [nested squares]</a>`     |
/// | `[link](url(with nested)paren)`       | `<a href="url(with nested)paren>link</a>`     |
/// | `[link](url(with(unclosed)nested`     | `<a href="url(with(unclosed">link</a>`        |
///
/// Examples of things that are not valid Anchor:
///
/// |                yamd                   | html equivalent                               |
/// |---------------------------------------|-----------------------------------------------|
/// | `[link]`                              | `<p>[link]</p>`                               |
/// | `[link](url with unclosed paren`      | `<p>[link](url with unclosed paren</p>`       |
///
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Anchor {
    pub text: String,
    pub url: String,
}

impl Anchor {
    pub fn new<S: Into<String>>(text: S, url: S) -> Self {
        Anchor {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]({})",
            self.text
                .replace("[", "\\[")
                .replace("]", "\\]")
                .replace("\n\n", "\\\n\n"),
            self.url
                .replace("(", "\\(")
                .replace(")", "\\)")
                .replace("\n\n", "\\\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::Anchor;

    #[test]
    fn anchor() {
        let anchor = Anchor::new("link", "url");
        assert_eq!(anchor.to_string(), "[link](url)");
    }

    #[test]
    fn anchor_with_nested_squares() {
        let anchor = Anchor::new("link [nested squares]", "url");
        assert_eq!(anchor.to_string(), "[link \\[nested squares\\]](url)");
    }

    #[test]
    fn anchor_with_nested_parentheses() {
        let anchor = Anchor::new("link", "url(with nested)parentheses");
        assert_eq!(
            anchor.to_string(),
            "[link](url\\(with nested\\)parentheses)"
        );
    }

    #[test]
    fn anchor_with_terminator() {
        let anchor = Anchor::new("link\n\n", "url\n\n");
        assert_eq!(anchor.to_string(), "[link\\\n\n](url\\\n\n)");
    }
}
