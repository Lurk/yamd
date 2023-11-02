use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::accordion_tab::AccordionTab;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum AccordionNodes {
    AccordionTab(AccordionTab),
}

impl Display for AccordionNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccordionNodes::AccordionTab(tab) => write!(f, "{}", tab),
        }
    }
}

impl Node for AccordionNodes {
    fn len(&self) -> usize {
        match self {
            AccordionNodes::AccordionTab(tab) => tab.len(),
        }
    }
}

impl From<AccordionTab> for AccordionNodes {
    fn from(tab: AccordionTab) -> Self {
        AccordionNodes::AccordionTab(tab)
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Accordion {
    pub nodes: Vec<AccordionNodes>,
}

impl Accordion {
    pub fn new(nodes: Vec<AccordionNodes>) -> Self {
        Accordion { nodes }
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Accordion::new(vec![])
    }
}

impl Display for Accordion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "///\n{nodes}\n\\\\\\",
            nodes = self
                .nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl Node for Accordion {
    fn len(&self) -> usize {
        let delimiter_len = if self.is_empty() {
            0
        } else {
            self.nodes.len() - 1
        };
        self.nodes.iter().map(|n| n.len()).sum::<usize>()
            + self.get_outer_token_length()
            + delimiter_len
    }
}

impl Branch<AccordionNodes> for Accordion {
    fn push<CanBeNode: Into<AccordionNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<AccordionNodes>> {
        vec![AccordionTab::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<AccordionNodes>> {
        None
    }

    fn get_outer_token_length(&self) -> usize {
        8
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Deserializer for Accordion {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut matcher = Matcher::new(input);

        if let Some(accordion) = matcher.get_match("///\n", "\n\\\\\\", false) {
            return Self::parse_branch(accordion.body, "\n", Self::default());
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        nodes::{paragraph::Paragraph, text::Text},
        toolkit::deserializer::Deserializer,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize_empty() {
        let input = "///\n\n\\\\\\";
        assert_eq!(Accordion::deserialize(input), Some(Accordion::default()));
    }

    #[test]
    fn test_deserialize_with_tabs() {
        let input = r#"///
//
/ header

\\
//
/ one more

\\
\\\"#;
        assert_eq!(
            Accordion::deserialize(input),
            Some(Accordion::new(vec![
                AccordionTab::new(Some("header"), vec![]).into(),
                AccordionTab::new(Some("one more"), vec![]).into()
            ]))
        );
    }

    #[test]
    fn test_len() {
        let input = r#"///
//
/ header

\\
//
/ one more

\\
\\\"#;
        assert_eq!(Accordion::deserialize(input).unwrap().len(), 41);
    }

    #[test]
    fn deserialize_nested() {
        assert_eq!(
            Accordion::deserialize(
                "///\n//\n/ header\n///\n//\n/ hi from nested\ncontent\n\\\\\n\\\\\\\n\\\\\n\\\\\\"
            ),
            Some(Accordion::new(vec![AccordionTab::new(
                Some("header"),
                vec![Accordion::new(vec![AccordionTab::new(
                    Some("hi from nested"),
                    vec![Paragraph::new(vec![Text::new("content").into()]).into()]
                )
                .into()])
                .into()]
            )
            .into()]))
        );
    }

    #[test]
    fn empty_accordion() {
        let accordion = Accordion::default();
        assert_eq!(accordion.len(), 8);
        assert_eq!(accordion.is_empty(), true);
    }
}
