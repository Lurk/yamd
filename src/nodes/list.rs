use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::ListItem;

#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ListTypes {
    /// List item starts with `-` ([Minus](type@crate::lexer::TokenKind::Minus) of length 1) followed by space
    /// [Space](type@crate::lexer::TokenKind::Space). Must be rendered as bullet marked list.
    Unordered,
    /// List item starts with `+` ([Plus](type@crate::lexer::TokenKind::Plus) of length 1) followed by space
    /// [Space](type@crate::lexer::TokenKind::Space). Must be rendered as numeric list.
    Ordered,
}

impl Display for ListTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListTypes::Unordered => write!(f, "-"),
            ListTypes::Ordered => write!(f, "+"),
        }
    }
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
/// HTML equivalent:
///
/// ```html
/// <ul><li>Unordered list item<ol><li>Ordered list item</li></ol></li></ul>
/// ```
///
/// ----
///
/// ```text
/// - Unordered list item
/// + Even though this looks like ordered list item, it will be part of Unordered list item
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <ul><li>Unordered list item
/// + Even though this looks like ordered list item, it will be part of Unordered list item</li></ul>
///
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
/// HTML equivalent:
///
/// ```html
/// <ul>
///     <li>
///         Level 0
///         <ul>
///             <li>
///                 Level 1
///                 <ul>
///                     <li>
///                         Level 2
///                     </li>
///                 </ul>
///             </li>
///             <li>
///                 Level 1
///             </li>
///         </ul>
///     </li>
///     <li>
///         Level 0
///         <ul>
///             <li>
///                 Level 1
///                 <ul>
///                     <li>
///                         Level 2
///                     </li>
///                 </ul>
///             </li>
///         </ul>
///     </li>
///     <li>
///         Level 0
///     </li>
/// </ul>
/// ```
///
/// -----
///
/// ```text
/// + level 0
///   + this will be part of level 0 (notice two spaces before `+`)
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <ol><li>level 0
///   + this will be part of level 0 (notice two spaces before `+`)</li></ol>
/// ```
///
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|list_item| format!(
                    "{}{} {}",
                    " ".repeat(self.level),
                    self.list_type,
                    list_item
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{ListItem, ListTypes};

    #[test]
    fn list() {
        let list = super::List::new(
            ListTypes::Unordered,
            0,
            vec![
                ListItem::new(vec!["test".to_string().into()], None),
                ListItem::new(vec!["test".to_string().into()], None),
            ],
        );
        assert_eq!(list.to_string(), "- test\n- test");
    }

    #[test]
    fn ordered() {
        assert_eq!(ListTypes::Ordered.to_string(), "+");
    }

    #[test]
    fn unordered() {
        assert_eq!(ListTypes::Unordered.to_string(), "-");
    }

    #[test]
    fn nested_list() {
        let list = super::List::new(
            ListTypes::Unordered,
            0,
            vec![ListItem::new(
                vec!["test".to_string().into()],
                Some(super::List::new(
                    ListTypes::Ordered,
                    1,
                    vec![ListItem::new(vec!["test".to_string().into()], None)],
                )),
            )],
        );
        assert_eq!(list.to_string(), "- test\n + test");
    }
}
