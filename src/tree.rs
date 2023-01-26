use crate::{h::H, p::P};

#[derive(Debug)]
pub enum TreeContent {
    P(P),
    H(H),
}

impl From<P> for TreeContent {
    fn from(value: P) -> Self {
        TreeContent::P(value)
    }
}

impl From<H> for TreeContent {
    fn from(value: H) -> Self {
        TreeContent::H(value)
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
