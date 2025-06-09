use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
#[derive(Debug, PartialEq, Clone, Default, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThematicBreak {}

impl ThematicBreak {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for ThematicBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-----")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thematic_break() {
        let thematic_break = ThematicBreak::new();
        assert_eq!(thematic_break.to_string(), "-----");
    }
}
