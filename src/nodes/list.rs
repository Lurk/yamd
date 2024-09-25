use serde::Serialize;

use super::ListItem;

#[derive(Debug, PartialEq, Clone, Serialize, Eq)]
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
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
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
