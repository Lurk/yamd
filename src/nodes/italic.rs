use std::fmt::{Display, Formatter};

use serde::Serialize;

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
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Italic(pub String);

impl Italic {
    pub fn new<Body: Into<String>>(body: Body) -> Self {
        Italic(body.into())
    }
}

impl Display for Italic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "_{}_", self.0)
    }
}
