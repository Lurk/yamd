use std::fmt::Display;

use serde::Serialize;

use super::{
    list::{List, ListTypes},
    paragraph::Paragraph,
};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct ListItem {
    pub list_type: ListTypes,
    pub level: usize,
    pub text: Paragraph,
    pub nested_list: Option<List>,
}

impl ListItem {
    pub fn new(
        list_type: ListTypes,
        level: usize,
        text: Paragraph,
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
            self.text,
            self.nested_list
                .as_ref()
                .map_or("".to_string(), |list| format!("\n{}", list))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ListItem;
    use crate::nodes::{
        list::{List, ListTypes},
        paragraph::Paragraph,
        text::Text,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize() {
        assert_eq!(
            ListItem::new(
                ListTypes::Unordered,
                0,
                Paragraph::new(vec![Text::new("test").into()]),
                None
            )
            .to_string(),
            "- test".to_string()
        );

        assert_eq!(
            ListItem::new(
                ListTypes::Ordered,
                0,
                Paragraph::new(vec![Text::new("test").into()]),
                None
            )
            .to_string(),
            "+ test".to_string()
        );

        assert_eq!(
            ListItem::new(
                ListTypes::Unordered,
                0,
                Paragraph::new(vec![Text::new("test").into()]),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        Paragraph::new(vec![Text::new("test").into()]),
                        None
                    )
                    .into()]
                ))
            )
            .to_string(),
            "- test\n - test".to_string()
        );
    }
}
