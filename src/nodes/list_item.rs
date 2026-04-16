use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{List, ParagraphNodes};

/// # ListItem
///
/// A single item in a [`List`]. Contains inline text and an optional nested [`List`].
///
/// ```text
/// - Item text
///  - Nested item
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <ul><li>Item text<ul><li>Nested item</li></ul></li><ul>
/// ```
///
/// # Round-trip invariant
///
/// The grammar requires exactly one space between the list marker and the item
/// text, so a `ListItem` whose `text` begins with whitespace cannot be produced
/// by parsing any source document. Constructing such a value is permitted, but
/// serializing and re-parsing it will not yield an equal value.
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListItem {
    pub text: Vec<ParagraphNodes>,
    pub nested_list: Option<List>,
}

impl ListItem {
    /// See the type-level docs for the round-trip invariant on `text`.
    pub fn new(text: Vec<ParagraphNodes>, nested_list: Option<List>) -> Self {
        Self { text, nested_list }
    }
}

impl Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: String = self.text.iter().map(|t| t.to_string()).collect();
        match &self.nested_list {
            Some(nested) => write!(f, "{}\n{}", text, nested),
            None => write!(f, "{}", text),
        }
    }
}
