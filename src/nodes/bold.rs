use std::fmt::Display;

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

impl Display for BoldNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoldNodes::Italic(i) => write!(f, "{}", i),
            BoldNodes::Strikethrough(s) => write!(f, "{}", s),
            BoldNodes::Text(t) => write!(f, "{}", t.replace("\n\n", "\\\n\n")),
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

impl Display for Bold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
}
