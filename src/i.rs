use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct I {
    text: String,
}

impl Into<ParagraphContent> for I {
    fn into(self) -> ParagraphContent {
        ParagraphContent::I(self)
    }
}

impl I {
    pub fn new<S: Into<String>>(text: S) -> Self {
        I { text: text.into() }
    }
}
