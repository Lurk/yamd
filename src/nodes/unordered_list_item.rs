use crate::sd::{
    deserializer::{Branch, Deserializer, FallbackNode, MaybeNode, Node},
    serializer::Serializer,
    tokenizer::{
        Pattern::{Exact, ExactRepeat, Repeat},
        Tokenizer,
    },
};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq)]
pub enum UnorderedListItemNodes {
    Paragraph(Paragraph),
}

impl From<Paragraph> for UnorderedListItemNodes {
    fn from(value: Paragraph) -> Self {
        UnorderedListItemNodes::Paragraph(value)
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
        }
    }
}

impl Serializer for UnorderedListItemNodes {
    fn serialize(&self) -> String {
        match self {
            UnorderedListItemNodes::Paragraph(node) => node.serialize(),
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
        vec![Paragraph::maybe_node()]
    }

    fn get_fallback_node() -> crate::sd::deserializer::DefinitelyNode<UnorderedListItemNodes> {
        Paragraph::fallback_node()
    }

    fn get_outer_token_length(&self) -> usize {
        2 + self.level
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
        if let Some(length) = tokenizer.get_body_start(vec![Repeat(' '), Exact('-'), Exact(' ')]) {
            let mut tokenizer = Tokenizer::new_with_match_end_of_input(input, true);
            if let Some(body) = tokenizer.get_token_body(
                vec![ExactRepeat(length - 2, ' '), Exact('-'), Exact(' ')],
                vec![
                    Exact('\n'),
                    ExactRepeat(length - 2, ' '),
                    Exact('-'),
                    Exact(' '),
                ],
            ) {
                let mut instance = Self::parse_branch(body);
                instance.set_level(length - 2);
                return Some(instance);
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
                level: 3,
                nodes: vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()]
            }
            .len(),
            8
        );
    }
}
