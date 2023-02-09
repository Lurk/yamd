use crate::sd::{
    deserializer::{Deserializer, Node, Tokenizer},
    serializer::Serializer,
};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq)]
pub enum UnorderedListItemNodes {
    P(Paragraph),
}

impl Node for UnorderedListItemNodes {
    fn len(&self) -> usize {
        match self {
            UnorderedListItemNodes::P(node) => node.len(),
        }
    }
}

impl Serializer for UnorderedListItemNodes {
    fn serialize(&self) -> String {
        match self {
            UnorderedListItemNodes::P(node) => node.serialize(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct UnorderedListItem {
    node: UnorderedListItemNodes,
}

impl UnorderedListItem {
    pub fn new<I: Into<UnorderedListItemNodes>>(node: I) -> Self {
        Self { node: node.into() }
    }
}

impl Node for UnorderedListItem {
    fn len(&self) -> usize {
        self.node.len() + 3
    }
}

impl Serializer for UnorderedListItem {
    fn serialize(&self) -> String {
        format!("- {}\n", self.node.serialize())
    }
}

impl Deserializer for UnorderedListItem {
    fn deserialize(input: &str) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_token_body(vec!['-', ' '], vec!['\n']) {
            if let Some(p) = Paragraph::deserialize(body) {
                return Some(Self::new(p));
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
            deserializer::{Branch, Deserializer},
            serializer::Serializer,
        },
    };

    use super::UnorderedListItem;

    #[test]
    fn deserialize() {
        assert_eq!(
            UnorderedListItem::deserialize("- foo\n"),
            Some(UnorderedListItem::new(Paragraph::from_vec(vec![
                Text::new("foo").into()
            ])))
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            UnorderedListItem::new(Paragraph::from_vec(vec![Text::new("foo").into()])).serialize(),
            String::from("- foo\n")
        );
    }
}
