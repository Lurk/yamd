use crate::p::{ParagraphContent, ToParagraph};

#[derive(Debug)]
pub struct Text {
    text: String,
}

impl ToParagraph for Text {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::Text(self)
    }
}

impl Text {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Text { text: text.into() }
    }
}

