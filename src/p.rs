use crate::a::A;
use crate::b::B;
use crate::h::H;
use crate::i::I;
use crate::inline_code::InlineCode;
use crate::s::S;
use crate::text::Text;
use crate::tree::{ToTree, TreeContent};

pub trait ToParagraph {
    fn to_paragraph(self) -> ParagraphContent;
}

#[derive(Debug)]
pub enum ParagraphContent {
    A(A),
    B(B),
    H(H),
    I(I),
    S(S),
    Text(Text),
    InlineCode(InlineCode),
}

#[derive(Debug)]
pub struct P {
    data: Vec<ParagraphContent>,
}

impl ToTree for P {
    fn to_tree(self) -> TreeContent {
        TreeContent::P(self)
    }
}

impl P {
    pub fn new() -> Self {
        P { data: vec![] }
    }

    pub fn push<TP: ToParagraph>(mut self, element: TP) -> Self {
        self.data.push(element.to_paragraph());
        self
    }
}
