use crate::{
    toolkit::context::Context,
    toolkit::deserializer::Deserializer,
    toolkit::{
        node::Node,
        tokenizer::{Pattern::Once, Tokenizer},
    },
};

/// Representation of an anchor
#[derive(Debug, PartialEq)]
pub struct Anchor {
    text: String,
    url: String,
}

impl Anchor {
    pub fn new<S: Into<String>>(text: S, url: S) -> Self {
        Anchor {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl Node for Anchor {
    fn len(&self) -> usize {
        self.text.len() + self.url.len() + 4
    }
    fn serialize(&self) -> String {
        format!("[{}]({})", self.text, self.url)
    }
}

impl Deserializer for Anchor {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(text_part) = tokenizer.get_token_body(&[Once('[')], &[Once(']')]) {
            let text_part = text_part.to_string();
            if let Some(url_part) = tokenizer.get_token_body(&[Once('(')], &[Once(')')]) {
                return Some(Anchor::new(text_part, url_part.to_string()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::{deserializer::Deserializer, node::Node};

    use super::Anchor;

    #[test]
    fn happy_path() {
        let a = Anchor::new("nice link", "https://test.io");
        assert_eq!(a.text, "nice link");
        assert_eq!(a.url, "https://test.io");
    }

    #[test]
    fn to_string_with_text() {
        let a: String = Anchor::new("nice link", "https://test.io").serialize();
        assert_eq!(a, "[nice link](https://test.io)".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Anchor::deserialize("[1](2)"), Some(Anchor::new("1", "2")))
    }

    #[test]
    fn len() {
        assert_eq!(Anchor::new("a", "b").len(), 6);
    }
}
