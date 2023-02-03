use crate::{nodes::h::H, nodes::p::P, sd::serializer::Serializer};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Yamd {
    nodes: Vec<YamdNodes>,
}

impl Yamd {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn from_vec(data: Vec<YamdNodes>) -> Self {
        Self { nodes: data }
    }

    pub fn push<TC: Into<YamdNodes>>(mut self, element: TC) -> Self {
        self.nodes.push(element.into());
        self
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

impl Default for Yamd {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::h::H, nodes::p::P, nodes::text::Text, sd::deserializer::Branch,
        sd::serializer::Serializer,
    };

    use super::Yamd;

    #[test]
    fn push() {
        let t: String = Yamd::new()
            .push(H::new("header", 1))
            .push(P::from_vec(vec![Text::new("text").into()]))
            .serialize();

        assert_eq!(t, "# header\n\ntext".to_string());
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
}
