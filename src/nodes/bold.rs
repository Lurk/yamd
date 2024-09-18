use std::fmt::{Display, Formatter};

use serde::Serialize;

use super::{Italic, Strikethrough};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum BoldNodes {
    Italic(Italic),
    Strikethrough(Strikethrough),
    Text(String),
}

impl From<Italic> for BoldNodes {
    fn from(i: Italic) -> Self {
        BoldNodes::Italic(i)
    }
}

impl From<Strikethrough> for BoldNodes {
    fn from(s: Strikethrough) -> Self {
        BoldNodes::Strikethrough(s)
    }
}

impl From<String> for BoldNodes {
    fn from(t: String) -> Self {
        BoldNodes::Text(t)
    }
}

/// # Bold
///
/// Any token except [Terminator](type@crate::lexer::TokenKind::Terminator) surrounded by
/// [Star](type@crate::lexer::TokenKind::Star) of length 2.
///
/// [Body](Bold::body) can contain one or more:
///
/// - [Italic]
/// - [Strikethrough]
/// - [String]
///
/// Example:
///
/// ```text
/// **Bold can contain an [anchor](#) and _italic_, or ~~strikethrough~~, or regular text**
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <b>
///     Bold can contain an
///     <a href="#">anchor</a>
///     and
///     <i>italic</i>
///     , or
///     <s>strikethrough</s>
///     , or regular text
/// </b>
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Default)]
pub struct Bold {
    pub body: Vec<BoldNodes>,
}

impl Bold {
    pub fn new(body: Vec<BoldNodes>) -> Self {
        Self { body }
    }
}

impl Display for BoldNodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoldNodes::Text(node) => write!(f, "{}", node),
            BoldNodes::Italic(node) => write!(f, "{}", node),
            BoldNodes::Strikethrough(node) => write!(f, "{}", node),
        }
    }
}

impl Display for Bold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "**{}**",
            self.body
                .iter()
                .map(|node| { node.to_string() })
                .collect::<Vec<String>>()
                .concat()
        )
    }
}
