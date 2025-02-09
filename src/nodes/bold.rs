use serde::Serialize;

use super::{Italic, Strikethrough};

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
#[serde(tag = "type", content = "value")]
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
#[derive(Debug, PartialEq, Serialize, Clone, Default, Eq)]
pub struct Bold {
    pub body: Vec<BoldNodes>,
}

impl Bold {
    pub fn new(body: Vec<BoldNodes>) -> Self {
        Self { body }
    }
}

impl From<Vec<BoldNodes>> for Bold {
    fn from(value: Vec<BoldNodes>) -> Self {
        Self::new(value)
    }
}
