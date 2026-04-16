use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
///
/// # Round-trip invariant
///
/// An empty body has no meaningful semantic or visual interpretation. The
/// `Display` impl currently collapses it to the empty string, which means an
/// empty code span is AST-lossy (disappears on round-trip). Constructing
/// `CodeSpan::new("")` is permitted for now but should not be relied on — a
/// future breaking change is expected to reject empty bodies at construction.
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CodeSpan(pub String);

impl CodeSpan {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        CodeSpan(body.into())
    }
}

impl Display for CodeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        write!(
            f,
            "`{}`",
            self.0
                .replace("\\", "\\\\")
                .replace("`", "\\`")
                .replace("\n\n", "\\\n\n")
        )
    }
}

impl From<String> for CodeSpan {
    fn from(value: String) -> Self {
        CodeSpan(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_span() {
        let code_span: CodeSpan = "anything even EOL\ncan be it".to_string().into();
        assert_eq!(code_span.to_string(), "`anything even EOL\ncan be it`");
    }

    #[test]
    fn code_span_with_backtick() {
        let code_span = CodeSpan::new("anything even EOL\ncan `be` it");
        assert_eq!(
            code_span.to_string(),
            "`anything even EOL\ncan \\`be\\` it`"
        );
    }

    #[test]
    fn code_span_with_terminator() {
        let code_span = CodeSpan::new("anything even EOL\n\ncan be it\n");
        assert_eq!(
            code_span.to_string(),
            "`anything even EOL\\\n\ncan be it\n`"
        );
    }
}
