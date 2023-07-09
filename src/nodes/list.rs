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
    pub fn new(list_type: ListTypes, level: usize, consumed_all_input: bool) -> Self {
        Self::new_with_nodes(list_type, level, consumed_all_input, vec![])
    }

    pub fn new_with_nodes(
        list_type: ListTypes,
        level: usize,
        consumed_all_input: bool,
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
                return level;
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
                .join("\n")
        )
    }
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.nodes.len() - 1
            + self.get_outer_token_length()
    }
    fn context(&self) -> Option<Context> {
        Some(Self::create_context(self.level, &self.list_type))
    }
}

impl Deserializer for List {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = match ctx {
            Some(_) => Self::get_level_from_context(&ctx) + 1,
            None => 0,
        };
        let mut matcher = Matcher::new(input);
        if let Some(unordered_list) = matcher.get_match(
            &[
                ZeroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('-'),
                Once(' '),
            ],
            &[RepeatTimes(2, '\n')],
            true,
        ) {
            return Self::parse_branch(
                &input[..unordered_list.start_token.len() + unordered_list.body.len()],
                Self::new(
                    ListTypes::Unordered,
                    level,
                    unordered_list.end_token.is_empty(),
                ),
            );
        } else if let Some(ordered_list) = matcher.get_match(
            &[
                ZeroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('+'),
                Once(' '),
            ],
            &[RepeatTimes(2, '\n')],
            true,
        ) {
            return Self::parse_branch(
                &input[..ordered_list.start_token.len() + ordered_list.body.len()],
                Self::new(ListTypes::Ordered, level, ordered_list.end_token.is_empty()),
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
        nodes::{list_item::ListItem, paragraph::Paragraph, text::Text},
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
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![Paragraph::new_with_nodes(
                            true,
                            vec![Text::new("unordered list item").into()],
                        )
                        .into()],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![Paragraph::new_with_nodes(
                            true,
                            vec![Text::new("unordered list item").into()],
                        )
                        .into()],
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
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![Paragraph::new_with_nodes(
                            true,
                            vec![Text::new("unordered list item").into()],
                        )
                        .into()],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![Paragraph::new_with_nodes(
                            true,
                            vec![Text::new("unordered list item").into()],
                        )
                        .into()],
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
            ListTypes::Ordered,
            0,
            true,
            vec![
                ListItem::new_with_nodes(
                    ListTypes::Ordered,
                    0,
                    vec![Paragraph::new_with_nodes(
                        true,
                        vec![Text::new("ordered list item").into()],
                    )
                    .into()],
                )
                .into(),
                ListItem::new_with_nodes(
                    ListTypes::Ordered,
                    0,
                    vec![Paragraph::new_with_nodes(
                        true,
                        vec![Text::new("ordered list item").into()],
                    )
                    .into()],
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
                ListTypes::Unordered,
                0,
                true,
                vec![
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
                    )
                    .into(),
                ],
            ))
        );
        assert_eq!(
            List::deserialize("- level 0\n- level 0\n\n"),
            Some(List::new_with_nodes(
                ListTypes::Unordered,
                0,
                false,
                vec![
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Unordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
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
                ListTypes::Ordered,
                0,
                true,
                vec![
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
                    )
                    .into(),
                ],
            ))
        );
        assert_eq!(
            List::deserialize("+ level 0\n+ level 0\n\n"),
            Some(List::new_with_nodes(
                ListTypes::Ordered,
                0,
                false,
                vec![
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()])
                                .into()
                        ],
                    )
                    .into(),
                ],
            ))
        );
    }

    #[test]
    fn deserialize_mixed() {
        let list = List::new_with_nodes(
            ListTypes::Ordered,
            0,
            true,
            vec![ListItem::new_with_nodes(
                ListTypes::Ordered,
                0,
                vec![
                    Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()]).into(),
                    List::new_with_nodes(
                        ListTypes::Unordered,
                        1,
                        true,
                        vec![ListItem::new_with_nodes(
                            ListTypes::Unordered,
                            1,
                            vec![Paragraph::new_with_nodes(
                                true,
                                vec![Text::new("level 0").into()],
                            )
                            .into()],
                        )
                        .into()],
                    )
                    .into(),
                ],
            )
            .into()],
        );

        assert_eq!(List::deserialize("+ level 0\n - level 0"), Some(list));
    }

    #[test]
    fn len() {
        assert_eq!(
            List::new_with_nodes(
                ListTypes::Ordered,
                0,
                true,
                vec![
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![Paragraph::new_with_nodes(true, vec![Text::new("l").into()]).into()],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![Paragraph::new_with_nodes(true, vec![Text::new("l").into()]).into()],
                    )
                    .into(),
                ],
            )
            .len(),
            7
        );
        assert_eq!(
            List::new_with_nodes(
                ListTypes::Ordered,
                0,
                false,
                vec![
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![Paragraph::new_with_nodes(true, vec![Text::new("l").into()]).into()],
                    )
                    .into(),
                    ListItem::new_with_nodes(
                        ListTypes::Ordered,
                        0,
                        vec![Paragraph::new_with_nodes(true, vec![Text::new("l").into()]).into()],
                    )
                    .into(),
                ],
            )
            .len(),
            9
        );
    }
}
