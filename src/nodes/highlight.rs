use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    node::Node,
    tokenizer::{
        Pattern::{Once, RepeatTimes},
        Tokenizer,
    },
};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq)]
pub enum HighlightNodes {
    Paragraph(Paragraph),
}

impl Node for HighlightNodes {
    fn serialize(&self) -> String {
        match self {
            HighlightNodes::Paragraph(node) => node.serialize(),
        }
    }

    fn len(&self) -> usize {
        match self {
            HighlightNodes::Paragraph(node) => node.len(),
        }
    }
}

impl From<Paragraph> for HighlightNodes {
    fn from(value: Paragraph) -> Self {
        Self::Paragraph(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Highlight {
    header: Option<String>,
    icon: Option<String>,
    nodes: Vec<HighlightNodes>,
}

impl Highlight {
    pub fn new<H: Into<String>, I: Into<String>>(header: Option<H>, icon: Option<I>) -> Self {
        Self::new_with_nodes(header, icon, vec![])
    }

    pub fn new_with_nodes<H: Into<String>, I: Into<String>>(
        header: Option<H>,
        icon: Option<I>,
        nodes: Vec<HighlightNodes>,
    ) -> Self {
        Self {
            header: match header {
                Some(header) => Some(header.into()),
                None => None,
            },
            icon: match icon {
                Some(icon) => Some(icon.into()),
                None => None,
            },
            nodes,
        }
    }
}

impl Node for Highlight {
    fn serialize(&self) -> String {
        let header = match &self.header {
            Some(header) => format!(">> {header}\n"),
            None => String::new(),
        };
        let icon = match &self.icon {
            Some(icon) => format!("> {icon}\n"),
            None => String::new(),
        };
        format!(
            ">>>\n{header}{icon}{}\n>>>",
            self.nodes
                .iter()
                .map(|node| node.serialize())
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>()
            + (self.nodes.len() - 1) * 2
            + self.get_outer_token_length()
    }
}

impl Branch<HighlightNodes> for Highlight {
    fn push<CanBeNode: Into<HighlightNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<HighlightNodes>> {
        vec![Paragraph::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<HighlightNodes>> {
        None
    }

    fn get_outer_token_length(&self) -> usize {
        let header = match &self.header {
            Some(header) => header.len() + 4,
            None => 0,
        };
        let icon = match &self.icon {
            Some(icon) => icon.len() + 3,
            None => 0,
        };

        8 + icon + header
    }
}

impl Deserializer for Highlight {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_node_body(
            &[RepeatTimes(3, '>'), Once('\n')],
            &[Once('\n'), RepeatTimes(3, '>')],
        ) {
            let mut tokenizer = Tokenizer::new(body);
            let header =
                match tokenizer.get_node_body(&[RepeatTimes(2, '>'), Once(' ')], &[Once('\n')]) {
                    Some(header) => Some(header.to_string()),
                    None => None,
                };

            let icon = match tokenizer.get_node_body(&[Once('>'), Once(' ')], &[Once('\n')]) {
                Some(icon) => Some(icon.to_string()),
                None => None,
            };

            return Self::parse_branch(tokenizer.get_rest(), Self::new(header, icon));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{highlight::Highlight, paragraph::Paragraph, text::Text},
        toolkit::{deserializer::Deserializer, node::Node},
    };

    #[test]
    fn len() {
        assert_eq!(
            Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .len(),
            21
        );
    }
    #[test]
    fn serialize() {
        assert_eq!(
            Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .serialize(),
            String::from(">>>\n>> h\n> i\nt\n\nt\n>>>")
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Highlight::deserialize(">>>\n>> h\n> i\nt\n\nt\n>>>"),
            Some(Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            ))
        )
    }
}
