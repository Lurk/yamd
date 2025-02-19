use serde::Serialize;

use super::{Anchor, Bold, CodeSpan, Emphasis, Italic, Strikethrough};

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
#[serde(tag = "type", content = "value")]
pub enum ParagraphNodes {
    Anchor(Anchor),
    Bold(Bold),
    Italic(Italic),
    Strikethrough(Strikethrough),
    Text(String),
    CodeSpan(CodeSpan),
    Emphasis(Emphasis),
}

impl From<Anchor> for ParagraphNodes {
    fn from(value: Anchor) -> Self {
        ParagraphNodes::Anchor(value)
    }
}

impl From<Bold> for ParagraphNodes {
    fn from(value: Bold) -> Self {
        ParagraphNodes::Bold(value)
    }
}

impl From<Italic> for ParagraphNodes {
    fn from(value: Italic) -> Self {
        ParagraphNodes::Italic(value)
    }
}

impl From<Strikethrough> for ParagraphNodes {
    fn from(value: Strikethrough) -> Self {
        ParagraphNodes::Strikethrough(value)
    }
}

impl From<String> for ParagraphNodes {
    fn from(value: String) -> Self {
        ParagraphNodes::Text(value)
    }
}

impl From<CodeSpan> for ParagraphNodes {
    fn from(value: CodeSpan) -> Self {
        ParagraphNodes::CodeSpan(value)
    }
}

impl From<Emphasis> for ParagraphNodes {
    fn from(value: Emphasis) -> Self {
        ParagraphNodes::Emphasis(value)
    }
}

/// # Paragraph
///
/// Any token until [Terminator](type@crate::lexer::TokenKind::Terminator) or end of input.
///
/// [Body](Paragraph::body) can contain one or more:
///
/// - [Anchor]
/// - [CodeSpan]
/// - [Bold]
/// - [Italic]
/// - [Strikethrough]
/// - [Emphasis]
/// - [String]
///
/// Example:
///
/// ```text
/// Paragraph can contain an [anchor](#), a `code span`, and **bold**, or _italic_, or ~~strikethrough~~, or
/// *emphasis*, or regular text.
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <p>
///     Paragraph can contain an
///     <a href="#">anchor</a>
///     , a
///     <code>code span</code>
///     , and
///     <b>bold</b>
///     , or
///     <i>italic</i>
///     , or
///     <s>strikethrough</s>
///     , or <em>emphasis</em>
///     , or regular text.
/// </p>
/// ```
///
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Paragraph {
    pub body: Vec<ParagraphNodes>,
}

impl Paragraph {
    pub fn new(nodes: Vec<ParagraphNodes>) -> Self {
        Self { body: nodes }
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl From<Vec<ParagraphNodes>> for Paragraph {
    fn from(value: Vec<ParagraphNodes>) -> Self {
        Self::new(value)
    }
}
