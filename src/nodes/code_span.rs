use std::fmt::Display;

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

impl Display for CodeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{}`",
            self.0.replace("`", "\\`").replace("\n\n", "\\\n\n")
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
        let code_span = CodeSpan::new("anything even EOL\ncan be it");
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
