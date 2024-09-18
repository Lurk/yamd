use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Code {
    pub lang: String,
    pub code: String,
}

impl Code {
    pub fn new<S: Into<String>>(lang: S, code: S) -> Self {
        Self {
            lang: lang.into(),
            code: code.into(),
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "```{}\n{}\n```", self.lang, self.code)
    }
}
