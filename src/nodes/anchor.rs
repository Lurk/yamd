use serde::Serialize;

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
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
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
