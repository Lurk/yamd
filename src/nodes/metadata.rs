use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};
use chrono::{DateTime, FixedOffset};

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub header: Option<String>,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub image: Option<String>,
    pub preview: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Metadata {
    pub fn new<S: Into<String>>(
        header: Option<S>,
        timestamp: Option<DateTime<FixedOffset>>,
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

impl Node<'_> for Metadata {
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

impl<'text> Deserializer<'text> for Metadata {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(metadata) = matcher.get_match("", "^^^\n\n", false) {
            let mut meta = Self::new::<&str>(None, None, None, None, None);
            metadata.body.split('\n').for_each(|line| {
                if line.starts_with("header: ") {
                    meta.header = Some(line.replace("header: ", ""));
                } else if line.starts_with("timestamp: ") {
                    meta.timestamp = DateTime::parse_from_str(
                        line.replace("timestamp: ", "").as_str(),
                        "%Y-%m-%d %H:%M:%S %z",
                    )
                    .ok();
                } else if line.starts_with("image: ") {
                    meta.image = Some(line.replace("image: ", ""));
                } else if line.starts_with("preview: ") {
                    meta.preview = Some(line.replace("preview: ", ""));
                } else if line.starts_with("tags: ") {
                    meta.tags = Some(
                        line.replace("tags: ", "")
                            .split(", ")
                            .map(|tag| tag.to_string())
                            .collect(),
                    );
                }
            });
            return Some(meta);
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
            Some(
                DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap(),
            ),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1", "tag2"]),
        );
        assert_eq!(
            metadata.serialize(),
            "header: header\ntimestamp: 2022-01-01 00:00:00 +02:00\nimage: image\npreview: preview\ntags: tag1, tag2\n^^^\n\n"
        );
    }

    #[test]
    fn test_len() {
        let metadata = Metadata::new(
            Some("header"),
            Some(
                DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap(),
            ),
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
            Some(
                DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap(),
            ),
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
            Some(
                DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap(),
            ),
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

    #[test]
    fn deserialize_only_with_header() {
        assert_eq!(
            Metadata::deserialize("header: header\n^^^\n\n"),
            Some(Metadata::new(Some("header"), None, None, None, None))
        );
    }

    #[test]
    fn deserialize_wrong_date() {
        assert_eq!(
            Metadata::deserialize("timestamp: 2022-01-01 00:00:00\n^^^\n\n"),
            Some(Metadata::new::<&str>(None, None, None, None, None))
        );
    }
}
