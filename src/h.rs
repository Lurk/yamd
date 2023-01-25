use crate::p::{ParagraphContent, ToParagraph};

#[derive(Debug)]
pub struct H {
    level: u8,
    text: String,
}

impl ToParagraph for H {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::H(self)
    }
}

impl H {
    pub fn new<S: Into<String>>(text: S, level: u8) -> Self {
        H {
            text: text.into(),
            level,
        }
    }
}
