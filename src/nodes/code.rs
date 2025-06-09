use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Code
///
/// Starts with [Backtick](type@crate::lexer::TokenKind::Backtick) of length < 3.
///
/// [Lang](Code::lang) is every token except [Terminator](type@crate::lexer::TokenKind::Terminator)
/// between [Backtick](type@crate::lexer::TokenKind::Backtick) of length < 3 and
/// [EOL](type@crate::lexer::TokenKind::Eol).
///
/// [Code](Code::code) is every token until [Backtick](type@crate::lexer::TokenKind::Backtick) of
/// length < 3.
///
/// Example:
///
/// ~~~text
/// ```rust
/// let a = 42;
/// ```
/// ~~~
///
/// HTML equivalent:
///
/// ```html
/// <pre><code class="rust">let a = 42;</code></pre>
/// ```

#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Code {
    pub lang: String,
    pub code: String,
}

impl Code {
    pub fn new<S: Into<String>>(lang: S, code: S) -> Self {
        Self {
            lang: lang.into(),
            code: code.into(),
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "```{}\n{}\n```",
            self.lang.replace("```", "\\```").replace("\n", "\\\n"),
            self.code.replace("```", "\\```")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code() {
        let code = Code::new("rust", "let a = 42;");
        assert_eq!(code.to_string(), "```rust\nlet a = 42;\n```");
    }

    #[test]
    fn code_with_backtick() {
        let code = Code::new("rust", "let a = 42;\nlet ```b` = 43;");
        assert_eq!(
            code.to_string(),
            "```rust\nlet a = 42;\nlet \\```b` = 43;\n```"
        );
    }

    #[test]
    fn language_with_eol() {
        let code = Code::new("rust\n\n", "let a = 42;");
        assert_eq!(code.to_string(), "```rust\\\n\\\n\nlet a = 42;\n```");
    }
}
