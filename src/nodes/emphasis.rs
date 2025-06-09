use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Emphasis
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Star](type@crate::lexer::TokenKind::Star).
///
/// Example:
///
/// ```text
/// *Emphasis can contain any token
/// even EOL*
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <em>Emphasis can contain any token
/// even EOL</em>
/// ```
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Emphasis(pub String);

impl Emphasis {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        Emphasis(body.into())
    }
}

impl Display for Emphasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "*{}*",
            self.0.replace("*", "\\*").replace("\n\n", "\\\n\n")
        )
    }
}

impl From<String> for Emphasis {
    fn from(value: String) -> Self {
        Emphasis(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::Emphasis;

    #[test]
    fn emphasis() {
        let emphasis: Emphasis = "Emphasis can contain any token even \n".to_string().into();
        assert_eq!(
            emphasis.to_string(),
            "*Emphasis can contain any token even \n*"
        );
    }

    #[test]
    fn emphasis_with_asterisk() {
        let emphasis = Emphasis::new("Emphasis with *asterisk*");
        assert_eq!(emphasis.to_string(), "*Emphasis with \\*asterisk\\**");
    }

    #[test]
    fn emphasis_with_terminator() {
        let emphasis = Emphasis::new("Emphasis with newline\n\n");
        assert_eq!(emphasis.to_string(), "*Emphasis with newline\\\n\n*");
    }
}
