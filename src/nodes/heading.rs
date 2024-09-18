use std::fmt::Display;

use serde::Serialize;

use super::Anchor;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
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
            Self::Text(text) => write!(f, "{}", text),
            Self::Anchor(anchor) => write!(f, "{}", anchor),
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
#[derive(Debug, PartialEq, Serialize, Clone)]
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
        let level = String::from('#').repeat(self.level as usize);
        write!(
            f,
            "{} {}",
            level,
            self.body.iter().map(|n| n.to_string()).collect::<String>()
        )
    }
}
