use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Anchor, Bold, CodeSpan, Emphasis, Italic, Strikethrough};

#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
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

impl Display for ParagraphNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphNodes::Anchor(a) => write!(f, "{}", a),
            ParagraphNodes::Bold(b) => write!(f, "{}", b),
            ParagraphNodes::Italic(i) => write!(f, "{}", i),
            ParagraphNodes::Strikethrough(s) => write!(f, "{}", s),
            ParagraphNodes::Text(t) => write!(
                f,
                "{}",
                t.replace("\\", "\\\\")
                    .replace("*", "\\*")
                    .replace("_", "\\_")
                    .replace("~", "\\~")
                    .replace("`", "\\`")
                    .replace("[", "\\[")
                    .replace("{", "\\{")
                    .replace("\n\n", "\\\n\\\n")
                    .replace("%}", "\\%}")
            ),
            ParagraphNodes::CodeSpan(c) => write!(f, "{}", c),
            ParagraphNodes::Emphasis(e) => write!(f, "{}", e),
        }
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
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Paragraph {
    pub body: Vec<ParagraphNodes>,
}

impl Paragraph {
    pub fn new(nodes: Vec<ParagraphNodes>) -> Self {
        Self { body: nodes }
    }
}

/// Prefer [`Paragraph::new`]. This impl is retained for backward compatibility
/// and may be removed in a future major release.
impl Default for Paragraph {
    fn default() -> Self {
        Self::new(vec![])
    }
}

/// Prefer [`Paragraph::new`]. This impl is retained for backward compatibility
/// and may be removed in a future major release.
impl From<Vec<ParagraphNodes>> for Paragraph {
    fn from(value: Vec<ParagraphNodes>) -> Self {
        Self::new(value)
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.body {
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}

/// Prepends a `\` escape when the serialized paragraph starts with a block-start
/// marker that would otherwise hijack re-parsing inside a recursively-parsed
/// container (Highlight body, Collapsible body). Must be applied AFTER any
/// container-specific closer escape (e.g. Highlight's universal `!!` escape),
/// because those escapes can rewrite the leading characters.
pub(crate) fn escape_leading_block_marker(s: String) -> String {
    if s.starts_with("- ")
        || s.starts_with("+ ")
        || s.starts_with("# ")
        || s.starts_with("!!")
        || s.starts_with("---")
    {
        format!("\\{}", s)
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{
        Anchor, Bold, BoldNodes, CodeSpan, Emphasis, Italic, Paragraph, ParagraphNodes,
        Strikethrough,
    };

    #[test]
    fn paragraph() {
        let paragraph = Paragraph::new(vec![
            ParagraphNodes::from("Paragraph can contain an ".to_string()),
            ParagraphNodes::from(Anchor::new("anchor", "https://example.com")),
            ParagraphNodes::from(", a ".to_string()),
            ParagraphNodes::from(CodeSpan::new("code span")),
            ParagraphNodes::from(", or ".to_string()),
            ParagraphNodes::from(Bold::new(vec![BoldNodes::from("bold".to_string())])),
            ParagraphNodes::from(", or ".to_string()),
            ParagraphNodes::from(Italic::new("italic")),
            ParagraphNodes::from(", or ".to_string()),
            ParagraphNodes::from(Strikethrough::new("strikethrough")),
            ParagraphNodes::from(", or ".to_string()),
            ParagraphNodes::from(Emphasis::new("emphasis")),
            ParagraphNodes::from(", or regular text.".to_string()),
        ]);
        assert_eq!(
            paragraph.to_string(),
            "Paragraph can contain an [anchor](https://example.com), a `code span`, or **bold**, or _italic_, or ~~strikethrough~~, or *emphasis*, or regular text."
        );
    }

    #[test]
    fn paragraph_with_terminator() {
        let paragraph = Paragraph::new(vec![ParagraphNodes::from("\n\n".to_string())]);
        assert_eq!(paragraph.to_string(), "\\\n\\\n");
    }

    #[test]
    fn paragraph_text_with_curly_close() {
        let paragraph = Paragraph::new(vec![ParagraphNodes::from("%}".to_string())]);
        assert_eq!(paragraph.to_string(), "\\%}");
    }

    #[test]
    fn default_produces_empty_paragraph() {
        let p: Paragraph = Default::default();
        assert_eq!(p.to_string(), "");
    }

    #[test]
    fn from_vec_produces_paragraph_with_body() {
        let p: Paragraph = vec![ParagraphNodes::from("hi".to_string())].into();
        assert_eq!(p.to_string(), "hi");
    }
}
