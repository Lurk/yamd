use crate::{h::H, p::P, serializer::Serializer};

#[derive(Debug)]
pub enum MdyNodes {
    P(P),
    H(H),
}

impl Serializer for MdyNodes {
    fn serialize(&self) -> String {
        match self {
            MdyNodes::P(v) => v.serialize(),
            MdyNodes::H(v) => v.serialize(),
        }
    }
}

#[derive(Debug)]
pub struct Mdy {
    nodes: Vec<MdyNodes>,
}

impl Mdy {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn from_vec(data: Vec<MdyNodes>) -> Self {
        Self { nodes: data }
    }

    pub fn push<TC: Into<MdyNodes>>(mut self, element: TC) -> Self {
        self.nodes.push(element.into());
        self
    }
}

impl Serializer for Mdy {
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Default for Mdy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{h::H, p::P, serializer::Serializer, text::Text};

    use super::Mdy;

    #[test]
    fn push() {
        let t: String = Mdy::new()
            .push(H::new("header", 1))
            .push(P::new().push(Text::new("text")))
            .serialize();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Mdy::from_vec(vec![
            H::new("header", 1).into(),
            P::new().push(Text::new("text")).into(),
        ])
        .serialize();

        assert_eq!(t, "# header\n\ntext".to_string());
    }
}
