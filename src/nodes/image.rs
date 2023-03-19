use crate::toolkit::{
    context::Context,
    deserializer::Deserializer,
    node::Node,
    tokenizer::{
        Pattern::{Once, ZerroOrMore},
        Tokenizer,
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
    fn len(&self) -> usize {
        self.alt.len() + self.url.len() + 5
    }
    fn serialize(&self) -> String {
        format!("![{}]({})", self.alt, self.url)
    }
}

impl Deserializer for Image {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(alt_body) =
            tokenizer.get_token_body(&[ZerroOrMore('\n'), Once('!'), Once('[')], &[Once(']')])
        {
            let alt_body = alt_body.to_string();
            if let Some(url_body) = tokenizer.get_token_body(&[Once('(')], &[Once(')')]) {
                return Some(Self::new(alt_body, url_body.to_string()));
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
