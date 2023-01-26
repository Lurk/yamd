use crate::a::A;
use crate::b::B;
use crate::h::H;
use crate::i::I;
use crate::inline_code::InlineCode;
use crate::s::S;
use crate::text::Text;

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

impl From<A> for ParagraphContent {
    fn from(value: A) -> Self {
        ParagraphContent::A(value)
    }
}

impl From<B> for ParagraphContent {
    fn from(value: B) -> Self {
        ParagraphContent::B(value)
    }
}

impl From<H> for ParagraphContent {
    fn from(value: H) -> Self {
        ParagraphContent::H(value)
    }
}

impl From<I> for ParagraphContent {
    fn from(value: I) -> Self {
        ParagraphContent::I(value)
    }
}

impl From<S> for ParagraphContent {
    fn from(value: S) -> Self {
        ParagraphContent::S(value)
    }
}

impl From<Text> for ParagraphContent {
    fn from(value: Text) -> Self {
        ParagraphContent::Text(value)
    }
}

impl From<InlineCode> for ParagraphContent {
    fn from(value: InlineCode) -> Self {
        ParagraphContent::InlineCode(value)
    }
}

#[derive(Debug)]
pub struct P {
    data: Vec<ParagraphContent>,
}

impl P {
    pub fn new() -> Self {
        P { data: vec![] }
    }

    pub fn push<TP: Into<ParagraphContent>>(mut self, element: TP) -> Self {
        self.data.push(element.into());
        self
    }
}
