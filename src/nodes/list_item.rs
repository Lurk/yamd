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
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListItem {
    pub text: Vec<ParagraphNodes>,
    pub nested_list: Option<List>,
}

impl ListItem {
    pub fn new(text: Vec<ParagraphNodes>, nested_list: Option<List>) -> Self {
        Self { text, nested_list }
    }
}

impl Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for text in &self.text {
            write!(f, "{}", text)?;
        }

        if let Some(nested_list) = &self.nested_list {
            write!(f, "\n{}", nested_list)?;
        }

        Ok(())
    }
}
