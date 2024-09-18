use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Image {
    pub alt: String,
    pub src: String,
}

impl Image {
    pub fn new<S: Into<String>>(alt: S, src: S) -> Self {
        Self {
            alt: alt.into(),
            src: src.into(),
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "![{}]({})", self.alt, self.src)
    }
}
