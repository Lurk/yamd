use crate::{
    nodes::heading::Heading,
    nodes::paragraph::Paragraph,
    sd::deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    sd::{context::Context, node::Node},
};

use super::{code::Code, image::Image, list::List};

#[derive(Debug, PartialEq)]
pub enum YamdNodes {
    P(Paragraph),
    H(Heading),
    Image(Image),
    Code(Code),
    List(List),
}

impl From<Paragraph> for YamdNodes {
    fn from(value: Paragraph) -> Self {
        YamdNodes::P(value)
    }
}

impl From<Heading> for YamdNodes {
    fn from(value: Heading) -> Self {
        YamdNodes::H(value)
    }
}

impl From<Image> for YamdNodes {
    fn from(value: Image) -> Self {
        YamdNodes::Image(value)
    }
}

impl From<Code> for YamdNodes {
    fn from(value: Code) -> Self {
        YamdNodes::Code(value)
    }
}

impl From<List> for YamdNodes {
    fn from(value: List) -> Self {
        YamdNodes::List(value)
    }
}

impl Node for YamdNodes {
    fn len(&self) -> usize {
        let len = match self {
            YamdNodes::P(node) => node.len(),
            YamdNodes::H(node) => node.len(),
            YamdNodes::Image(node) => node.len(),
            YamdNodes::Code(node) => node.len(),
            YamdNodes::List(node) => node.len(),
        };
        len + 2
    }
    fn serialize(&self) -> String {
        match self {
            YamdNodes::P(node) => node.serialize(),
            YamdNodes::H(node) => node.serialize(),
            YamdNodes::Image(node) => node.serialize(),
            YamdNodes::Code(node) => node.serialize(),
            YamdNodes::List(node) => node.serialize(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Yamd {
    nodes: Vec<YamdNodes>,
}

impl Branch<YamdNodes> for Yamd {
    fn new_with_context(_: &Option<Context>) -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec_with_context(data: Vec<YamdNodes>, _: Option<Context>) -> Self {
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
            List::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<YamdNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        0
    }
}

impl Deserializer for Yamd {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        Self::parse_branch(input, &None)
    }
}

impl Default for Yamd {
    fn default() -> Self {
        Self::new_with_context(&None)
    }
}

impl Node for Yamd {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::heading::Heading,
        nodes::paragraph::Paragraph,
        nodes::{bold::Bold, code::Code, image::Image, text::Text},
        sd::deserializer::Branch,
        sd::{deserializer::Deserializer, node::Node},
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
            ]))
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
            ]))
        );
    }
}
