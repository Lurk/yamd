use std::{fmt::Display, ops::Deref};

use serde::Serialize;

/// # Metadata
///
/// Can be only in the beginning of the document surrounded by [Minus](type@crate::lexer::TokenKind::Minus)
/// of length 3 followed by [EOL](type@crate::lexer::TokenKind::Eol) and [EOL](type@crate::lexer::TokenKind::Eol)
/// followed by [Minus](type@crate::lexer::TokenKind::Minus) of length 3. Can contain any string that is
/// parsable by the consumer.
///
/// For example toml:
///
/// ```text
/// ---
/// title: "Yamd"
/// tags:
/// - software
/// - rust
/// ---
/// ```
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Metadata(String);

impl Metadata {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "---\n{}\n---", self.0)
    }
}

impl Deref for Metadata {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
