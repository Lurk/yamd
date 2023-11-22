use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

use super::{
    list::{List, ListTypes},
    list_item_content::ListItemContent,
};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct ListItem {
    pub list_type: ListTypes,
    pub level: usize,
    pub text: ListItemContent,
    pub nested_list: Option<List>,
}

impl ListItem {
    pub fn new(list_type: ListTypes, level: usize, text: ListItemContent) -> Self {
        Self::new_with_nested_list(list_type, level, text, None)
    }

    pub fn new_with_nested_list(
        list_type: ListTypes,
        level: usize,
        text: ListItemContent,
        nested_list: Option<List>,
    ) -> Self {
        Self {
            list_type,
            level,
            text,
            nested_list,
        }
    }
    fn get_list_type_from_context(ctx: &Option<Context>) -> ListTypes {
        if let Some(ctx) = ctx {
            if let Some(list_type) = ctx.get_char_value("list_type") {
                if list_type == '+' {
                    return ListTypes::Ordered;
                }
            }
        }
        ListTypes::Unordered
    }

    fn get_level_from_context(ctx: &Option<Context>) -> usize {
        match ctx {
            Some(ctx) => ctx.get_usize_value("level").unwrap_or(0),
            None => 0,
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

impl Node for ListItem {
    fn len(&self) -> usize {
        self.nested_list.as_ref().map_or(0, |list| list.len() + 1)
            + self.text.len()
            + self.level
            + 2
    }
}

impl Deserializer for ListItem {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = Self::get_level_from_context(&ctx);
        let list_type = match Self::get_list_type_from_context(&ctx) {
            ListTypes::Unordered => "-",
            ListTypes::Ordered => "+",
        };
        let mut matcher = Matcher::new(input);
        if let Some(list_item) = matcher.get_match(
            format!("{}{} ", " ".repeat(level), list_type).as_str(),
            format!("\n{}{} ", " ".repeat(level), list_type).as_str(),
            true,
        ) {
            if let Some(text) = ListItemContent::deserialize(list_item.body) {
                let mut nested_list = None;
                if text.len() + 1 < list_item.body.len() {
                    let mut ctx = Context::new();
                    ctx.add("level", level);
                    if let Some(list) =
                        List::deserialize_with_context(&list_item.body[text.len() + 1..], Some(ctx))
                    {
                        nested_list = Some(list);
                    } else {
                        return None;
                    }
                }
                return Some(Self::new_with_nested_list(
                    Self::get_list_type_from_context(&ctx),
                    level,
                    text,
                    nested_list,
                ));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::ListItem;
    use crate::{
        nodes::{
            list::{List, ListTypes},
            list_item_content::ListItemContent,
            text::Text,
        },
        toolkit::{context::Context, deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize() {
        assert_eq!(
            ListItem::new(
                ListTypes::Unordered,
                0,
                ListItemContent::new(vec![Text::new("test").into()])
            )
            .to_string(),
            "- test".to_string()
        );

        assert_eq!(
            ListItem::new(
                ListTypes::Ordered,
                0,
                ListItemContent::new(vec![Text::new("test").into()])
            )
            .to_string(),
            "+ test".to_string()
        );

        assert_eq!(
            ListItem::new_with_nested_list(
                ListTypes::Unordered,
                0,
                ListItemContent::new(vec![Text::new("test").into()]),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new(vec![Text::new("test").into()])
                    )
                    .into()]
                ))
            )
            .to_string(),
            "- test\n - test".to_string()
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ListItem::new(
                ListTypes::Unordered,
                0,
                ListItemContent::new(vec![Text::new("test").into()])
            )
            .len(),
            6
        );

        assert_eq!(
            ListItem::new_with_nested_list(
                ListTypes::Unordered,
                0,
                ListItemContent::new(vec![Text::new("test").into()]),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new(vec![Text::new("test").into()])
                    )
                    .into()]
                ))
            )
            .len(),
            14
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            ListItem::deserialize("- test").unwrap(),
            ListItem::new(
                ListTypes::Unordered,
                0,
                ListItemContent::new(vec![Text::new("test").into()])
            )
        );
    }

    #[test]
    fn deserialize_with_nested_list() {
        assert_eq!(
            ListItem::deserialize("- 111111\n - 22222"),
            Some(ListItem::new_with_nested_list(
                ListTypes::Unordered,
                0,
                ListItemContent::new(vec![Text::new("111111").into()]),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new(vec![Text::new("22222").into()])
                    )
                    .into()]
                ))
            ))
        );
    }

    #[test]
    fn deserialize_with_nested_list_and_text() {
        assert_eq!(ListItem::deserialize("- 111111\n - 22222\n 33333"), None);
    }

    #[test]
    fn deserialize_with_wrong_context() {
        let mut ctx = Context::new();
        ctx.add("list_type", '-');
        assert_eq!(
            ListItem::deserialize_with_context("+ test", Some(ctx)),
            None
        );
    }
}
