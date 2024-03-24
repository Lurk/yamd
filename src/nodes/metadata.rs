use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::toolkit::{context::Context, parser::Parse};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
}

impl Metadata {
    pub fn new<S: Into<String>>(
        title: Option<S>,
        date: Option<DateTime<FixedOffset>>,
        image: Option<S>,
        preview: Option<S>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            title: title.map(|h| h.into()),
            date,
            image: image.map(|i| i.into()),
            preview: preview.map(|p| p.into()),
            is_draft: None,
            tags,
        }
    }
}

impl Parse for Metadata {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        if input[current_position..].starts_with("---\n") {
            let start = current_position + 4;
            if let Some(end) = input[start..].find("\n---") {
                let meta: Metadata = serde_yaml::from_str(&input[start..start + end]).ok()?;
                return Some((meta, end + 8));
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize() {
        let metadata = Metadata::new(
            Some("header"),
            Some(DateTime::parse_from_rfc3339("2022-01-01T00:00:00+02:00").unwrap()),
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
    fn test_parse() {
        let metadata = Metadata {
            title: Some("title".to_string()),
            date: Some(DateTime::parse_from_rfc3339("2022-12-30T20:33:55+01:00").unwrap()),
            image: Some("image".to_string()),
            preview: Some("preview".to_string()),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            is_draft: Some(true),
        };
        let str = "---\ntitle: title\ndate: 2022-12-30T20:33:55+01:00\nimage: image\npreview: preview\ntags:\n- tag1\n- tag2\nis_draft: true\n---";
        assert_eq!(Metadata::parse(str, 0, None), Some((metadata, str.len())));
    }

    #[test]
    fn parse_empty() {
        assert_eq!(
            Metadata::parse("---\n\n---", 0, None),
            Some((
                Metadata {
                    title: None,
                    date: None,
                    image: None,
                    preview: None,
                    tags: None,
                    is_draft: None,
                },
                8
            ))
        );
    }

    #[test]
    fn parse_fail() {
        assert_eq!(Metadata::parse("random string", 0, None), None);
        assert_eq!(Metadata::parse("---\nrandom string---", 0, None), None);
    }

    #[test]
    fn parse_only_with_title() {
        assert_eq!(
            Metadata::parse("---\ntitle: header\n---", 0, None),
            Some((
                Metadata {
                    title: Some("header".to_string()),
                    preview: None,
                    date: None,
                    image: None,
                    tags: None,
                    is_draft: None,
                },
                21
            ))
        );
    }

    #[test]
    fn default() {
        assert_eq!(
            Metadata::default(),
            Metadata::new::<&str>(None, None, None, None, None)
        );
        assert_eq!(Metadata::default().to_string(), "");
    }

    #[test]
    fn deserialize_with_quotes() {
        let input = "---\ntitle: \"header\"\n---";
        let m = Metadata::parse(input, 0, None);
        assert_eq!(input.len(), m.unwrap().1);
    }
}
