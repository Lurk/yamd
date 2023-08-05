use crate::toolkit::{
    context::Context, deserializer::Deserializer, matcher::Matcher, node::Node,
    pattern::Quantifiers::*,
};

use super::{
    list::{List, ListTypes},
    list_item_content::ListItemContent,
};

#[derive(Debug, PartialEq)]
pub struct ListItem {
    list_type: ListTypes,
    level: usize,
    text: ListItemContent,
    nested_list: Option<List>,
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

impl Node for ListItem {
    fn serialize(&self) -> String {
        let list_type = match self.list_type {
            ListTypes::Unordered => '-',
            ListTypes::Ordered => '+',
        };
        format!(
            "{}{} {}{}",
            String::from(' ').repeat(self.level),
            list_type,
            self.text.serialize(),
            self.nested_list
                .as_ref()
                .map_or("".to_string(), |list| list.serialize())
        )
    }

    fn len(&self) -> usize {
        self.nested_list.as_ref().map_or(0, |list| list.len()) + self.text.len() + self.level + 2
    }
}

impl Deserializer for ListItem {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = Self::get_level_from_context(&ctx);
        let list_type = match Self::get_list_type_from_context(&ctx) {
            ListTypes::Unordered => Once('-'),
            ListTypes::Ordered => Once('+'),
        };
        let mut matcher = Matcher::new(input);
        if let Some(list_item) = matcher.get_match(
            &[RepeatTimes(level, ' '), list_type.clone(), Once(' ')],
            &[Once('\n'), RepeatTimes(level, ' '), list_type, Once(' ')],
            true,
        ) {
            let content_body = if list_item.end_token.is_empty() {
                list_item.body
            } else {
                &input[list_item.start_token.len()
                    ..list_item.start_token.len() + list_item.body.len() + 1]
            };
            if let Some(text) = ListItemContent::deserialize(content_body) {
                let mut nested_list = None;
                if text.len() < list_item.body.len() {
                    let mut ctx = Context::new();
                    ctx.add("level", level);
                    if let Some(list) =
                        List::deserialize_with_context(&list_item.body[text.len()..], Some(ctx))
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
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize() {
        assert_eq!(
            ListItem::new(
                ListTypes::Unordered,
                0,
                ListItemContent::new_with_nodes(true, vec![Text::new("test").into()])
            )
            .serialize(),
            "- test".to_string()
        );

        assert_eq!(
            ListItem::new(
                ListTypes::Ordered,
                0,
                ListItemContent::new_with_nodes(true, vec![Text::new("test").into()])
            )
            .serialize(),
            "+ test".to_string()
        );

        assert_eq!(
            ListItem::new_with_nested_list(
                ListTypes::Unordered,
                0,
                ListItemContent::new_with_nodes(false, vec![Text::new("test").into()]),
                Some(List::new_with_nodes(
                    true,
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new_with_nodes(true, vec![Text::new("test").into()])
                    )
                    .into()]
                ))
            )
            .serialize(),
            "- test\n - test".to_string()
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ListItem::new(
                ListTypes::Unordered,
                0,
                ListItemContent::new_with_nodes(true, vec![Text::new("test").into()])
            )
            .len(),
            6
        );

        assert_eq!(
            ListItem::new_with_nested_list(
                ListTypes::Unordered,
                0,
                ListItemContent::new_with_nodes(false, vec![Text::new("test").into()]),
                Some(List::new_with_nodes(
                    true,
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new_with_nodes(true, vec![Text::new("test").into()])
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
                ListItemContent::new_with_nodes(true, vec![Text::new("test").into()])
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
                ListItemContent::new_with_nodes(false, vec![Text::new("111111").into()]),
                Some(List::new_with_nodes(
                    true,
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new_with_nodes(true, vec![Text::new("22222").into()])
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
}
