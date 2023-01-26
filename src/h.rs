use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct H {
    level: u8,
    text: String,
}

impl Into<ParagraphContent> for H {
    fn into(self) -> ParagraphContent {
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
