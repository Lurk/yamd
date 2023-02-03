use crate::{
    nodes::h::H,
    nodes::p::P,
    sd::deserializer::{Branch, Deserializer, MaybeNode, Node, Tokenizer},
    sd::serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub enum YamdNodes {
    P(P),
    H(H),
}

impl Serializer for YamdNodes {
    fn serialize(&self) -> String {
        match self {
            YamdNodes::P(v) => v.serialize(),
            YamdNodes::H(v) => v.serialize(),
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

    fn get_parsers() -> Vec<MaybeNode<YamdNodes>> {
        vec![Box::new(|str, pos| H::maybe_node(str, pos))]
    }

    fn get_fallback() -> Box<dyn Fn(&str) -> YamdNodes> {
        Box::new(|str| {
            let (node, _) = P::deserialize(str, 0).unwrap_or((P::new(), 0));
            node.into()
        })
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
    fn deserialize(input: &str, _start_position: usize) -> Option<(Self, usize)> {
        let result = Self::parse_branch(input);
        return Some((result, input.len()));
    }
}

impl Default for Yamd {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::h::H,
        nodes::p::P,
        nodes::{b::B, text::Text},
        sd::deserializer::Branch,
        sd::{deserializer::Deserializer, serializer::Serializer},
    };

    use super::Yamd;

    #[test]
    fn push() {
        let mut t = Yamd::new();
        t.push(H::new("header", 1));
        t.push(P::from_vec(vec![Text::new("text").into()]));

        assert_eq!(t.serialize(), "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Yamd::from_vec(vec![
            H::new("header", 1).into(),
            P::from_vec(vec![Text::new("text").into()]).into(),
        ])
        .serialize();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Yamd::deserialize("# h\n\nt", 0),
            Some((
                Yamd::from_vec(vec![
                    H::new("h", 1).into(),
                    P::from_vec(vec![Text::new("t").into()]).into()
                ]),
                6
            ))
        );

        assert_eq!(
            Yamd::deserialize("t**b**\n\n## h", 0),
            Some((
                Yamd::from_vec(vec![
                    P::from_vec(vec![
                        Text::new("t").into(),
                        B::from_vec(vec![Text::new("b").into()]).into()
                    ])
                    .into(),
                    H::new("h", 2).into(),
                ]),
                12
            ))
        );
    }
}
