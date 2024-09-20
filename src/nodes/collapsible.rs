use std::fmt::Display;

use serde::Serialize;

use super::YamdNodes;

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Collapsible {
    pub title: String,
    pub body: Vec<YamdNodes>,
}

impl Collapsible {
    pub fn new<S: Into<String>>(title: S, body: Vec<YamdNodes>) -> Self {
        Self {
            body,
            title: title.into(),
        }
    }
}

impl Display for Collapsible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{% {title}\n{nodes}\n%}}",
            title = self.title,
            nodes = self
                .body
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}
