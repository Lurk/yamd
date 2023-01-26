use crate::p::P;

#[derive(Debug)]
pub enum TreeContent {
    P(P),
}

impl From<P> for TreeContent {
    fn from(value: P) -> Self {
        TreeContent::P(value)
    }
}

#[derive(Debug)]
pub struct Tree {
    data: Vec<TreeContent>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { data: vec![] }
    }

    pub fn push<TT: Into<TreeContent>>(mut self, element: TT) -> Self {
        self.data.push(element.into());
        self
    }
}
