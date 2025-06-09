use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Italic
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Underscore](type@crate::lexer::TokenKind::Underscore).
///
/// Example:
///
/// ```text
/// _Italic can contain any token
/// even EOL_
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <i>Italic can contain any token
/// even EOL</i>
/// ```
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Italic(pub String);

impl Italic {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        Italic(body.into())
    }
}

impl Display for Italic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "_{}_",
            self.0.replace("_", "\\_").replace("\n\n", "\\\n\n")
        )
    }
}

impl From<String> for Italic {
    fn from(value: String) -> Self {
        Italic(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::Italic;

    #[test]
    fn italic() {
        let italic: Italic = "Italic can contain any token even \n".to_string().into();
        assert_eq!(italic.to_string(), "_Italic can contain any token even \n_");
    }

    #[test]
    fn italic_with_underscore() {
        let italic = Italic::new("Italic can contain any token even \ncan _be_ it");
        assert_eq!(
            italic.to_string(),
            "_Italic can contain any token even \ncan \\_be\\_ it_"
        );
    }

    #[test]
    fn italic_with_terminator() {
        let italic = Italic::new("Italic can contain any token even \n\n can be it");
        assert_eq!(
            italic.to_string(),
            "_Italic can contain any token even \\\n\n can be it_"
        );
    }
}
