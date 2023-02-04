use crate::{
    nodes::bold::BoldNodes,
    nodes::p::ParagraphNode,
    sd::deserializer::{Deserializer, Node, Tokenizer},
    sd::serializer::Serializer,
};

/// Representation of an Italic text
#[derive(Debug, PartialEq)]
pub struct I {
    text: String,
}

impl I {
    pub fn new<S: Into<String>>(text: S) -> Self {
        I { text: text.into() }
    }
}

impl Serializer for I {
    fn serialize(&self) -> String {
        format!("_{}_", self.text)
    }
}

impl From<I> for BoldNodes {
    fn from(value: I) -> Self {
        BoldNodes::I(value)
    }
}

impl From<I> for ParagraphNode {
    fn from(value: I) -> Self {
        ParagraphNode::I(value)
    }
}

impl Node for I {
    fn len(&self) -> usize {
        self.text.len() + 2
    }

    fn get_token_length(&self) -> usize {
        0
    }
}

impl Deserializer for I {
    fn deserialize(input: &str) -> Option<Self> {
        let mut chars = Tokenizer::new(input);
        if let Some(body) = chars.get_token_body(vec!['_'], vec!['_']) {
            return Some(I::new(body.to_string().replace('\n', "")));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{
        deserializer::{Deserializer, Node},
        serializer::Serializer,
    };

    use super::I;

    #[test]
    fn happy_path() {
        let i = I::new("italic");
        assert_eq!(i.text, "italic".to_string());
    }

    #[test]
    fn to_string() {
        let i: String = I::new("italic").serialize();
        assert_eq!(i, "_italic_".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(I::deserialize("_italic_"), Some(I::new("italic")));
        assert_eq!(I::deserialize("_italic_not"), Some(I::new("italic")));
        assert_eq!(I::deserialize("_it alic_not"), Some(I::new("it alic")));
        assert_eq!(I::deserialize("not italic_not"), None);
        assert_eq!(I::deserialize("*italic not"), None);
        assert_eq!(I::deserialize("_ita\nlic_"), Some(I::new("italic")));
        assert_eq!(I::deserialize("_ita\n\nlic_"), None);
    }

    #[test]
    fn len() {
        assert_eq!(I::new("i").len(), 3);
    }
}
