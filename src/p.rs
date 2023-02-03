use crate::a::A;
use crate::b::B;
use crate::i::I;
use crate::inline_code::InlineCode;
use crate::mdy::MdyNodes;
use crate::s::S;
use crate::serializer::Serializer;
use crate::text::Text;

#[derive(Debug)]
pub enum ParagraphNode {
    A(A),
    B(B),
    I(I),
    S(S),
    Text(Text),
    InlineCode(InlineCode),
}

impl Serializer for ParagraphNode {
    fn serialize(&self) -> String {
        match self {
            ParagraphNode::A(v) => v.serialize(),
            ParagraphNode::B(v) => v.serialize(),
            ParagraphNode::I(v) => v.serialize(),
            ParagraphNode::S(v) => v.serialize(),
            ParagraphNode::Text(v) => v.serialize(),
            ParagraphNode::InlineCode(v) => v.serialize(),
        }
    }
}

#[derive(Debug)]
pub struct P {
    nodes: Vec<ParagraphNode>,
}

impl P {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn from_vec(data: Vec<ParagraphNode>) -> Self {
        Self { nodes: data }
    }

    pub fn push<TP: Into<ParagraphNode>>(mut self, element: TP) -> Self {
        self.nodes.push(element.into());
        self
    }
}

impl Serializer for P {
    fn serialize(&self) -> String {
        self.nodes
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
