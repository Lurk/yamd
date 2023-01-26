use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct InlineCode {
    text: String,
}

impl Into<ParagraphContent> for InlineCode {
    fn into(self) -> ParagraphContent {
        ParagraphContent::InlineCode(self)
    }
}

impl InlineCode {
    pub fn new<S: Into<String>>(text: S) -> Self {
        InlineCode { text: text.into() }
    }
}
