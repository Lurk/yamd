use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    matcher::Matcher,
    node::Node,
    pattern::Quantifiers::*,
};

use super::list_item::ListItem;

#[derive(Debug, PartialEq, Clone)]
pub enum ListTypes {
    Unordered,
    Ordered,
}

#[derive(Debug, PartialEq)]
pub enum ListNodes {
    ListItem(ListItem),
}

impl Node for ListNodes {
    fn serialize(&self) -> String {
        match self {
            ListNodes::ListItem(node) => node.serialize(),
        }
    }
    fn len(&self) -> usize {
        match self {
            ListNodes::ListItem(node) => node.len(),
        }
    }
}

impl From<ListItem> for ListNodes {
    fn from(value: ListItem) -> Self {
        ListNodes::ListItem(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct List {
    list_type: ListTypes,
    level: usize,
    nodes: Vec<ListNodes>,
    consumed_all_input: bool,
}

impl List {
    pub fn new(consumed_all_input: bool, list_type: ListTypes, level: usize) -> Self {
        Self::new_with_nodes(consumed_all_input, list_type, level, vec![])
    }

    pub fn new_with_nodes(
        consumed_all_input: bool,
        list_type: ListTypes,
        level: usize,
        nodes: Vec<ListNodes>,
    ) -> Self {
        Self {
            list_type,
            level,
            nodes,
            consumed_all_input,
        }
    }

    fn get_level_from_context(ctx: &Option<Context>) -> usize {
        if let Some(actual_ctx) = ctx {
            if let Some(level) = actual_ctx.get_usize_value("level") {
                return level + 1;
            }
        }
        0
    }

    pub fn create_context(level: usize, list_type: &ListTypes) -> Context {
        let mut ctx = Context::new();
        ctx.add(
            "list_type",
            match list_type {
                ListTypes::Unordered => '-',
                ListTypes::Ordered => '+',
            },
        );
        ctx.add("level", level);
        ctx
    }
}

impl Node for List {
    fn serialize(&self) -> String {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        format!(
            "{}{end}",
            self.nodes
                .iter()
                .map(|node| node.serialize())
                .collect::<Vec<String>>()
                .join("")
        )
    }
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
    fn context(&self) -> Option<Context> {
        Some(Self::create_context(self.level, &self.list_type))
    }
}

impl Deserializer for List {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = Self::get_level_from_context(&ctx);
        let mut matcher = Matcher::new(input);
        if let Some(unordered_list) = matcher.get_match(
            &[RepeatTimes(level, ' '), Once('-'), Once(' ')],
            &[RepeatTimes(2, '\n')],
            true,
        ) {
            return Self::parse_branch(
                &input[..unordered_list.start_token.len() + unordered_list.body.len()],
                Self::new(
                    unordered_list.end_token.is_empty(),
                    ListTypes::Unordered,
                    level,
                ),
            );
        } else if let Some(ordered_list) = matcher.get_match(
            &[RepeatTimes(level, ' '), Once('+'), Once(' ')],
            &[RepeatTimes(2, '\n')],
            true,
        ) {
            return Self::parse_branch(
                &input[..ordered_list.start_token.len() + ordered_list.body.len()],
                Self::new(ordered_list.end_token.is_empty(), ListTypes::Ordered, level),
            );
        }
        None
    }
}

impl Branch<ListNodes> for List {
    fn push<CanBeNode: Into<ListNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into())
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ListNodes>> {
        vec![ListItem::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ListNodes>> {
        None
    }

    fn get_outer_token_length(&self) -> usize {
        if self.consumed_all_input {
            0
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{List, ListTypes};
    use crate::{
        nodes::{list_item::ListItem, list_item_content::ListItemContent, text::Text},
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_unordered() {
        assert_eq!(
            List {
                list_type: ListTypes::Unordered,
                level: 0,
                consumed_all_input: true,
                nodes: vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(
                            false,
                            vec![Text::new("unordered list item").into()],
                        )
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(
                            true,
                            vec![Text::new("unordered list item").into()],
                        )
                    )
                    .into(),
                ],
            }
            .serialize(),
            "- unordered list item\n- unordered list item"
        );
        assert_eq!(
            List {
                list_type: ListTypes::Unordered,
                level: 0,
                consumed_all_input: false,
                nodes: vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(
                            false,
                            vec![Text::new("unordered list item").into()],
                        )
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(
                            true,
                            vec![Text::new("unordered list item").into()],
                        )
                    )
                    .into(),
                ],
            }
            .serialize(),
            "- unordered list item\n- unordered list item\n\n"
        );
    }

    #[test]
    fn serialize_ordered() {
        let list = List::new_with_nodes(
            true,
            ListTypes::Ordered,
            0,
            vec![
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new_with_nodes(
                        false,
                        vec![Text::new("ordered list item").into()],
                    ),
                )
                .into(),
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new_with_nodes(
                        true,
                        vec![Text::new("ordered list item").into()],
                    ),
                )
                .into(),
            ],
        );

        assert_eq!(list.serialize(), "+ ordered list item\n+ ordered list item");
    }

