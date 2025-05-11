use std::fmt::Display;

use serde::Serialize;

/// # Strikethrough
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Tilde](type@crate::lexer::TokenKind::Tilde) of length 2.
///
/// Example:
///
/// ```text
/// ~~Strikethrough can contain any token
/// even EOL~~
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <s>Strikethrough can contain any token
/// even EOL</s>
/// ```

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Strikethrough(pub String);

impl Strikethrough {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        Strikethrough(body.into())
    }
}

impl Display for Strikethrough {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "~~{}~~",
            self.0.replace("~", "\\~").replace("\n\n", "\\\n\n")
        )
    }
}

impl From<String> for Strikethrough {
    fn from(value: String) -> Self {
        Strikethrough(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::Strikethrough;

    #[test]
    fn strikethrough() {
        let strikethrough: Strikethrough = "Strikethrough can contain any token even \n"
            .to_string()
            .into();
        assert_eq!(
            strikethrough.to_string(),
            "~~Strikethrough can contain any token even \n~~"
        );
    }

    #[test]
    fn strikethrough_with_tilde() {
        let strikethrough = Strikethrough::new("Strikethrough can contain any token even ~~");
        assert_eq!(
            strikethrough.to_string(),
            "~~Strikethrough can contain any token even \\~\\~~~"
        );
    }

    #[test]
    fn strikethrough_with_terminator() {
        let strikethrough = Strikethrough::new("Strikethrough with terminator\n\n");
        assert_eq!(
            strikethrough.to_string(),
            "~~Strikethrough with terminator\\\n\n~~"
        );
    }
}
