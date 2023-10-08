use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq, Serialize)]
pub enum HighlightNodes {
    Paragraph(Paragraph),
}

impl Display for HighlightNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HighlightNodes::Paragraph(node) => write!(f, "{}", node),
        }
    }
}

impl Node for HighlightNodes {
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

#[derive(Debug, PartialEq, Serialize)]
pub struct Highlight {
    pub header: Option<String>,
    pub icon: Option<String>,
    pub nodes: Vec<HighlightNodes>,
    consumed_all_input: bool,
}

impl Highlight {
    pub fn new<H: Into<String>, I: Into<String>>(
        header: Option<H>,
        icon: Option<I>,
        consumed_all_input: bool,
    ) -> Self {
        Self::new_with_nodes(header, icon, consumed_all_input, vec![])
    }

    pub fn new_with_nodes<H: Into<String>, I: Into<String>>(
        header: Option<H>,
        icon: Option<I>,
        consumed_all_input: bool,
        nodes: Vec<HighlightNodes>,
    ) -> Self {
        Self {
            header: header.map(|header| header.into()),
            icon: icon.map(|icon| icon.into()),
            nodes,
            consumed_all_input,
        }
    }
}

impl Display for Highlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = match &self.header {
            Some(header) => format!(">> {header}\n"),
            None => String::new(),
        };
        let icon = match &self.icon {
            Some(icon) => format!("> {icon}\n"),
            None => String::new(),
        };
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        write!(
            f,
            ">>>\n{header}{icon}{}\n>>>{end}",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join(""),
            header = header,
            icon = icon,
            end = end
        )
    }
}

impl Node for Highlight {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
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
        let end = if self.consumed_all_input { 0 } else { 2 };

        8 + icon + header + end
    }
}

impl Deserializer for Highlight {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut outer_matcher = Matcher::new(input);
        if let Some(highlight) = outer_matcher.get_match(">>>\n", "\n>>>", false) {
            let mut matcher = Matcher::new(highlight.body);
            let header = matcher
                .get_match(">> ", "\n", false)
                .map(|header| header.body);

            let icon = matcher.get_match("> ", "\n", false).map(|icon| icon.body);
            let consumed_all_input = outer_matcher.get_match("\n", "", false).is_none();

            return Self::parse_branch(
                matcher.get_rest(),
                Self::new(header, icon, consumed_all_input),
            );
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
    use pretty_assertions::assert_eq;

    #[test]
    fn len() {
        assert_eq!(
            Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                true,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .len(),
            21
        );
        assert_eq!(
            Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                false,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .len(),
            23
        );
        assert_eq!(
            Highlight::new_with_nodes::<String, String>(
                None,
                None,
                false,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .len(),
            14
        );
    }
    #[test]
    fn serialize() {
        assert_eq!(
            Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                true,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .to_string(),
            String::from(">>>\n>> h\n> i\nt\n\nt\n>>>")
        );
        assert_eq!(
            Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                false,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .to_string(),
            String::from(">>>\n>> h\n> i\nt\n\nt\n>>>\n\n")
        );
        assert_eq!(
            Highlight::new_with_nodes::<String, String>(
                None,
                None,
                false,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            )
            .to_string(),
            String::from(">>>\nt\n\nt\n>>>\n\n")
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Highlight::deserialize(">>>\n>> h\n> i\nt\n\nt\n>>>"),
            Some(Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                true,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t").into()]).into()
                ]
            ))
        );

        assert_eq!(
            Highlight::deserialize(">>>\n>> h\n> i\nt\n\nt2\n>>>\n\n"),
            Some(Highlight::new_with_nodes(
                Some("h"),
                Some("i"),
                false,
                vec![
                    Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("t2").into()]).into()
                ]
            ))
        )
    }
}
