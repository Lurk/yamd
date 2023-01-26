use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct Text {
    text: String,
}

impl Into<ParagraphContent> for Text {
    fn into(self) -> ParagraphContent {
        ParagraphContent::Text(self)
    }
}

impl Text {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Text { text: text.into() }
    }
}
