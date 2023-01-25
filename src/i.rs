use crate::p::{ParagraphContent, ToParagraph};

#[derive(Debug)]
pub struct I {
    text: String,
}

impl ToParagraph for I {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::I(self)
    }
}

impl I {
    pub fn new<S: Into<String>>(text: S) -> Self {
        I { text: text.into() }
    }
}
