use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Image {
    pub alt: String,
    pub src: String,
}

impl Image {
    pub fn new<S: Into<String>>(alt: S, src: S) -> Self {
        Self {
            alt: alt.into(),
            src: src.into(),
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "![{}]({})", self.alt, self.src)
    }
}

impl Node for Image {
    fn len(&self) -> usize {
        self.alt.len() + self.src.len() + 5
    }
}

impl Deserializer for Image {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(alt) = matcher.get_match("![", "]", false) {
            if let Some(url) = matcher.get_match("(", ")", false) {
                return Some(Self::new(alt.body, url.body));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

    #[test]
    fn serializer() {
        assert_eq!(Image::new('a', 'u').to_string(), String::from("![a](u)"));
    }

    #[test]
    fn len() {
        assert_eq!(Image::new('a', 'u').len(), 7);
    }

    #[test]
    fn deserializer() {
        assert_eq!(
            Image::deserialize("![alt](url)"),
            Some(Image::new("alt", "url"))
        );
        assert_eq!(Image::deserialize("![alt](url"), None);
        assert_eq!(Image::deserialize("[alt](url)"), None);
    }
}
