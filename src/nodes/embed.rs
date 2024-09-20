use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Embed {
    pub args: String,
    pub kind: String,
}

impl Embed {
    pub fn new<K: Into<String>, A: Into<String>>(kind: K, args: A) -> Self {
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
