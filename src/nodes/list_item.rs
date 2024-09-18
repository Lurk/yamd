use std::fmt::Display;

use serde::Serialize;

use super::{List, ListTypes, ParagraphNodes};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct ListItem {
    pub list_type: ListTypes,
    pub level: usize,
    pub text: Vec<ParagraphNodes>,
    pub nested_list: Option<List>,
}

impl ListItem {
    pub fn new(
        list_type: ListTypes,
        level: usize,
        text: Vec<ParagraphNodes>,
        nested_list: Option<List>,
    ) -> Self {
        Self {
            list_type,
            level,
            text,
            nested_list,
        }
    }
}

impl Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list_type = match self.list_type {
            ListTypes::Unordered => '-',
            ListTypes::Ordered => '+',
        };
        write!(
            f,
            "{}{} {}{}",
            String::from(' ').repeat(self.level),
            list_type,
            self.text.iter().map(|n| n.to_string()).collect::<String>(),
            self.nested_list
                .as_ref()
                .map_or("".to_string(), |list| format!("\n{}", list))
        )
    }
}
