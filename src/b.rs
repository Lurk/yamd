#[derive(Debug)]
pub struct B {
    text: String,
}

impl B {
    pub fn new(text: String) -> Self {
        B { text }
    }
}
