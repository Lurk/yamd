use crate::{i::I, s::S, text::Text};

#[derive(Debug)]
pub enum BContent {
    Text(Text),
    I(I),
    S(S),
}

impl From<Text> for BContent {
    fn from(value: Text) -> Self {
        BContent::Text(value)
    }
}

impl From<I> for BContent {
    fn from(value: I) -> Self {
        BContent::I(value)
    }
}

impl From<S> for BContent {
    fn from(value: S) -> Self {
        BContent::S(value)
    }
}

#[derive(Debug)]
pub struct B {
    data: Vec<BContent>,
}

impl B {
    pub fn new() -> Self {
        B { data: vec![] }
    }

    fn push<BC: Into<BContent>>(mut self, element: BC) -> Self {
        self.data.push(element.into());
        self
    }
}
