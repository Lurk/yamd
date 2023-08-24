use crate::{
    toolkit::context::Context,
    toolkit::deserializer::Deserializer,
    toolkit::{matcher::Matcher, node::Node},
};

/// Representation of an anchor
#[derive(Debug, PartialEq)]
pub struct Anchor<'text> {
    pub text: &'text str,
    pub url: &'text str,
}

impl<'text> Anchor<'text> {
    pub fn new(text: &'text str, url: &'text str) -> Self {
        Anchor { text, url }
    }
}

impl<'text> Node<'text> for Anchor<'text> {
    fn serialize(&self) -> String {
        format!("[{}]({})", self.text, self.url)
    }
    fn len(&self) -> usize {
        self.text.len() + self.url.len() + 4
    }
}

impl<'text> Deserializer<'text> for Anchor<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
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
