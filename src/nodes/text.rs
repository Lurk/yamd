use crate::toolkit::{
    context::Context,
    deserializer::{DefinitelyNode, Deserializer, FallbackNode},
    node::Node,
};

/// Representation of a regular text
#[derive(Debug, PartialEq)]
pub struct Text<'text> {
    pub text: &'text str,
}

impl<'text> Text<'text> {
    pub fn new(text: &'text str) -> Self {
        Text { text }
    }
}

impl<'text> Deserializer<'text> for Text<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        Some(Text::new(input))
    }
}

impl<'text> Node<'text> for Text<'text> {
    fn serialize(&self) -> String {
        self.text.to_string()
    }
    fn len(&self) -> usize {
        self.text.len()
    }
}

impl<'text> FallbackNode<'text> for Text<'text> {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<'text, BranchNodes>
    where
        Self: Into<BranchNodes>,
    {
        Box::new(|input| Text::new(input).into())
    }
}

#[cfg(test)]
mod tests {
    use super::Text;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

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
