use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq)]
pub struct Metadata {
    header: Option<String>,
    timestamp: Option<usize>,
    image: Option<String>,
    preview: Option<String>,
    tags: Option<Vec<String>>,
}

impl Metadata {
    pub fn new<S: Into<String>>(
        header: Option<S>,
        timestamp: Option<usize>,
        image: Option<S>,
        preview: Option<S>,
        tags: Option<Vec<S>>,
    ) -> Self {
        Self {
            header: header.map(|h| h.into()),
            timestamp,
            image: image.map(|i| i.into()),
            preview: preview.map(|p| p.into()),
            tags: tags.map(|t| t.into_iter().map(|tag| tag.into()).collect()),
        }
    }
}

impl Node for Metadata {
    fn serialize(&self) -> String {
        format!(
            "{}{}{}{}{}^^^\n\n",
            self.header
                .as_ref()
                .map_or("".to_string(), |h| format!("header: {h}\n")),
            self.timestamp
                .as_ref()
                .map_or("".to_string(), |t| format!("timestamp: {t}\n")),
            self.image
                .as_ref()
                .map_or("".to_string(), |i| format!("image: {i}\n")),
            self.preview
                .as_ref()
                .map_or("".to_string(), |p| format!("preview: {p}\n")),
            self.tags
                .as_ref()
                .map_or("".to_string(), |t| format!("tags: {}\n", t.join(", "))),
        )
    }

    fn len(&self) -> usize {
        5 + self.header.as_ref().map_or(0, |h| h.len() + 9)
            + self
                .timestamp
                .as_ref()
                .map_or(0, |t| t.to_string().len() + 12)
            + self.image.as_ref().map_or(0, |i| i.len() + 8)
            + self.preview.as_ref().map_or(0, |p| p.len() + 10)
            + self.tags.as_ref().map_or(0, |t| {
                t.iter().map(|tag| tag.len()).sum::<usize>()
                    + 7
                    + if t.len() > 1 { (t.len() - 1) * 2 } else { 0 }
            })
    }
}

impl Deserializer for Metadata {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(metadata) = matcher.get_match("", "^^^\n\n", false) {
            let mut inner_matcher = Matcher::new(metadata.body);
            let header = inner_matcher
                .get_match("header: ", "\n", false)
                .map(|h| h.body);
            let timestamp = inner_matcher
                .get_match("timestamp: ", "\n", false)
                .map(|t| t.body.parse::<usize>().unwrap_or(0));
            let image = inner_matcher
                .get_match("image: ", "\n", false)
                .map(|i| i.body);
            let preview = inner_matcher
                .get_match("preview: ", "\n", false)
                .map(|p| p.body);
            let tags = inner_matcher
                .get_match("tags: ", "\n", false)
                .map(|t| t.body.split(", ").collect());
            return Some(Self::new(header, timestamp, image, preview, tags));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let metadata = Metadata::new(
            Some("header"),
            Some(1672428835705),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1", "tag2"]),
        );
        assert_eq!(
            metadata.serialize(),
            "header: header\ntimestamp: 1672428835705\nimage: image\npreview: preview\ntags: tag1, tag2\n^^^\n\n"
        );
    }

    #[test]
    fn test_len() {
        let metadata = Metadata::new(
            Some("header"),
            Some(1672428835705),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1", "tag2"]),
        );
        assert_eq!(metadata.len(), metadata.serialize().len());
    }

    #[test]
    fn len_with_one_tag() {
        let metadata = Metadata::new(
            Some("header"),
            Some(1672428835705),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1"]),
        );
        assert_eq!(metadata.len(), metadata.serialize().len());
    }

    #[test]
    fn test_deserialize() {
        let metadata = Metadata::new(
            Some("header"),
            Some(1672428835705),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1", "tag2"]),
        );
        assert_eq!(
            Metadata::deserialize(metadata.serialize().as_str()),
            Some(metadata)
        );
    }

    #[test]
    fn deserialize_empty() {
        assert_eq!(
            Metadata::deserialize("^^^\n\n"),
            Some(Metadata::new::<&str>(None, None, None, None, None))
        );
    }

    #[test]
    fn deserialize_fail() {
        assert_eq!(Metadata::deserialize("random string"), None);
    }
}
