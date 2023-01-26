#[derive(Debug)]
pub struct H {
    level: u8,
    text: String,
}

impl H {
    pub fn new<S: Into<String>>(text: S, level: u8) -> Self {
        H {
            text: text.into(),
            level,
        }
    }
}
