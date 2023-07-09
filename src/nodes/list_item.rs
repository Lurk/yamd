use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
    pattern::Quantifiers::*,
};

use super::{
    list::{List, ListTypes},
    paragraph::Paragraph,
};

#[derive(Debug, PartialEq)]
pub enum ListItemNodes {
    Paragraph(Paragraph),
    List(List),
}

impl From<Paragraph> for ListItemNodes {
    fn from(value: Paragraph) -> Self {
        ListItemNodes::Paragraph(value)
    }
}

impl From<List> for ListItemNodes {
    fn from(value: List) -> Self {
        ListItemNodes::List(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct ListItem {
    list_type: ListTypes,
    level: usize,
    nodes: Vec<ListItemNodes>,
}

impl ListItem {
    pub fn new(list_type: ListTypes, level: usize) -> Self {
        Self::new_with_nodes(list_type, level, vec![])
    }

    pub fn new_with_nodes(list_type: ListTypes, level: usize, nodes: Vec<ListItemNodes>) -> Self {
        Self {
            list_type,
            level,
            nodes,
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

impl Node for ListItemNodes {
    fn serialize(&self) -> String {
        match self {
            ListItemNodes::Paragraph(node) => node.serialize(),
            ListItemNodes::List(node) => node.serialize(),
        }
    }
    fn len(&self) -> usize {
        match self {
            ListItemNodes::Paragraph(node) => node.len(),
            ListItemNodes::List(node) => node.len(),
        }
    }
}

impl Branch<ListItemNodes> for ListItem {
    fn push<CanBeNode: Into<ListItemNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ListItemNodes>> {
        vec![List::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ListItemNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        let add = if self.level > 0 { 1 } else { 0 };
        2 + self.level + add
    }
}

impl Node for ListItem {
    fn serialize(&self) -> String {
        let list_type = match self.list_type {
            ListTypes::Unordered => '-',
            ListTypes::Ordered => '+',
        };
        format!(
            "{}{} {}",
            String::from(' ').repeat(self.level),
            list_type,
            self.nodes
                .iter()
                .map(|element| { element.serialize() })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
    fn context(&self) -> Option<Context> {
        let mut ctx = Context::new();
        ctx.add("level", self.level);
        Some(ctx)
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
            &[
                ZeroOrMore('\n'),
                RepeatTimes(level, ' '),
                list_type.clone(),
                Once(' '),
            ],
            &[Once('\n'), RepeatTimes(level, ' '), list_type, Once(' ')],
            true,
        ) {
            return Self::parse_branch(
                list_item.body,
                Self::new(
                    Self::get_list_type_from_context(&ctx),
                    Self::get_level_from_context(&ctx),
                ),
            );
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
            paragraph::Paragraph,
            text::Text,
        },
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn deserialize() {
        assert_eq!(
            ListItem::deserialize("- foo"),
            Some(ListItem::new_with_nodes(
                ListTypes::Unordered,
                0,
                vec![Paragraph::new_with_nodes(true, vec![Text::new("foo").into()]).into()]
            ))
        );
        assert_eq!(
            ListItem::deserialize_with_context(
                "    - foo",
                Some(List::create_context(4, &ListTypes::Unordered))
            ),
            Some(ListItem::new_with_nodes(
                ListTypes::Unordered,
                4,
                vec![Paragraph::new_with_nodes(true, vec![Text::new("foo").into()]).into()],
            ))
        );
        assert_eq!(ListItem::deserialize("  s  - foo"), None);
    }

    #[test]
    fn deserialize_with_body() {
        assert_eq!(
            ListItem::deserialize("- foo\nbla\n- bar"),
            Some(ListItem::new_with_nodes(
                ListTypes::Unordered,
                0,
                vec![Paragraph::new_with_nodes(true, vec![Text::new("foo\nbla").into()]).into()]
            ))
        )
    }

    #[test]
    fn deserialize_with_nested_list() {
        assert_eq!(
            ListItem::deserialize("- foo\n - bar\n - baz"),
            Some(ListItem::new_with_nodes(
                ListTypes::Unordered,
                0,
                vec![
                    Paragraph::new_with_nodes(true, vec![Text::new("foo").into()]).into(),
                    List::new_with_nodes(
                        ListTypes::Unordered,
                        1,
                        true,
                        vec![
                            ListItem::new_with_nodes(
                                ListTypes::Unordered,
                                1,
                                vec![Paragraph::new_with_nodes(
                                    true,
                                    vec![Text::new("bar").into()]
                                )
                                .into()],
                            )
                            .into(),
                            ListItem::new_with_nodes(
                                ListTypes::Unordered,
                                1,
                                vec![Paragraph::new_with_nodes(
                                    true,
                                    vec![Text::new("baz").into()]
                                )
                                .into()],
                            )
                            .into()
                        ],
                    )
                    .into()
                ]
            ))
        );
    }

    #[test]
    fn deserialize_with_deeply_nested_list() {
        assert_eq!(
            ListItem::deserialize("- level 0\n - level 1\n  - level 2\n - level 1"),
            Some(ListItem::new_with_nodes(
                ListTypes::Unordered,
                0,
                vec![
                    Paragraph::new_with_nodes(true, vec![Text::new("level 0").into()]).into(),
                    List::new_with_nodes(
                        ListTypes::Unordered,
                        1,
                        true,
                        vec![
                            ListItem::new_with_nodes(
                                ListTypes::Unordered,
                                1,
                                vec![
                                    Paragraph::new_with_nodes(
                                        true,
                                        vec![Text::new("level 1").into()]
                                    )
                                    .into(),
                                    List::new_with_nodes(
                                        ListTypes::Unordered,
                                        2,
                                        true,
                                        vec![ListItem::new_with_nodes(
                                            ListTypes::Unordered,
                                            2,
                                            vec![Paragraph::new_with_nodes(
                                                true,
                                                vec![Text::new("level 2").into()]
                                            )
                                            .into()],
                                        )
                                        .into()],
                                    )
                                    .into()
                                ],
                            )
                            .into(),
                            ListItem::new_with_nodes(
                                ListTypes::Unordered,
                                1,
                                vec![Paragraph::new_with_nodes(
                                    true,
                                    vec![Text::new("level 1").into()]
                                )
                                .into()],
                            )
                            .into()
                        ],
                    )
                    .into()
                ],
            ))
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            ListItem::new_with_nodes(
                ListTypes::Unordered,
                0,
                vec![Paragraph::new_with_nodes(true, vec![Text::new("foo").into()]).into()]
            )
            .serialize(),
            String::from("- foo")
        );

        assert_eq!(
            ListItem::new_with_nodes(
                ListTypes::Unordered,
                6,
                vec![Paragraph::new_with_nodes(true, vec![Text::new("foo").into()]).into()],
            )
            .serialize(),
            String::from("      - foo")
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ListItem::new_with_nodes(
                ListTypes::Unordered,
                0,
                vec![Paragraph::new_with_nodes(true, vec![Text::new("foo").into()]).into()]
            )
            .len(),
            5
        );

        assert_eq!(
            ListItem::new_with_nodes(
                ListTypes::Unordered,
                3,
                vec![Paragraph::new_with_nodes(true, vec![Text::new("foo").into()]).into()],
            )
            .len(),
            9
        );
    }
}
