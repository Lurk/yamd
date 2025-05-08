use std::fmt::Display;

use serde::Serialize;

/// # Emphasis
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Start](type@crate::lexer::TokenKind::Star).
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
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
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
        let emphasis = Emphasis::new("Emphasis can contain any token even \n");
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
