use crate::sd::{
    deserializer::{Deserializer, Node},
    serializer::Serializer,
    tokenizer::{
        Pattern::{Exact, ExactRepeat, Repeat},
        Tokenizer,
    },
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
        if let Some(length) = tokenizer.get_body_start(vec![Repeat(' '), Exact('-'), Exact(' ')]) {
            let mut tokenizer = Tokenizer::new_with_match_end_of_input(input, true);
            if let Some(token) = tokenizer.get_token_body(
                vec![ExactRepeat(length - 2, ' '), Exact('-'), Exact(' ')],
                vec![ExactRepeat(length - 2, ' '), Exact('-'), Exact(' ')],
            ) {
                if let Some(p) = Paragraph::deserialize(token) {
                    return Some(Self::new(length - 2, p));
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
            Some(UnorderedListItem::new(
                0,
                Paragraph::from_vec(vec![Text::new("foo").into()])
            ))
        );
        assert_eq!(
            UnorderedListItem::deserialize("    - foo"),
            Some(UnorderedListItem::new(
                4,
                Paragraph::from_vec(vec![Text::new("foo").into()])
            ))
        );
        assert_eq!(UnorderedListItem::deserialize("  s  - foo"), None);
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
