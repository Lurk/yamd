use crate::a::A;
use crate::b::B;
use crate::i::I;
use crate::inline_code::InlineCode;
use crate::mdy::MdyNodes;
use crate::s::S;
use crate::serializer::Serializer;
use crate::text::Text;

#[derive(Debug)]
pub enum ParagraphNodes {
    A(A),
    B(B),
    I(I),
    S(S),
    Text(Text),
    InlineCode(InlineCode),
}

impl Serializer for ParagraphNodes {
    fn serialize(&self) -> String {
        match self {
            ParagraphNodes::A(v) => v.serialize(),
            ParagraphNodes::B(v) => v.serialize(),
            ParagraphNodes::I(v) => v.serialize(),
            ParagraphNodes::S(v) => v.serialize(),
            ParagraphNodes::Text(v) => v.serialize(),
            ParagraphNodes::InlineCode(v) => v.serialize(),
        }
    }
}

#[derive(Debug)]
pub struct P {
    data: Vec<ParagraphNodes>,
}

impl P {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_vec(data: Vec<ParagraphNodes>) -> Self {
        Self { data }
    }

    pub fn push<TP: Into<ParagraphNodes>>(mut self, element: TP) -> Self {
        self.data.push(element.into());
        self
    }
}

impl Serializer for P {
    fn serialize(&self) -> String {
        self.data
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .concat()
    }
}

impl Default for P {
    fn default() -> Self {
        Self::new()
    }
}

impl From<P> for MdyNodes {
    fn from(value: P) -> Self {
        MdyNodes::P(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        b::B, deserializer::Branch, inline_code::InlineCode, serializer::Serializer, text::Text,
    };

    use super::P;

    #[test]
    fn push() {
        let p: String = P::new()
            .push(Text::new("simple text "))
            .push(B::from_vec(vec![Text::new("bold text").into()]))
            .push(InlineCode::new("let foo='bar';"))
            .serialize();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }

    #[test]
    fn from_vec() {
        let p: String = P::from_vec(vec![
            Text::new("simple text ").into(),
            B::from_vec(vec![Text::new("bold text").into()]).into(),
            InlineCode::new("let foo='bar';").into(),
        ])
        .serialize();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }
}
