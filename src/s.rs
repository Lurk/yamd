use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct S {
    text: String,
}

impl Into<ParagraphContent> for S {
    fn into(self) -> ParagraphContent {
        ParagraphContent::S(self)
    }
}

impl S {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        S { text: text.into() }
    }
}
