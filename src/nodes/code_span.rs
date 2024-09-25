use serde::Serialize;

/// # Code span
///
/// Any characters except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by a
/// [Backtick](type@crate::lexer::TokenKind::Backtick) of length 1.
///
/// Example:
///
/// ```text
/// `anything even EOL
/// can be it`
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <code>anything even EOL
/// can be it</code>
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct CodeSpan(pub String);

impl CodeSpan {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        CodeSpan(body.into())
    }
}
