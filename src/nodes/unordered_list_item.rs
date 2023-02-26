use crate::sd::{
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
    fn set_level(&mut self, level: usize) {
        self.level = level;
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
    fn new() -> Self {
        Self {
            level: 0,
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
    fn deserialize(input: &str) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(pattern_lenghs) = tokenizer.get_pattern_lenghs(vec![
            ZerroOrMore('\n'),
            ZerroOrMore(' '),
            Once('-'),
            Once(' '),
        ]) {
            let level = *pattern_lenghs.get(1).unwrap_or(&0);
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
                if let Some(mut instance) = Self::parse_branch(body) {
                    instance.set_level(level);
                    return Some(instance);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{paragraph::Paragraph, text::Text},
        sd::{
            deserializer::{Branch, Deserializer, Node},
            serializer::Serializer,
        },
    };

    use super::UnorderedListItem;

    #[test]
    fn deserialize() {
        assert_eq!(
            UnorderedListItem::deserialize("- foo"),
            Some(UnorderedListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            })
        );
        assert_eq!(
            UnorderedListItem::deserialize("    - foo"),
            Some(UnorderedListItem {
                level: 4,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            })
        );
        assert_eq!(UnorderedListItem::deserialize("  s  - foo"), None);
    }

    #[test]
    fn deserialize_with_body() {
        assert_eq!(
            UnorderedListItem::deserialize("- foo\nbla\n- bar"),
            Some(UnorderedListItem {
                level: 0,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo\nbla").into()]).into(),]
            })
        )
    }

    #[test]
    fn deserialize_with_nested_list() {
        assert_eq!(
            UnorderedListItem::deserialize("- foo\n - bar\n - baz"),
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
