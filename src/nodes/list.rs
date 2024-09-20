use std::fmt::{Display, Formatter};

use serde::Serialize;

use super::ListItem;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ListTypes {
    /// List item starts with `-` ([Minus](type@crate::lexer::TokenKind::Minus) of length 1) followed by space
    /// [Space](type@crate::lexer::TokenKind::Space). Must be rendered as bullet marked list.
    Unordered,
    /// List item starts with `+` ([Plus](type@crate::lexer::TokenKind::Plus) of length 1) followed by space
    /// [Space](type@crate::lexer::TokenKind::Space). Must be rendered as numeric list.
    Ordered,
}

/// # List
///
/// ## Types
///
/// List can be two types (check [ListTypes] for more info). Each list can contain sub list of any type, but type can
/// not be mixed on a same level.
///
/// Examples:
///
/// ```text
/// - Unordered list item
///  + Ordered list item
/// ```
///
/// ```text
/// - Unordered list item
/// + Even though this looks like ordered list item, it will be part of Unordered list item
/// ```
///
/// ## Level
///
/// List level determined by amount of spaces ([Space](type@crate::lexer::TokenKind::Space)) before list type marker.
/// No space means level `0`. Level can be increased only by 1. Level can be decreased to any
/// level.
///
/// Examples:
///
/// ```text
/// - Level 0
///  - Level 1
///   - Level 2
///  - Level 1
/// - Level 0
///  - Level 1
///   - Level 2
/// - Level 0
/// ```
///
/// ```text
/// + level 0
///   + this will be part of level 0 (notice two spaces before `-`)
/// ```
///
///
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct List {
    pub list_type: ListTypes,
    pub level: usize,
    pub body: Vec<ListItem>,
}

impl List {
    pub fn new(list_type: ListTypes, level: usize, body: Vec<ListItem>) -> Self {
        Self {
            list_type,
            level,
            body,
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for n in self.body.iter() {
            f.write_str(n.to_string().as_str())?;
        }
        Ok(())
    }
}
