use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::accordion_tab::AccordionTab;

#[derive(Debug, PartialEq)]
pub enum AccordionNodes<'text> {
    AccordionTab(AccordionTab<'text>),
}

impl Node<'_> for AccordionNodes<'_> {
    fn serialize(&self) -> String {
        match self {
            AccordionNodes::AccordionTab(tab) => tab.serialize(),
        }
    }

    fn len(&self) -> usize {
        match self {
            AccordionNodes::AccordionTab(tab) => tab.len(),
        }
    }
}

impl<'text> From<AccordionTab<'text>> for AccordionNodes<'text> {
    fn from(tab: AccordionTab<'text>) -> Self {
        AccordionNodes::AccordionTab(tab)
    }
}

#[derive(Debug, PartialEq)]
pub struct Accordion<'text> {
    consumed_all_input: bool,
    pub nodes: Vec<AccordionNodes<'text>>,
}

impl<'text> Accordion<'text> {
    pub fn new(consumed_all_input: bool) -> Self {
        Self::new_with_nodes(consumed_all_input, vec![])
    }

    pub fn new_with_nodes(consumed_all_input: bool, nodes: Vec<AccordionNodes<'text>>) -> Self {
        Accordion {
            consumed_all_input,
            nodes,
        }
    }
}

impl Node<'_> for Accordion<'_> {
    fn serialize(&self) -> String {
        format!(
            "///\n{nodes}\n\\\\\\{end}",
            nodes = self
                .nodes
                .iter()
                .map(|n| n.serialize())
                .collect::<Vec<String>>()
                .join(""),
            end = if self.consumed_all_input { "" } else { "\n\n" }
        )
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|n| n.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl<'text> Branch<'text, AccordionNodes<'text>> for Accordion<'text> {
    fn push<CanBeNode: Into<AccordionNodes<'text>>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<'text, AccordionNodes<'text>>> {
        vec![AccordionTab::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<'text, AccordionNodes<'text>>> {
        None
    }

    fn get_outer_token_length(&self) -> usize {
        8 + if self.consumed_all_input { 0 } else { 2 }
    }
}

impl<'text> Deserializer<'text> for Accordion<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut matcher = Matcher::new(input);

        if let Some(accordion) = matcher.get_match("///\n", "\n\\\\\\", false) {
            let consumed_all_input = matcher.get_match("\n\n", "", false).is_none();
            return Self::parse_branch(accordion.body, Self::new(consumed_all_input));
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
        assert_eq!(Accordion::deserialize(input), Some(Accordion::new(true)));
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
            Some(Accordion::new_with_nodes(
                true,
                vec![
                    AccordionTab::new(false, Some("header")).into(),
                    AccordionTab::new(true, Some("one more")).into()
                ]
            ))
        );
    }

    #[test]
    fn consumed_all_input() {
        let input = "///\n\n\\\\\\\n\n";
        assert_eq!(Accordion::deserialize(input), Some(Accordion::new(false)));
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
            Some(Accordion::new_with_nodes(
                true,
                vec![AccordionTab::new_with_nodes(
                    true,
                    Some("header"),
                    vec![Accordion::new_with_nodes(
                        true,
                        vec![AccordionTab::new_with_nodes(
                            true,
                            Some("hi from nested"),
                            vec![Paragraph::new_with_nodes(
                                true,
                                vec![Text::new("content").into()]
                            )
                            .into()]
                        )
                        .into()]
                    )
                    .into()]
                )
                .into()]
            ))
        );
    }
}
