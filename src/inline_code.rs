use crate::p::{ParagraphContent, ToParagraph};

#[derive(Debug)]
pub struct InlineCode {
    text: String,
}

impl ToParagraph for InlineCode {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::InlineCode(self)
    }
}

impl InlineCode {
    pub fn new<S: Into<String>>(text: S) -> Self {
        InlineCode { text: text.into() }
    }
}
