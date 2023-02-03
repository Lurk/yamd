use crate::a::A;
use crate::b::B;
use crate::deserializer::{Branch, Deserializer, Node};
use crate::i::I;
use crate::inline_code::InlineCode;
use crate::mdy::YamdNodes;
use crate::s::S;
use crate::serializer::Serializer;
use crate::text::Text;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct P {
    nodes: Vec<ParagraphNode>,
}

impl Branch<ParagraphNode> for P {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec(data: Vec<ParagraphNode>) -> Self {
        Self { nodes: data }
    }

    fn push<TP: Into<ParagraphNode>>(&mut self, element: TP) {
        self.nodes.push(element.into());
    }

    fn get_parsers() -> Vec<crate::deserializer::MaybeNode<ParagraphNode>> {
        vec![
            Box::new(|str, pos| A::maybe_node(str, pos)),
            Box::new(|str, pos| B::maybe_node(str, pos)),
            Box::new(|str, pos| I::maybe_node(str, pos)),
            Box::new(|str, pos| S::maybe_node(str, pos)),
            Box::new(|str, pos| InlineCode::maybe_node(str, pos)),
        ]
    }

    fn get_fallback() -> Box<dyn Fn(&str) -> ParagraphNode> {
        Box::new(|str| Text::new(str).into())
    }
}

impl Deserializer for P {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let end_position = match input.find("\n\n") {
            Some(position) => position,
            None => input.len(),
        };
        println!("{}", &input[start_position..end_position]);
        Some((
            Self::parse_branch(&input[start_position..end_position]),
            end_position,
        ))
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

impl From<P> for YamdNodes {
    fn from(value: P) -> Self {
        YamdNodes::P(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        b::B,
        deserializer::{Branch, Deserializer},
        inline_code::InlineCode,
        serializer::Serializer,
        text::Text,
    };

    use super::P;

    #[test]
    fn push() {
        let mut p = P::new();
        p.push(Text::new("simple text "));
        p.push(B::from_vec(vec![Text::new("bold text").into()]));
        p.push(InlineCode::new("let foo='bar';"));

        assert_eq!(
            p.serialize(),
            "simple text **bold text**`let foo='bar';`".to_string()
        );
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

    #[test]
    fn deserialize() {
        assert_eq!(
            P::deserialize("simple text **bold text**`let foo='bar';`", 0),
            Some((
                P::from_vec(vec![
                    Text::new("simple text ").into(),
                    B::from_vec(vec![Text::new("bold text").into()]).into(),
                    InlineCode::new("let foo='bar';").into(),
                ]),
                41
            ))
        );
        assert_eq!(
            P::deserialize("1 2\n\n3", 2),
            Some((P::from_vec(vec![Text::new("2").into()]), 3))
        );
    }
}
