use std::fmt::Display;

use crate::toolkit::{matcher::Matcher, node::Node};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Default, Clone, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl Metadata {
    pub fn new<S: Into<String>>(
        title: Option<S>,
        timestamp: Option<DateTime<FixedOffset>>,
        image: Option<S>,
        preview: Option<S>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            title: title.map(|h| h.into()),
            date: timestamp,
            image: image.map(|i| i.into()),
            preview: preview.map(|p| p.into()),
            tags,
        }
    }

    pub fn deserialize(input: &str) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(metadata) = matcher.get_match("---\n", "---", false) {
            let meta: Metadata = serde_yaml::from_str(metadata.body).unwrap_or_else(|e| {
                panic!("Failed to deserialize metadata: {}\n{}\n", metadata.body, e)
            });
            return Some(meta);
        }

        None
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.title.is_none()
            && self.date.is_none()
            && self.image.is_none()
            && self.preview.is_none()
            && self.tags.is_none()
        {
            Ok(())
        } else {
            write!(f, "---\n{}---", serde_yaml::to_string(self).unwrap())
        }
    }
}

impl Node for Metadata {
    fn len(&self) -> usize {
        self.to_string().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
            Some(vec!["tag1".to_string(), "tag2".to_string()]),
        );
        assert_eq!(
            metadata.to_string(),
            "---\ntitle: header\ndate: 2022-01-01T00:00:00+02:00\nimage: image\npreview: preview\ntags:\n- tag1\n- tag2\n---"
        );
    }

    #[test]
    fn test_len() {
        let metadata = Metadata::new(
            Some("title"),
            Some(
                DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap(),
            ),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1".to_string(), "tag2".to_string()]),
        );
        assert_eq!(metadata.len(), metadata.to_string().len());
    }

    #[test]
    fn len_with_one_tag() {
        let metadata = Metadata::new(
            Some("title"),
            Some(
                DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap(),
            ),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1".to_string()]),
        );
        assert_eq!(metadata.len(), metadata.to_string().len());
    }

    #[test]
    fn test_deserialize() {
        let metadata = Metadata::new(
            Some("title"),
            Some(
                DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap(),
            ),
            Some("image"),
            Some("preview"),
            Some(vec!["tag1".to_string(), "tag2".to_string()]),
        );
        assert_eq!(
            Metadata::deserialize(metadata.to_string().as_str()),
            Some(metadata)
        );
    }

    #[test]
    fn deserialize_empty() {
        assert_eq!(
            Metadata::deserialize("---\n---"),
            Some(Metadata::new::<&str>(None, None, None, None, None))
        );
    }

    #[test]
    fn deserialize_fail() {
        assert_eq!(Metadata::deserialize("random string"), None);
    }

    #[test]
    fn deserialize_only_with_title() {
        assert_eq!(
            Metadata::deserialize("---\ntitle: header\n---"),
            Some(Metadata::new(Some("header"), None, None, None, None))
        );
    }

    #[test]
    fn default() {
        assert_eq!(
            Metadata::default(),
            Metadata::new::<&str>(None, None, None, None, None)
        );
        assert_eq!(Metadata::default().to_string(), "");
        assert_eq!(Metadata::default().len(), 0);
    }
}
