use serde::Serialize;

/// # Italic
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Underscore](type@crate::lexer::TokenKind::Underscore).
///
/// Example:
///
/// ```text
/// _Italic can contain any token
/// even EOL_
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <i>Italic can contain any token
/// even EOL</i>
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Italic(pub String);

impl Italic {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        Italic(body.into())
    }
}
