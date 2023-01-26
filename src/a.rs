use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct A {
    text: Option<String>,
    url: String,
}

impl Into<ParagraphContent> for A {
    fn into(self) -> ParagraphContent {
        ParagraphContent::A(self)
    }
}

impl A {
    pub fn new<S: Into<String>>(url: S, text: Option<String>) -> Self {
        A {
            text,
            url: url.into(),
        }
    }
}
