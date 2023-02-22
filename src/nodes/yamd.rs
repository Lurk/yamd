use crate::{
    nodes::heading::Heading,
    nodes::paragraph::Paragraph,
    sd::deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode, Node},
    sd::serializer::Serializer,
};

use super::{code::Code, image::Image};

#[derive(Debug, PartialEq)]
pub enum YamdNodes {
    P(Paragraph),
    H(Heading),
    Image(Image),
    Code(Code),
}

impl Node for YamdNodes {
    fn len(&self) -> usize {
        match self {
            YamdNodes::P(node) => node.len() + 2,
            YamdNodes::H(node) => node.len() + 2,
            YamdNodes::Image(node) => node.len() + 2,
            YamdNodes::Code(node) => node.len() + 2,
        }
    }
}

impl Serializer for YamdNodes {
    fn serialize(&self) -> String {
        match self {
            YamdNodes::P(node) => node.serialize(),
            YamdNodes::H(node) => node.serialize(),
            YamdNodes::Image(node) => node.serialize(),
            YamdNodes::Code(node) => node.serialize(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Yamd {
    nodes: Vec<YamdNodes>,
}

impl Branch<YamdNodes> for Yamd {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec(data: Vec<YamdNodes>) -> Self {
        Self { nodes: data }
    }

    fn push<TC: Into<YamdNodes>>(&mut self, element: TC) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<YamdNodes>> {
        vec![
            Heading::maybe_node(),
            Image::maybe_node(),
            Code::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<YamdNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        0
    }
}

impl Serializer for Yamd {
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Deserializer for Yamd {
    fn deserialize(input: &str) -> Option<Self> {
        Self::parse_branch(input)
    }
}

impl Default for Yamd {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Yamd {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::heading::Heading,
        nodes::paragraph::Paragraph,
        nodes::{bold::Bold, code::Code, image::Image, text::Text},
        sd::deserializer::Branch,
        sd::{deserializer::Deserializer, serializer::Serializer},
    };

    use super::Yamd;

    #[test]
    fn push() {
        let mut t = Yamd::new();
        t.push(Heading::new("header", 1));
        t.push(Paragraph::from_vec(vec![Text::new("text").into()]));

        assert_eq!(t.serialize(), "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Yamd::from_vec(vec![
            Heading::new("header", 1).into(),
            Paragraph::from_vec(vec![Text::new("text").into()]).into(),
        ])
        .serialize();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Yamd::deserialize("# hh\n\ntt\n\n![a](u)"),
            Some(Yamd::from_vec(vec![
                Heading::new("hh", 1).into(),
                Paragraph::from_vec(vec![Text::new("tt").into()]).into(),
                Image::new("a", "u").into()
            ]),)
        );

        assert_eq!(
            Yamd::deserialize("t**b**\n\n![a](u)\n\n## h"),
            Some(Yamd::from_vec(vec![
                Paragraph::from_vec(vec![
                    Text::new("t").into(),
                    Bold::from_vec(vec![Text::new("b").into()]).into()
                ])
                .into(),
                Image::new('a', 'u').into(),
                Heading::new("h", 2).into(),
            ]),)
        );

        assert_eq!(
            Yamd::deserialize("```rust\nlet a=1;\n```\n\nt**b**\n\n![a](u)\n\n## h"),
            Some(Yamd::from_vec(vec![
                Code::new("rust", "let a=1;").into(),
                Paragraph::from_vec(vec![
                    Text::new("t").into(),
                    Bold::from_vec(vec![Text::new("b").into()]).into()
                ])
                .into(),
                Image::new('a', 'u').into(),
                Heading::new("h", 2).into(),
            ]),)
        );
    }
}
