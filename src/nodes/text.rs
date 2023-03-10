use crate::sd::{
    context::Context,
    deserializer::{DefinitelyNode, Deserializer, FallbackNode},
    node::Node,
};

/// Representation of a regular text
#[derive(Debug, PartialEq)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Text { text: text.into() }
    }
}

impl Deserializer for Text {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        Some(Text::new(input.to_string()))
    }
}

impl Node for Text {
    fn len(&self) -> usize {
        self.text.len()
    }
    fn serialize(&self) -> String {
        self.text.clone()
    }
}

impl FallbackNode for Text {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
    where
        Self: Into<BranchNodes>,
    {
        Box::new(|input| Text::new(input).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{deserializer::Deserializer, node::Node};

    use super::Text;

    #[test]
    fn happy_path() {
        let text = Text::new("shiny text");
        assert_eq!(text.text, "shiny text".to_string());
    }

    #[test]
    fn to_string() {
        let text: String = Text::new("shiny text").serialize();
        assert_eq!(text, "shiny text".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Text::deserialize("t"), Some(Text::new("t")));
    }
}
