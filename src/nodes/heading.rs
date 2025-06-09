use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Anchor;

#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum HeadingNodes {
    Text(String),
    Anchor(Anchor),
}

impl From<String> for HeadingNodes {
    fn from(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<Anchor> for HeadingNodes {
    fn from(anchor: Anchor) -> Self {
        Self::Anchor(anchor)
    }
}

impl Display for HeadingNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadingNodes::Text(text) => {
                write!(f, "{}", text.replace("\n\n", "\\\n\n").replace("#", "\\#"))
            }
            HeadingNodes::Anchor(anchor) => write!(f, "{}", anchor),
        }
    }
}

/// # Heading
///
/// Starts with [Hash](type@crate::lexer::TokenKind::Hash) of length < 7, followed by
/// [Space](type@crate::lexer::TokenKind::Space).
///
/// [Level](Heading::level) is determined by the amount of [Hash](type@crate::lexer::TokenKind::Hash)'es
/// before [Space](type@crate::lexer::TokenKind::Space).
///
/// [Body](Heading::body) can contain one or more:
///
/// - [Anchor]
/// - [String]
///
/// Example:
///
/// ```text
/// ### Header can contain an [anchor](#) or regular text.
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <h3>Header can contain an <a href="#">anchor</a> or regular text.</h3>
/// ```
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Heading {
    pub level: u8,
    pub body: Vec<HeadingNodes>,
}

impl Heading {
    pub fn new(level: u8, nodes: Vec<HeadingNodes>) -> Self {
        Self { level, body: nodes }
    }
}

impl Display for Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", "#".repeat(self.level as usize))?;
        for node in &self.body {
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{Anchor, Heading, HeadingNodes};

    #[test]
    fn heading() {
        let heading = Heading::new(
            3,
            vec![
                HeadingNodes::from("Header can contain an ".to_string()),
                HeadingNodes::from(Anchor::new("anchor".to_string(), "#".to_string())),
                HeadingNodes::from(" or regular text.".to_string()),
            ],
        );
        assert_eq!(
            heading.to_string(),
            "### Header can contain an [anchor](#) or regular text."
        );
    }

    #[test]
    fn heading_with_hash() {
        let heading = Heading::new(3, vec![HeadingNodes::from("# ##".to_string())]);
        assert_eq!(heading.to_string(), "### \\# \\#\\#");
    }
}
