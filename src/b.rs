use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct B {
    text: String,
}

impl B {
    pub fn new(text: String) -> Self {
        B { text }
    }
}

impl Into<ParagraphContent> for B {
    fn into(self) -> ParagraphContent {
        ParagraphContent::B(self)
    }
}
