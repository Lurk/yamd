use crate::p::{ParagraphContent, ToParagraph};

#[derive(Debug)]
pub struct A {
    text: Option<String>,
    url: String,
}

impl ToParagraph for A {
    fn to_paragraph(self) -> ParagraphContent {
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
