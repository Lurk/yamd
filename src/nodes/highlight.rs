use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Paragraph;

/// # Highlight
///
/// Must start and end with [Bang](type@crate::lexer::TokenKind::Bang) of length 2.
///
/// [Title](Highlight::title) is sequence of tokens between first
/// [Bang](type@crate::lexer::TokenKind::Bang) of length 2 followed by
/// [Space](type@crate::lexer::TokenKind::Space) and [Eol](type@crate::lexer::TokenKind::Eol).
/// Can be omitted.
///
/// [Icon](Highlight::icon) is sequence of tokens between
/// [Bang](type@crate::lexer::TokenKind::Bang) of length 1 followed by
/// [Space](type@crate::lexer::TokenKind::Space) and [Eol](type@crate::lexer::TokenKind::Eol).
/// Can be omitted.
///
/// [Title](Highlight::title) and [Icon](Highlight::icon) can not contain
/// [Terminator](type@crate::lexer::TokenKind::Terminator).
///
/// [Body](Highlight::body) is one or more [Paragraph]'s.
///
/// Example:
///
/// ```text
/// !! Tile
/// ! Icon
/// body
/// !!
/// ```
///
/// Example without title:
///
/// ```text
/// !!
/// ! Icon
/// body
/// !!
/// ```
///
/// Example without icon:
///
/// ```text
/// !! Tile
/// body
/// !!
/// ```
///
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Highlight {
    pub title: Option<String>,
    pub icon: Option<String>,
    pub body: Vec<Paragraph>,
}

impl Highlight {
    pub fn new<T: Into<String>, I: Into<String>>(
        title: Option<T>,
        icon: Option<I>,
        body: Vec<Paragraph>,
    ) -> Self {
        Self {
            title: title.map(|title| title.into()),
            icon: icon.map(|icon| icon.into()),
            body,
        }
    }
}

impl Display for Highlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = self
            .title
            .as_ref()
            .map_or("".to_string(), |t| format!(" {}", t));
        let icon = self
            .icon
            .as_ref()
            .map_or("".to_string(), |i| format!("! {}\n", i));
        write!(
            f,
            "!!{}\n{}{}\n!!",
            title,
            icon,
            self.body
                .iter()
                .map(|paragraph| paragraph.to_string())
                .collect::<Vec<_>>()
                .join("\n\n")
                .replace("!!", "\\!!")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Paragraph;

    #[test]
    fn highlight() {
        let highlight = Highlight::new(
            Some("title"),
            Some("icon"),
            vec![Paragraph::new(vec!["body".to_string().into()])],
        );
        assert_eq!(highlight.to_string(), "!! title\n! icon\nbody\n!!");
    }

    #[test]
    fn highlight_without_title() {
        let highlight = Highlight::new::<&str, &str>(
            None,
            Some("icon"),
            vec![Paragraph::new(vec!["body".to_string().into()])],
        );
        assert_eq!(highlight.to_string(), "!!\n! icon\nbody\n!!");
    }

    #[test]
    fn highlight_without_icon() {
        let highlight = Highlight::new::<&str, &str>(
            Some("title"),
            None,
            vec![Paragraph::new(vec!["body".to_string().into()])],
        );
        assert_eq!(highlight.to_string(), "!! title\nbody\n!!");
    }

    #[test]
    fn highlight_with_double_bang_in_the_middle() {
        let highlight = Highlight::new::<&str, &str>(
            Some("title"),
            Some("icon"),
            vec![
                Paragraph::new(vec!["body".to_string().into()]),
                Paragraph::new(vec!["a\n!!".to_string().into()]),
            ],
        );
        assert_eq!(
            highlight.to_string(),
            "!! title\n! icon\nbody\n\na\n\\!!\n!!"
        );
    }
}
