use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Italic, Strikethrough};

#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
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

impl Display for BoldNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoldNodes::Italic(i) => write!(f, "{}", i),
            BoldNodes::Strikethrough(s) => write!(f, "{}", s),
            BoldNodes::Text(t) => {
                write!(
                    f,
                    "{}",
                    t.replace("\\", "\\\\")
                        .replace("*", "\\*")
                        .replace("_", "\\_")
                        .replace("~", "\\~")
                        .replace("\n\n", "\\\n\n")
                )
            }
        }
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
/// **Bold can contain  _italic_, or ~~strikethrough~~, or regular text**
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <b>
///     Bold can contain
///     <i>italic</i>
///     , or
///     <s>strikethrough</s>
///     , or regular text
/// </b>
/// ```
///
/// # Round-trip invariant
///
/// A body with no effective content (empty children or only empty-string text
/// children) has no meaningful semantic or visual interpretation. The
/// `Display` impl currently collapses such a value to the empty string, which
/// means it is AST-lossy (disappears on round-trip). Constructing a bold with
/// an empty effective body is permitted for now but should not be relied on —
/// a future breaking change is expected to reject empty bodies at
/// construction.
#[derive(Debug, PartialEq, Clone, Default, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bold {
    pub body: Vec<BoldNodes>,
}

impl Bold {
    pub fn new(body: Vec<BoldNodes>) -> Self {
        Self { body }
    }
}

/// Prefer [`Bold::new`]. This impl is retained for backward compatibility
/// and may be removed in a future major release.
impl From<Vec<BoldNodes>> for Bold {
    fn from(value: Vec<BoldNodes>) -> Self {
        Self::new(value)
    }
}

impl Display for Bold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_content = self.body.iter().any(|node| match node {
            BoldNodes::Text(t) => !t.is_empty(),
            _ => true,
        });
        if !has_content {
            return Ok(());
        }
        write!(f, "**")?;
        for node in &self.body {
            write!(f, "{}", node.to_string().replace("**", "\\**"))?;
        }
        write!(f, "**")
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{Bold, BoldNodes, Italic, Strikethrough};

    #[test]
    fn bold() {
        let bold = Bold::new(vec![
            BoldNodes::from("Bold can contain ".to_string()),
            BoldNodes::from(Italic::new("italic")),
            BoldNodes::from(", or ".to_string()),
            BoldNodes::from(Strikethrough::new("strikethrough")),
            BoldNodes::from(", or regular text".to_string()),
        ]);
        assert_eq!(
            bold.to_string(),
            "**Bold can contain _italic_, or ~~strikethrough~~, or regular text**".to_string()
        );
    }

    #[test]
    fn bold_with_terminator() {
        let bold = Bold::new(vec![BoldNodes::from("\n\n".to_string())]);
        assert_eq!(bold.to_string(), "**\\\n\n**");
    }

    #[test]
    fn bold_with_only_non_text_nodes() {
        let bold = Bold::new(vec![BoldNodes::from(Italic::new("x"))]);
        assert_eq!(bold.to_string(), "**_x_**");
    }

    #[test]
    fn from_vec_produces_bold_with_body() {
        let b: Bold = vec![BoldNodes::from("hi".to_string())].into();
        assert_eq!(b.to_string(), "**hi**");
    }
}
