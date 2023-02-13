use crate::sd::{
    deserializer::{
        Deserializer, Node,
        Pattern::{Exact, Repeat},
        Tokenizer,
    },
    serializer::Serializer,
};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq)]
pub struct UnorderedListItem {
    level: usize,
    node: Paragraph,
}

impl UnorderedListItem {
    pub fn new(level: usize, node: Paragraph) -> Self {
        Self { level, node }
    }
}

impl Node for UnorderedListItem {
    fn len(&self) -> usize {
        self.node.len() + 3 + self.level
    }
}

impl Serializer for UnorderedListItem {
    fn serialize(&self) -> String {
        format!(
            "{}- {}\n",
            String::from(' ').repeat(self.level),
            self.node.serialize()
        )
    }
}

impl Deserializer for UnorderedListItem {
    fn deserialize(input: &str) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(level) = tokenizer.get_token_body(vec![Repeat(' ')], vec![Exact('-')]) {
            if level.chars().all(|c| c == ' ') {
                let level = level.len();
                if let Some(body) = tokenizer.get_token_body(vec![Exact(' ')], vec![Exact('\n')]) {
                    if let Some(p) = Paragraph::deserialize(body) {
                        return Some(Self::new(level, p));
                    }
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
            UnorderedListItem::deserialize("- foo\n"),
            Some(UnorderedListItem::new(
                0,
                Paragraph::from_vec(vec![Text::new("foo").into()])
            ))
        );
        assert_eq!(
            UnorderedListItem::deserialize("    - foo\n"),
            Some(UnorderedListItem::new(
                4,
                Paragraph::from_vec(vec![Text::new("foo").into()])
            ))
        );
        assert_eq!(UnorderedListItem::deserialize("  s  - foo\n"), None);
    }

    #[test]
    fn serialize() {
        assert_eq!(
            UnorderedListItem::new(0, Paragraph::from_vec(vec![Text::new("foo").into()]))
                .serialize(),
            String::from("- foo\n")
        );
        assert_eq!(
            UnorderedListItem::new(6, Paragraph::from_vec(vec![Text::new("foo").into()]))
                .serialize(),
            String::from("      - foo\n")
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            UnorderedListItem::new(3, Paragraph::from_vec(vec![Text::new("foo").into()])).len(),
            9
        );
    }
}
