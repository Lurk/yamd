use crate::{
    toolkit::context::Context,
    toolkit::deserializer::Deserializer,
    toolkit::{
        node::Node,
        tokenizer::{Matcher, Quantifiers::Once},
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
    fn serialize(&self) -> String {
        format!("[{}]({})", self.text, self.url)
    }
    fn len(&self) -> usize {
        self.text.len() + self.url.len() + 4
    }
}

impl Deserializer for Anchor {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(text) = matcher.get_match(&[Once('[')], &[Once(']')], false) {
            if let Some(url) = matcher.get_match(&[Once('(')], &[Once(')')], false) {
                return Some(Anchor::new(text.body, url.body));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Anchor;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let a = Anchor::new("nice link", "https://test.io");
        assert_eq!(a.text, "nice link");
        assert_eq!(a.url, "https://test.io");
    }

    #[test]
    fn serialize() {
        let a: String = Anchor::new("nice link", "https://test.io").serialize();
        assert_eq!(a, "[nice link](https://test.io)".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(Anchor::deserialize("[1](2)"), Some(Anchor::new("1", "2")));
        assert_eq!(Anchor::deserialize("[1"), None);
        assert_eq!(Anchor::deserialize("[1](2"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Anchor::new("a", "b").len(), 6);
    }
}
