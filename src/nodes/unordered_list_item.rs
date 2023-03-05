use crate::sd::{
    context::ContextValues,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode, Node},
    serializer::Serializer,
    tokenizer::{
        Pattern::{Once, RepeatTimes, ZerroOrMore},
        Tokenizer,
    },
};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq)]
pub enum UnorderedListItemNodes {
    Paragraph(Paragraph),
    List(UnorderedListItem),
}

impl From<Paragraph> for UnorderedListItemNodes {
    fn from(value: Paragraph) -> Self {
        UnorderedListItemNodes::Paragraph(value)
    }
}

impl From<UnorderedListItem> for UnorderedListItemNodes {
    fn from(value: UnorderedListItem) -> Self {
        UnorderedListItemNodes::List(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct UnorderedListItem {
    level: usize,
    nodes: Vec<UnorderedListItemNodes>,
}

impl UnorderedListItem {
    fn get_level_from_context(ctx: &Option<ContextValues>) -> usize {
        match ctx {
            Some(value) => match value.get_usize_value() {
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

impl Branch<UnorderedListItemNodes> for UnorderedListItem {
    fn new(ctx: &Option<ContextValues>) -> Self {
        Self {
            level: Self::get_level_from_context(ctx),
            nodes: vec![],
        }
    }

    fn push<CanBeNode: Into<UnorderedListItemNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn from_vec(nodes: Vec<UnorderedListItemNodes>) -> Self {
        Self { level: 0, nodes }
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<UnorderedListItemNodes>> {
        vec![UnorderedListItem::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<UnorderedListItemNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        let add = if self.level > 0 { 1 } else { 0 };
        2 + self.level + add
    }
}

impl Node for UnorderedListItem {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }

    fn context(&self) -> Option<ContextValues> {
        Some(self.level.into())
    }
}

impl Serializer for UnorderedListItem {
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

impl Deserializer for UnorderedListItem {
    fn deserialize(input: &str, ctx: Option<ContextValues>) -> Option<Self> {
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
        nodes::{paragraph::Paragraph, text::Text},
        sd::{
            context::ContextValues,
            deserializer::{Branch, Deserializer, Node},
            serializer::Serializer,
        },
    };

    use super::UnorderedListItem;

    #[test]
    fn deserialize() {
        assert_eq!(
            UnorderedListItem::deserialize("- foo", None),
            Some(UnorderedListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            })
        );
        assert_eq!(
            UnorderedListItem::deserialize("    - foo", Some(ContextValues::Usize(3))),
            Some(UnorderedListItem {
                level: 4,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            })
        );
        assert_eq!(UnorderedListItem::deserialize("  s  - foo", None), None);
    }

    #[test]
    fn deserialize_with_body() {
        assert_eq!(
            UnorderedListItem::deserialize("- foo\nbla\n- bar", None),
            Some(UnorderedListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo\nbla").into()]).into()]
            })
        )
    }

    #[test]
    fn deserialize_with_nested_list() {
        assert_eq!(
            UnorderedListItem::deserialize("- foo\n - bar\n - baz", None),
            Some(UnorderedListItem {
                level: 0,
                nodes: vec![
                    Paragraph::from_vec(vec![Text::new("foo").into()]).into(),
                    UnorderedListItem {
                        level: 1,
                        nodes: vec![Paragraph::from_vec(vec![Text::new("bar").into()]).into()]
                    }
                    .into(),
                    UnorderedListItem {
                        level: 1,
                        nodes: vec![Paragraph::from_vec(vec![Text::new("baz").into()]).into()]
                    }
                    .into()
                ]
            })
        );
    }

    #[test]
    fn deserialize_with_deeply_nested_list() {
        assert_eq!(
            UnorderedListItem::deserialize("- level 0\n - level 1\n  - level 2\n - level 1", None),
            Some(UnorderedListItem {
                level: 0,
                nodes: vec![
                    Paragraph::from_vec(vec![Text::new("level 0").into()]).into(),
                    UnorderedListItem {
                        level: 1,
                        nodes: vec![
                            Paragraph::from_vec(vec![Text::new("level 1").into()]).into(),
                            UnorderedListItem {
                                level: 2,
                                nodes: vec![
                                    Paragraph::from_vec(vec![Text::new("level 2").into()]).into()
                                ]
                            }
                            .into()
                        ]
                    }
                    .into(),
                    UnorderedListItem {
                        level: 1,
                        nodes: vec![Paragraph::from_vec(vec![Text::new("level 1").into()]).into()]
                    }
                    .into()
                ]
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            UnorderedListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            }
            .serialize(),
            String::from("- foo")
        );
        assert_eq!(
            UnorderedListItem {
                level: 6,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            }
            .serialize(),
            String::from("      - foo")
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            UnorderedListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            }
            .len(),
            5
        );
        assert_eq!(
            UnorderedListItem {
                level: 3,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            }
            .len(),
            9
        );
    }
}
