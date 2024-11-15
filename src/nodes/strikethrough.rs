use serde::Serialize;

/// # Strikethrough
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Tilde](type@crate::lexer::TokenKind::Tilde) of length 2.
///
/// Example:
///
/// ```text
/// ~~Strikethrough can contain any token
/// even EOL~~
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <s>Strikethrough can contain any token
/// even EOL</s>
/// ```

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Strikethrough(pub String);

impl Strikethrough {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        Strikethrough(body.into())
    }
}
