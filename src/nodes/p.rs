use crate::nodes::{
    anchor::Anchor, bold::Bold, i::I, inline_code::InlineCode, s::S, text::Text, yamd::YamdNodes,
};
use crate::sd::{
    deserializer::{Branch, Deserializer, MaybeNode, Node},
    serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub enum ParagraphNode {
    A(Anchor),
    B(Bold),
    I(I),
    S(S),
    Text(Text),
    InlineCode(InlineCode),
}

impl Node for ParagraphNode {
    fn len(&self) -> usize {
        match self {
            ParagraphNode::A(node) => node.len(),
            ParagraphNode::B(node) => node.len(),
            ParagraphNode::I(node) => node.len(),
            ParagraphNode::S(node) => node.len(),
            ParagraphNode::Text(node) => node.len(),
            ParagraphNode::InlineCode(node) => node.len(),
        }
    }

    fn get_token_length(&self) -> usize {
        0
    }
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

    fn get_maybe_nodes() -> Vec<MaybeNode<ParagraphNode>> {
        vec![
            Box::new(Anchor::maybe_node),
            Box::new(Bold::maybe_node),
            Box::new(I::maybe_node),
            Box::new(S::maybe_node),
            Box::new(InlineCode::maybe_node),
        ]
    }

    fn get_fallback_node() -> Box<dyn Fn(&str) -> ParagraphNode> {
        Box::new(|str| Text::new(str).into())
    }
}

impl Deserializer for P {
    fn deserialize(input: &str) -> Option<Self> {
        let end_position = match input.find("\n\n") {
            Some(position) => position,
            None => input.len(),
        };
        Some(Self::parse_branch(&input[..end_position]))
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

impl Node for P {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }

    fn get_token_length(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::bold::Bold,
        nodes::inline_code::InlineCode,
        nodes::text::Text,
        sd::deserializer::{Branch, Deserializer},
        sd::serializer::Serializer,
    };

    use super::P;

    #[test]
    fn push() {
        let mut p = P::new();
        p.push(Text::new("simple text "));
        p.push(Bold::from_vec(vec![Text::new("bold text").into()]));
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
            Bold::from_vec(vec![Text::new("bold text").into()]).into(),
            InlineCode::new("let foo='bar';").into(),
        ])
        .serialize();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            P::deserialize("simple text **bold text**`let foo='bar';`"),
            Some(P::from_vec(vec![
                Text::new("simple text ").into(),
                Bold::from_vec(vec![Text::new("bold text").into()]).into(),
                InlineCode::new("let foo='bar';").into(),
            ]),)
        );
        assert_eq!(
            P::deserialize("1 2\n\n3"),
            Some(P::from_vec(vec![Text::new("1 2").into()]))
        );
    }
}
