use std::fmt::Display;

use serde::Serialize;

use super::{List, ParagraphNodes};

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
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
