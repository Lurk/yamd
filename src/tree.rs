use crate::p::P;

pub enum TreeContent {
    P(P),
}

pub trait ToTree {
    fn to_tree(self) -> TreeContent;
}

pub struct Tree {
    data: Vec<TreeContent>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { data: vec![] }
    }

    pub fn push<TT: ToTree>(mut self, element: TT) -> Self {
        self.data.push(element.to_tree());
        self
    }
}
