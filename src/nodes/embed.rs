use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        write!(
            f,
            "{{{{{}|{}}}}}",
            self.kind.replace("|", "\\|"),
            self.args.replace("}", "\\}")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embed() {
        let embed = Embed::new("youtube", "dQw4w9WgXcQ");
        assert_eq!(embed.to_string(), "{{youtube|dQw4w9WgXcQ}}");
    }

    #[test]
    fn embed_with_escaped_parts() {
        let embed = Embed::new("youtube|as", "dQw4w9WgXcQ}}");
        assert_eq!(embed.to_string(), "{{youtube\\|as|dQw4w9WgXcQ\\}\\}}}");
    }
}
