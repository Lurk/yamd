use crate::{
    nodes::b::BNode,
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

impl From<I> for BNode {
    fn from(value: I) -> Self {
        BNode::I(value)
    }
}

impl From<I> for ParagraphNode {
    fn from(value: I) -> Self {
        ParagraphNode::I(value)
    }
}

impl Node for I {}

impl Deserializer for I {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = Tokenizer::new(input, start_position);
        if let Some(body) = chars.get_token_body(vec!['_'], vec!['_']) {
            return Some((
                I::new(body.to_string().replace('\n', "")),
                chars.get_next_position(),
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{deserializer::Deserializer, serializer::Serializer};

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
        assert_eq!(I::deserialize("_italic_", 0), Some((I::new("italic"), 8)));
        assert_eq!(
            I::deserialize("not_italic_not", 3),
            Some((I::new("italic"), 11))
        );
        assert_eq!(
            I::deserialize("not_it alic_not", 3),
            Some((I::new("it alic"), 12))
        );
        assert_eq!(I::deserialize("not italic_not", 3), None);
        assert_eq!(I::deserialize("*italic not", 0), None);
        assert_eq!(I::deserialize("_ita\nlic_", 0), Some((I::new("italic"), 9)));
        assert_eq!(I::deserialize("_ita\n\nlic_", 0), None);
    }
}
