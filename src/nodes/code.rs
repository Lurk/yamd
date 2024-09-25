use serde::Serialize;

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

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
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
