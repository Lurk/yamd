use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Embed {
    pub args: String,
    pub kind: String,
}

impl Embed {
    pub fn new<S: Into<String>>(kind: S, args: S) -> Self {
        Self {
            kind: kind.into(),
            args: args.into(),
        }
    }
}

impl Display for Embed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{{{}|{}}}}}", self.kind, self.args)
    }
}

impl Node for Embed {
    fn len(&self) -> usize {
        5 + self.kind.len() + self.args.len()
    }
}

impl Deserializer for Embed {
    fn deserialize_with_context(
        input: &str,
        _: Option<crate::toolkit::context::Context>,
    ) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(embed) = matcher.get_match("{{", "}}", false) {
            if let Some((kind, args)) = embed.body.split_once('|') {
                return Some(Self::new(kind.to_string(), args.to_string()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::embed::Embed,
        toolkit::{deserializer::Deserializer, node::Node},
    };

    #[test]
    fn serializer() {
        assert_eq!(
            Embed::new("youtube", "https://www.youtube.com/embed/wsfdjlkjsdf",).to_string(),
            "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}"
        );
    }

    #[test]
    fn len() {
        assert_eq!(Embed::new("y", "h",).len(), 7);
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Embed::deserialize_with_context(
                "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}",
                None
            ),
            Some(Embed::new(
                "youtube",
                "https://www.youtube.com/embed/wsfdjlkjsdf",
            ))
        );
    }

    #[test]
    fn failed_deserialize() {
        assert_eq!(Embed::deserialize_with_context("{{youtube}}", None), None);
    }
}
