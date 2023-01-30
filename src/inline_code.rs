use crate::p::ParagraphContent;

#[derive(Debug)]
pub struct InlineCode {
    text: String,
}

impl InlineCode {
    pub fn new<S: Into<String>>(text: S) -> Self {
        InlineCode { text: text.into() }
    }
}

impl From<InlineCode> for String {
    fn from(value: InlineCode) -> Self {
        format!("`{}`", value.text)
    }
}

impl From<InlineCode> for ParagraphContent {
    fn from(value: InlineCode) -> Self {
        ParagraphContent::InlineCode(value)
    }
}

#[cfg(test)]
mod tests {
    use super::InlineCode;

    #[test]
    fn to_string() {
        let inline_code: String = InlineCode::new("const bar = 'baz'").into();
        assert_eq!(inline_code, "`const bar = 'baz'`".to_string())
    }
}
