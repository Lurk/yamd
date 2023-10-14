use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq, Serialize)]
pub struct Image {
    pub alt: String,
    pub src: String,
    #[serde(skip_serializing)]
    consumed_all_input: bool,
}

impl Image {
    pub fn new<S: Into<String>>(consumed_all_input: bool, alt: S, url: S) -> Self {
        Self {
            alt: alt.into(),
            src: url.into(),
            consumed_all_input,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = if self.consumed_all_input {
            "\n"
        } else {
            "\n\n"
        };

        write!(f, "![{}]({}){}", self.alt, self.src, end)
    }
}

impl Node for Image {
    fn len(&self) -> usize {
        let end = if self.consumed_all_input { 1 } else { 2 };
        self.alt.len() + self.src.len() + 5 + end
    }
}

impl Deserializer for Image {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(alt) = matcher.get_match("![", "]", false) {
            if let Some(url) = matcher.get_match("(", ")\n", false) {
                let consumed_all_input = matcher.get_match("\n", "", false).is_none();
                return Some(Self::new(consumed_all_input, alt.body, url.body));
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
        assert_eq!(
            Image::new(true, 'a', 'u').to_string(),
            String::from("![a](u)\n")
        );
        assert_eq!(
            Image::new(false, 'a', 'u').to_string(),
            String::from("![a](u)\n\n")
        )
    }

    #[test]
    fn len() {
        assert_eq!(Image::new(true, 'a', 'u').len(), 8);
        assert_eq!(Image::new(false, 'a', 'u').len(), 9);
    }

    #[test]
    fn deserializer() {
        assert_eq!(
            Image::deserialize("![alt](url)\n"),
            Some(Image::new(true, "alt", "url"))
        );
        assert_eq!(
            Image::deserialize("![alt](url)\n\n"),
            Some(Image::new(false, "alt", "url"))
        );

        assert_eq!(Image::deserialize("![alt](url"), None);
        assert_eq!(Image::deserialize("[alt](url)"), None);
    }
}
