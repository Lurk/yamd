use crate::sd::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode, Node},
    serializer::Serializer,
    tokenizer::{
        Pattern::{Once, RepeatTimes, ZerroOrMore},
        Tokenizer,
    },
};

use super::{list::List, paragraph::Paragraph};

#[derive(Debug, PartialEq)]
pub enum UnorderedListItemNodes {
    Paragraph(Paragraph),
    List(List),
}

impl From<Paragraph> for UnorderedListItemNodes {
    fn from(value: Paragraph) -> Self {
        UnorderedListItemNodes::Paragraph(value)
    }
}

impl From<List> for UnorderedListItemNodes {
    fn from(value: List) -> Self {
        UnorderedListItemNodes::List(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct ListItem {
    level: usize,
    nodes: Vec<UnorderedListItemNodes>,
}

impl ListItem {
    fn get_level_from_context(ctx: &Option<Context>) -> usize {
        match ctx {
            Some(value) => match value.get_usize_value("level") {
                Some(parrent_level) => parrent_level + 1,
                None => 0,
            },
            None => 0,
        }
    }
}

impl Node for UnorderedListItemNodes {
    fn len(&self) -> usize {
        match self {
            UnorderedListItemNodes::Paragraph(node) => node.len(),
            UnorderedListItemNodes::List(node) => node.len(),
        }
    }
}

impl Serializer for UnorderedListItemNodes {
    fn serialize(&self) -> String {
        match self {
            UnorderedListItemNodes::Paragraph(node) => node.serialize(),
            UnorderedListItemNodes::List(node) => node.serialize(),
        }
    }
}

impl Branch<UnorderedListItemNodes> for ListItem {
    fn new(ctx: &Option<Context>) -> Self {
        Self {
            level: Self::get_level_from_context(ctx),
            nodes: vec![],
        }
    }

    fn push<CanBeNode: Into<UnorderedListItemNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn from_vec(nodes: Vec<UnorderedListItemNodes>, ctx: Option<Context>) -> Self {
        Self {
            level: Self::get_level_from_context(&ctx),
            nodes,
        }
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<UnorderedListItemNodes>> {
        vec![List::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<UnorderedListItemNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        let add = if self.level > 0 { 1 } else { 0 };
        2 + self.level + add
    }
}

impl Node for ListItem {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }

    fn context(&self) -> Option<Context> {
        let mut ctx = Context::new();
        ctx.add("level", self.level);
        Some(ctx)
    }
}

impl Serializer for ListItem {
    fn serialize(&self) -> String {
        format!(
            "{}- {}",
            String::from(' ').repeat(self.level),
            self.nodes
                .iter()
                .map(|element| { element.serialize() })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Deserializer for ListItem {
    fn deserialize(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = Self::get_level_from_context(&ctx);
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_token_body_with_options(
            vec![
                ZerroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('-'),
                Once(' '),
            ],
            vec![Once('\n'), RepeatTimes(level, ' '), Once('-'), Once(' ')],
            true,
        ) {
            return Self::parse_branch(body, &ctx);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{list::List, paragraph::Paragraph, text::Text},
        sd::{
            context::Context,
            deserializer::{Branch, Deserializer, Node},
            serializer::Serializer,
        },
    };

    use super::ListItem;

    #[test]
    fn deserialize() {
        assert_eq!(
            ListItem::deserialize("- foo", None),
            Some(ListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()], None).into()]
            })
        );
        let mut ctx = Context::new();
        ctx.add("level", 3);
        assert_eq!(
            ListItem::deserialize("    - foo", Some(ctx)),
            Some(ListItem {
                level: 4,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()], None).into()]
            })
        );
        assert_eq!(ListItem::deserialize("  s  - foo", None), None);
    }

    #[test]
    fn deserialize_with_body() {
        assert_eq!(
            ListItem::deserialize("- foo\nbla\n- bar", None),
            Some(ListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo\nbla").into()], None).into()]
            })
        )
    }

    #[test]
    fn deserialize_with_nested_list() {
        assert_eq!(
            ListItem::deserialize("- foo\n - bar\n - baz", None),
            Some(ListItem {
                level: 0,
                nodes: vec![
                    Paragraph::from_vec(vec![Text::new("foo").into()], None).into(),
                    List::from_vec(
                        vec![
                            ListItem {
                                level: 1,
                                nodes: vec![Paragraph::from_vec(
                                    vec![Text::new("bar").into()],
                                    None
                                )
                                .into()]
                            }
                            .into(),
                            ListItem {
                                level: 1,
                                nodes: vec![Paragraph::from_vec(
                                    vec![Text::new("baz").into()],
                                    None
                                )
                                .into()]
                            }
                            .into()
                        ],
                        None
                    )
                    .into()
                ]
            })
        );
    }

    #[test]
    fn deserialize_with_deeply_nested_list() {
        let mut ctx_for_level_2 = Context::new();
        ctx_for_level_2.add("t", '-');
        ctx_for_level_2.add("level", 2);
        let mut ctx_for_level_1 = Context::new();
        ctx_for_level_1.add("t", '-');
        ctx_for_level_1.add("level", 1);

        assert_eq!(
            ListItem::deserialize("- level 0\n - level 1\n  - level 2\n - level 1", None),
            Some(ListItem {
                level: 0,
                nodes: vec![
                    Paragraph::from_vec(vec![Text::new("level 0").into()], None).into(),
                    List::from_vec(
                        vec![
                            ListItem {
                                level: 1,
                                nodes: vec![
                                    Paragraph::from_vec(vec![Text::new("level 1").into()], None)
                                        .into(),
                                    List::from_vec(
                                        vec![ListItem {
                                            level: 2,
                                            nodes: vec![Paragraph::from_vec(
                                                vec![Text::new("level 2").into()],
                                                Some(ctx_for_level_2)
                                            )
                                            .into()]
                                        }
                                        .into()],
                                        Some(ctx_for_level_1)
                                    )
                                    .into()
                                ]
                            }
                            .into(),
                            ListItem {
                                level: 1,
                                nodes: vec![Paragraph::from_vec(
                                    vec![Text::new("level 1").into()],
                                    None
                                )
                                .into()]
                            }
                            .into()
                        ],
                        None
                    )
                    .into()
                ]
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            ListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()], None).into()]
            }
            .serialize(),
            String::from("- foo")
        );
        assert_eq!(
            ListItem {
                level: 6,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()], None).into()]
            }
            .serialize(),
            String::from("      - foo")
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()], None).into()]
            }
            .len(),
            5
        );
        assert_eq!(
            ListItem {
                level: 3,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()], None).into()]
            }
            .len(),
            9
        );
    }
}
