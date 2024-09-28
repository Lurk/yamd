use serde::Serialize;

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
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
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
