use crate::toolkit::{
    context::Context,
    deserializer::Deserializer,
    node::Node,
    tokenizer::{
        Matcher,
        Quantifiers::{Once, ZeroOrMore},
    },
};

#[derive(Debug, PartialEq)]
pub struct Image {
    alt: String,
    url: String,
}

impl Image {
    pub fn new<S: Into<String>>(alt: S, url: S) -> Self {
        Self {
            alt: alt.into(),
            url: url.into(),
        }
    }
}

impl Node for Image {
    fn serialize(&self) -> String {
        format!("![{}]({})", self.alt, self.url)
    }
    fn len(&self) -> usize {
        self.alt.len() + self.url.len() + 5
    }
}

impl Deserializer for Image {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(alt) = matcher.get_match(
            &[ZeroOrMore('\n'), Once('!'), Once('[')],
            &[Once(']')],
            false,
        ) {
            if let Some(url) = matcher.get_match(&[Once('(')], &[Once(')')], false) {
                return Some(Self::new(alt.body, url.body));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::{deserializer::Deserializer, node::Node};

    use super::Image;

    #[test]
    fn serializer() {
        assert_eq!(Image::new('a', 'u').serialize(), String::from("![a](u)"))
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
