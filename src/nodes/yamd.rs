use crate::{
    nodes::h::H,
    nodes::p::P,
    sd::deserializer::{Branch, Deserializer, MaybeNode, Node},
    sd::serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub enum YamdNodes {
    P(P),
    H(H),
}

impl Node for YamdNodes {
    fn len(&self) -> usize {
        match self {
            YamdNodes::P(node) => node.len() + self.get_token_length(),
            YamdNodes::H(node) => node.len() + self.get_token_length(),
        }
    }

    fn get_token_length(&self) -> usize {
        2
    }
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

    fn get_maybe_nodes() -> Vec<MaybeNode<YamdNodes>> {
        vec![Box::new(H::maybe_node)]
    }

    fn get_fallback() -> Box<dyn Fn(&str) -> YamdNodes> {
        Box::new(|str| {
            let node = P::deserialize(str).unwrap_or(P::new());
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
    fn deserialize(input: &str) -> Option<Self> {
        Some(Self::parse_branch(input))
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

    fn get_token_length(&self) -> usize {
        0
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
            Yamd::deserialize("# hh\n\ntt"),
            Some(Yamd::from_vec(vec![
                H::new("hh", 1).into(),
                P::from_vec(vec![Text::new("tt").into()]).into()
            ]),)
        );

        assert_eq!(
            Yamd::deserialize("t**b**\n\n## h"),
            Some(Yamd::from_vec(vec![
                P::from_vec(vec![
                    Text::new("t").into(),
                    B::from_vec(vec![Text::new("b").into()]).into()
                ])
                .into(),
                H::new("h", 2).into(),
            ]),)
        );
    }
}
