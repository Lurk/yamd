use std::fmt::Display;

use serde::Serialize;

/// # Embed
///
/// Starts with [LeftCurlyBrace](type@crate::lexer::TokenKind::LeftCurlyBrace) of length 2.
///
/// [Kind](Embed::kind) every token except [Terminator](type@crate::lexer::TokenKind::Terminator)
/// until [Pipe](type@crate::lexer::TokenKind::Pipe).
///
/// [Args](Embed::args) every token except [Terminator](type@crate::lexer::TokenKind::Terminator)
/// until [LeftCurlyBrace](type@crate::lexer::TokenKind::LeftCurlyBrace) of length 2.
///
/// Examples:
///
/// ```text
/// {{youtube|dQw4w9WgXcQ}}
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <iframe class="youtube" src="https://www.youtube.com/embed/dQw4w9WgXcQ"></iframe>
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Embed {
    pub kind: String,
    pub args: String,
}

impl Embed {
    pub fn new<K: Into<String>, A: Into<String>>(kind: K, args: A) -> Self {
        Self {
            kind: kind.into(),
            args: args.into(),
        }
    }
}

impl Display for Embed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{{{}|{}}}}}", self.kind, self.args)
    }
}
