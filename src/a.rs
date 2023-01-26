#[derive(Debug)]
pub struct A {
    text: Option<String>,
    url: String,
}

impl A {
    pub fn new<S: Into<String>>(url: S, text: Option<String>) -> Self {
        A {
            text,
            url: url.into(),
        }
    }
}
