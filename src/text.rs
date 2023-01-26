#[derive(Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Text { text: text.into() }
    }
}
