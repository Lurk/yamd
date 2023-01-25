use crate::p::{ParagraphContent, ToParagraph};

#[derive(Debug)]
pub struct S {
    text: String,
}

impl ToParagraph for S {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::S(self)
    }
}

impl S {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        S { text: text.into() }
    }
}
