use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::{
    toolkit::context::Context,
    toolkit::deserializer::Deserializer,
    toolkit::{matcher::Matcher, node::Node},
};

/// Representation of an anchor
#[derive(Debug, PartialEq, Serialize)]
pub struct Anchor {
    pub text: String,
    pub url: String,
}

impl Anchor {
    pub fn new<S: Into<String>>(text: S, url: S) -> Self {
        Anchor {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl Display for Anchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]({})", self.text, self.url)
    }
}

impl Node for Anchor {
    fn len(&self) -> usize {
        self.text.len() + self.url.len() + 4
    }
}

impl Deserializer for Anchor {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(text) = matcher.get_match("[", "]", false) {
            if let Some(url) = matcher.get_match("(", ")", false) {
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
        let a: String = Anchor::new("nice link", "https://test.io").to_string();
        assert_eq!(a, "[nice link](https://test.io)".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(Anchor::deserialize("[1](2)"), Some(Anchor::new("1", "2")));
        assert_eq!(Anchor::deserialize("[1"), None);
        assert_eq!(Anchor::deserialize("[1](2"), None);
    }

    #[test]
    fn deserilalze_with_parentesis_in_url() {
        assert_eq!(
            Anchor::deserialize(
                "[the Rope data structure](https://en.wikipedia.org/wiki/Rope_(data_structure))"
            ),
            Some(Anchor::new(
                "the Rope data structure",
                "https://en.wikipedia.org/wiki/Rope_(data_structure)"
            ))
        );
    }

    #[test]
    fn len() {
        assert_eq!(Anchor::new("a", "b").len(), 6);
    }
}
