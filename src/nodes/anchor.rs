use crate::{
    nodes::paragraph::ParagraphNodes,
    sd::deserializer::{Deserializer, Node, Tokenizer},
    sd::serializer::Serializer,
};

/// Representation of an anchor
#[derive(Debug, PartialEq)]
pub struct Anchor {
    text: String,
    url: String,
}

impl Anchor {
    pub fn new<S: Into<String>>(url: S, text: S) -> Self {
        Anchor {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl Serializer for Anchor {
    fn serialize(&self) -> String {
        format!("[{}]({})", self.text, self.url)
    }
}

impl From<Anchor> for ParagraphNodes {
    fn from(value: Anchor) -> Self {
        ParagraphNodes::A(value)
    }
}

impl Node for Anchor {
    fn len(&self) -> usize {
        self.text.len() + self.url.len() + 4
    }
}

impl Deserializer for Anchor {
    fn deserialize(input: &str) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(text_part) = tokenizer.get_token_body(vec!['['], vec![']']) {
            let text_part = text_part.to_string();
            if let Some(url_part) = tokenizer.get_token_body(vec!['('], vec![')']) {
                return Some(Anchor::new(url_part.to_string(), text_part));
            }
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

    use super::Anchor;

    #[test]
    fn happy_path() {
        let a = Anchor::new("https://test.io", "nice link");
        assert_eq!(a.text, "nice link");
        assert_eq!(a.url, "https://test.io");
    }

    #[test]
    fn to_string_with_text() {
        let a: String = Anchor::new("https://test.io", "nice link").serialize();
        assert_eq!(a, "[nice link](https://test.io)".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Anchor::deserialize("[1](2)"), Some(Anchor::new("2", "1")))
    }

    #[test]
    fn len() {
        assert_eq!(Anchor::new("a", "b").len(), 6);
    }
}
