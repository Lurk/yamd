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
