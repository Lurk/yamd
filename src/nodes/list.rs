use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    node::Node,
    tokenizer::{
        Quantifiers::{Once, RepeatTimes, ZeroOrMore},
        Matcher,
    },
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
}

impl List {
    pub fn new(list_type: ListTypes, level: usize) -> Self {
        Self::new_with_nodes(list_type, level, vec![])
    }

    pub fn new_with_nodes(list_type: ListTypes, level: usize, nodes: Vec<ListNodes>) -> Self {
        Self {
            list_type,
            level,
            nodes,
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
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.nodes.len() - 1
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
        let matcher = Matcher::new(input);
        if matcher
            .get_node_body_start_position(&[
                ZeroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('-'),
                Once(' '),
            ])
            .is_some()
        {
            return Self::parse_branch(input, Self::new(ListTypes::Unordered, level));
        } else if matcher
            .get_node_body_start_position(&[
                ZeroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('+'),
                Once(' '),
            ])
            .is_some()
        {
            return Self::parse_branch(input, Self::new(ListTypes::Ordered, level));
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
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{list_item::ListItem, paragraph::Paragraph, text::Text},
        toolkit::{deserializer::Deserializer, node::Node},
    };

    use super::{List, ListTypes};

    #[test]
    fn serialize_unordered() {
        let list = List {
            list_type: ListTypes::Unordered,
            level: 0,
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
        };

        assert_eq!(
            list.serialize(),
            "- unordered list item\n- unordered list item"
        );
    }

    #[test]
    fn serialize_ordered() {
        let list = List::new_with_nodes(
            ListTypes::Ordered,
            0,
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
        let list = List::new_with_nodes(
            ListTypes::Unordered,
            0,
            vec![
                ListItem::new_with_nodes(
                    ListTypes::Unordered,
                    0,
                    vec![Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()]).into()],
                )
                .into(),
                ListItem::new_with_nodes(
                    ListTypes::Unordered,
                    0,
                    vec![Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()]).into()],
                )
                .into(),
            ],
        );

        assert_eq!(List::deserialize("- level 0\n- level 0"), Some(list));
    }

    #[test]
    fn deserialize_ordered() {
        let list = List::new_with_nodes(
            ListTypes::Ordered,
            0,
            vec![
                ListItem::new_with_nodes(
                    ListTypes::Ordered,
                    0,
                    vec![Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()]).into()],
                )
                .into(),
                ListItem::new_with_nodes(
                    ListTypes::Ordered,
                    0,
                    vec![Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()]).into()],
                )
                .into(),
            ],
        );

        assert_eq!(List::deserialize("+ level 0\n+ level 0"), Some(list));
    }

    #[test]
    fn deserialize_mixed() {
        let list = List::new_with_nodes(
            ListTypes::Ordered,
            0,
            vec![ListItem::new_with_nodes(
                ListTypes::Ordered,
                0,
                vec![
                    Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()]).into(),
                    List::new_with_nodes(
                        ListTypes::Unordered,
                        1,
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
        let list = List::new_with_nodes(
            ListTypes::Ordered,
            0,
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
        );

        assert_eq!(list.len(), 7);
    }
}
