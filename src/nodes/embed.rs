use crate::toolkit::{
    deserializer::Deserializer, matcher::Matcher, node::Node, pattern::Quantifiers::*,
};

#[derive(Debug, PartialEq)]
pub struct Embed {
    url: String,
    kind: String,
    consumed_all_input: bool,
}

impl Embed {
    pub fn new<S: Into<String>>(kind: S, url: S, consumed_all_input: bool) -> Self {
        Self {
            kind: kind.into(),
            url: url.into(),
            consumed_all_input,
        }
    }
}

impl Node for Embed {
    fn serialize(&self) -> String {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        format!("{{{{{}|{}}}}}{end}", self.kind, self.url)
    }

    fn len(&self) -> usize {
        let end = if self.consumed_all_input { 0 } else { 2 };
        5 + self.kind.len() + self.url.len() + end
    }
}

impl Deserializer for Embed {
    fn deserialize_with_context(
        input: &str,
        _: Option<crate::toolkit::context::Context>,
    ) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(embed) =
            matcher.get_match(&[RepeatTimes(2, '{')], &[RepeatTimes(2, '}')], false)
        {
            let mut embed = embed.body.split('|');
            if let (Some(kind), Some(url)) = (embed.next(), embed.next()) {
                let consumed_all_input = matcher
                    .get_match(&[RepeatTimes(2, '\n')], &[], false)
                    .is_none();
                return Some(Self::new(
                    kind.to_string(),
                    url.to_string(),
                    consumed_all_input,
                ));
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
            Embed::new(
                "youtube",
                "https://www.youtube.com/embed/wsfdjlkjsdf",
                false,
            )
            .serialize(),
            "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}\n\n"
        );
        assert_eq!(
            Embed::new("youtube", "https://www.youtube.com/embed/wsfdjlkjsdf", true).serialize(),
            "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}"
        );
    }

    #[test]
    fn len() {
        assert_eq!(Embed::new("y", "h", false,).len(), 9);
        assert_eq!(Embed::new("y", "h", true).len(), 7);
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Embed::deserialize_with_context(
                "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}\n\n",
                None
            ),
            Some(Embed::new(
                "youtube",
                "https://www.youtube.com/embed/wsfdjlkjsdf",
                false,
            ))
        );
        assert_eq!(
            Embed::deserialize_with_context(
                "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}",
                None
            ),
            Some(Embed::new(
                "youtube",
                "https://www.youtube.com/embed/wsfdjlkjsdf",
                true,
            ))
        );
    }
}
