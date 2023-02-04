use crate::{
    nodes::b::BNode,
    nodes::p::ParagraphNode,
    sd::deserializer::{Deserializer, Node},
    sd::serializer::Serializer,
};

/// Representation of a regular text
#[derive(Debug, PartialEq)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Text { text: text.into() }
    }
}

impl Serializer for Text {
    fn serialize(&self) -> String {
        self.text.clone()
    }
}

impl From<Text> for BNode {
    fn from(value: Text) -> Self {
        BNode::Text(value)
    }
}

impl From<Text> for ParagraphNode {
    fn from(value: Text) -> Self {
        ParagraphNode::Text(value)
    }
}

impl Deserializer for Text {
    fn deserialize(input: &str) -> Option<Self> {
        Some(Text::new(input.to_string()))
    }
}

impl Node for Text {
    fn len(&self) -> usize {
        self.text.len()
    }

    fn get_token_length(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{deserializer::Deserializer, serializer::Serializer};

    use super::Text;

    #[test]
    fn happy_path() {
        let text = Text::new("shiny text");
        assert_eq!(text.text, "shiny text".to_string());
    }

    #[test]
    fn to_string() {
        let text: String = Text::new("shiny text").serialize();
        assert_eq!(text, "shiny text".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Text::deserialize("t"), Some(Text::new("t")));
    }
}
