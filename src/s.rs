#[derive(Debug)]
pub struct S {
    text: String,
}

impl S {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        S { text: text.into() }
    }
}