    #[test]
    fn deserialize_wrong_level() {
        assert_eq!(
            List::deserialize_with_context(
                "- level 0\n- level 0",
                Some(List::create_context(1, &ListTypes::Unordered))
            ),
            None
        );
    }

    #[test]
    fn deserialize_unordered() {
        assert_eq!(
            List::deserialize("- level 0\n- level 0"),
            Some(List::new_with_nodes(
                true,
                ListTypes::Unordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(false, vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(true, vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
        assert_eq!(
            List::deserialize("- level 0\n- level 0\n\n"),
            Some(List::new_with_nodes(
                false,
                ListTypes::Unordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(false, vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new_with_nodes(true, vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
    }

    #[test]
    fn deserialize_ordered() {
        assert_eq!(
            List::deserialize("+ level 0\n+ level 0"),
            Some(List::new_with_nodes(
                true,
                ListTypes::Ordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new_with_nodes(false, vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new_with_nodes(true, vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
        assert_eq!(
            List::deserialize("+ level 0\n+ level 0\n\n"),
            Some(List::new_with_nodes(
                false,
                ListTypes::Ordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new_with_nodes(false, vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new_with_nodes(true, vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
    }

    #[test]
    fn deserialize_mixed() {
        let list = List::new_with_nodes(
            true,
            ListTypes::Ordered,
            0,
            vec![ListItem::new_with_nested_list(
                ListTypes::Ordered,
                0,
                ListItemContent::new_with_nodes(false, vec![Text::new("level 0").into()]),
                Some(List::new_with_nodes(
                    true,
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new_with_nodes(true, vec![Text::new("level 0").into()]),
                    )
                    .into()],
                )),
            )
            .into()],
        );

        assert_eq!(List::deserialize("+ level 0\n - level 0"), Some(list));
    }

    #[test]
    fn deserialized_nested() {
        let list = List::new_with_nodes(
            false,
            ListTypes::Unordered,
            0,
            vec![ListItem::new_with_nested_list(
                ListTypes::Unordered,
                0,
                ListItemContent::new_with_nodes(false, vec![Text::new("one").into()]).into(),
                Some(List::new_with_nodes(
                    true,
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new_with_nodes(true, vec![Text::new("two").into()]),
                    )
                    .into()],
                )),
            )
            .into()],
        );

        let input = r#"- one
 - two

"#;
        assert_eq!(List::deserialize(input), Some(list));
    }

    #[test]
    fn len() {
        let list = List::new_with_nodes(
            true,
            ListTypes::Ordered,
            0,
            vec![
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new_with_nodes(false, vec![Text::new("l").into()]),
                )
                .into(),
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new_with_nodes(true, vec![Text::new("l").into()]),
                )
                .into(),
            ],
        );
        assert_eq!(list.len(), 7);
        assert_eq!(
            List::new_with_nodes(
                false,
                ListTypes::Ordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new_with_nodes(false, vec![Text::new("l").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new_with_nodes(true, vec![Text::new("l").into()])
                    )
                    .into(),
                ],
            )
            .len(),
            9
        );
    }
}
