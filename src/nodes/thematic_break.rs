use std::fmt::{Display, Formatter};

use serde::Serialize;

/// # Thematic Break
///
/// [Minus](type@crate::lexer::TokenKind::Minus) with length five.
///
/// Example:
/// ```text
/// -----
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Default, Eq)]
pub struct ThematicBreak {}

impl ThematicBreak {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for ThematicBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("-----")
    }
}
