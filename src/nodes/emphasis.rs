use serde::Serialize;

/// # Emphasis
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Start](type@crate::lexer::TokenKind::Star).
///
/// Example:
///
/// ```text
/// *Emphasis can contain any token
/// even EOL*
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <em>Emphasis can contain any token
/// even EOL</em>
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Emphasis(pub String);

impl Emphasis {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        Emphasis(body.into())
    }
}
