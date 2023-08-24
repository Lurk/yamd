use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq)]
pub struct Embed<'text> {
    pub url: &'text str,
    pub kind: &'text str,
    consumed_all_input: bool,
}

impl<'text> Embed<'text> {
    pub fn new(consumed_all_input: bool, kind: &'text str, url: &'text str) -> Self {
        Self {
            kind,
            url,
            consumed_all_input,
        }
    }
}

impl<'text> Node<'text> for Embed<'text> {
    fn serialize(&self) -> String {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        format!("{{{{{}|{}}}}}{end}", self.kind, self.url)
    }

    fn len(&self) -> usize {
        let end = if self.consumed_all_input { 0 } else { 2 };
        5 + self.kind.len() + self.url.len() + end
    }
}

impl<'text> Deserializer<'text> for Embed<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(embed) = matcher.get_match("{{", "}}", false) {
            let mut embed = embed.body.split('|');
            if let (Some(kind), Some(url)) = (embed.next(), embed.next()) {
                let consumed_all_input = matcher.get_match("\n\n", "", false).is_none();
                return Some(Self::new(consumed_all_input, kind, url));
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
                false,
                "youtube",
                "https://www.youtube.com/embed/wsfdjlkjsdf",
            )
            .serialize(),
            "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}\n\n"
        );
        assert_eq!(
            Embed::new(true, "youtube", "https://www.youtube.com/embed/wsfdjlkjsdf").serialize(),
            "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}"
        );
    }

    #[test]
    fn len() {
        assert_eq!(Embed::new(false, "y", "h").len(), 9);
        assert_eq!(Embed::new(true, "y", "h").len(), 7);
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Embed::deserialize_with_context(
                "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}\n\n",
                None
            ),
            Some(Embed::new(
                false,
                "youtube",
                "https://www.youtube.com/embed/wsfdjlkjsdf",
            ))
        );
        assert_eq!(
            Embed::deserialize_with_context(
                "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}",
                None
            ),
            Some(Embed::new(
                true,
                "youtube",
                "https://www.youtube.com/embed/wsfdjlkjsdf",
            ))
        );
    }
}
