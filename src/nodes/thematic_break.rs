use serde::Serialize;

/// # Thematic Break
///
/// [Minus](type@crate::lexer::TokenKind::Minus) with length five.
///
/// Example:
/// ```text
/// -----
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <hr />
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Default, Eq)]
pub struct ThematicBreak {}

impl ThematicBreak {
    pub fn new() -> Self {
        Self {}
    }
}
