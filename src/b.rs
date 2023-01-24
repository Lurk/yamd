use crate::p::{ParagraphContent, ToParagraph};

pub struct B {
    text: String,
}

impl ToParagraph for B {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::B(self)
    }
}
impl B {
    pub fn new(text: String) -> Self {
        B { text }
    }
}
