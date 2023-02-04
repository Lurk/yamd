use crate::{
    nodes::i::I,
    nodes::p::ParagraphNode,
    nodes::s::S,
    nodes::text::Text,
    sd::deserializer::{Branch, Deserializer, MaybeNode, Node, Tokenizer},
    sd::serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub enum BNode {
    Text(Text),
    I(I),
    S(S),
}

impl Node for BNode {
    fn len(&self) -> usize {
        match self {
            BNode::Text(node) => node.len(),
            BNode::I(node) => node.len(),
            BNode::S(node) => node.len(),
        }
    }

    fn get_token_length(&self) -> usize {
        0
    }
}

impl Serializer for BNode {
    fn serialize(&self) -> String {
        match self {
            BNode::Text(v) => v.serialize(),
            BNode::I(v) => v.serialize(),
            BNode::S(v) => v.serialize(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct B {
    nodes: Vec<BNode>,
}

impl From<B> for ParagraphNode {
    fn from(value: B) -> Self {
        ParagraphNode::B(value)
    }
}

impl Serializer for B {
    fn serialize(&self) -> String {
        format!(
            "**{}**",
            self.nodes
                .iter()
                .map(|element| { element.serialize() })
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

impl Branch<BNode> for B {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec(data: Vec<BNode>) -> Self {
        Self { nodes: data }
    }

    fn push<BC: Into<BNode>>(&mut self, element: BC) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<BNode>> {
        vec![Box::new(I::maybe_node), Box::new(S::maybe_node)]
    }

    fn get_fallback() -> Box<dyn Fn(&str) -> BNode> {
        Box::new(|str| Text::new(str).into())
    }
}

impl Default for B {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for B {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_token_length()
    }

    fn get_token_length(&self) -> usize {
        4
    }
}

impl Deserializer for B {
    fn deserialize(input: &str, start_position: usize) -> Option<Self> {
        let mut chars = Tokenizer::new(input, start_position);
        if let Some(body) = chars.get_token_body(vec!['*', '*'], vec!['*', '*']) {
            let result = Self::parse_branch(body);
            return Some(result);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::b::B,
        nodes::i::I,
        nodes::s::S,
        nodes::text::Text,
        sd::deserializer::{Branch, Deserializer},
        sd::{deserializer::Node, serializer::Serializer},
    };

    #[test]
    fn only_text() {
        let mut b = B::new();
        b.push(Text::new("B as bold"));
        let str = b.serialize();
        assert_eq!(str, "**B as bold**".to_string());
    }

    #[test]
    fn from_vec() {
        let b: String = B::from_vec(vec![
            Text::new("B as bold ").into(),
            I::new("Italic").into(),
            S::new("Strikethrough").into(),
        ])
        .serialize();
        assert_eq!(b, "**B as bold _Italic_~~Strikethrough~~**".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(
            B::deserialize("**b**", 0),
            Some(B::from_vec(vec![Text::new("b").into()]))
        );

        assert_eq!(
            B::deserialize("**b ~~st~~ _i t_**", 0),
            Some(B::from_vec(vec![
                Text::new("b ").into(),
                S::new("st").into(),
                Text::new(" ").into(),
                I::new("i t").into()
            ]))
        );
    }

    #[test]
    fn len() {
        assert_eq!(B::from_vec(vec![Text::new("T").into()]).len(), 5);
        assert_eq!(
            B::from_vec(vec![Text::new("T").into(), S::new("S").into()]).len(),
            10
        );
    }
}
