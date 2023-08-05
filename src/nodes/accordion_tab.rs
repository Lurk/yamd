use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
    pattern::Quantifiers::*,
};

use super::{
    accordion::Accordion, cloudinary_image_gallery::CloudinaryImageGallery, embed::Embed,
    heading::Heading, image::Image, image_gallery::ImageGallery, list::List, paragraph::Paragraph,
};

#[derive(Debug, PartialEq)]
pub enum AccordionTabNodes {
    Pargaraph(Paragraph),
    Heading(Heading),
    Image(Image),
    ImageGallery(ImageGallery),
    CloudinaryImageGallery(CloudinaryImageGallery),
    List(List),
    Embed(Embed),
    Accordion(Accordion),
}

impl Node for AccordionTabNodes {
    fn serialize(&self) -> String {
        match self {
            AccordionTabNodes::Pargaraph(node) => node.serialize(),
            AccordionTabNodes::Heading(node) => node.serialize(),
            AccordionTabNodes::Image(node) => node.serialize(),
            AccordionTabNodes::ImageGallery(node) => node.serialize(),
            AccordionTabNodes::CloudinaryImageGallery(node) => node.serialize(),
            AccordionTabNodes::List(node) => node.serialize(),
            AccordionTabNodes::Embed(node) => node.serialize(),
            AccordionTabNodes::Accordion(node) => node.serialize(),
        }
    }

    fn len(&self) -> usize {
        match self {
            AccordionTabNodes::Pargaraph(node) => node.len(),
            AccordionTabNodes::Heading(node) => node.len(),
            AccordionTabNodes::Image(node) => node.len(),
            AccordionTabNodes::ImageGallery(node) => node.len(),
            AccordionTabNodes::CloudinaryImageGallery(node) => node.len(),
            AccordionTabNodes::List(node) => node.len(),
            AccordionTabNodes::Embed(node) => node.len(),
            AccordionTabNodes::Accordion(node) => node.len(),
        }
    }
}

impl From<Paragraph> for AccordionTabNodes {
    fn from(value: Paragraph) -> Self {
        Self::Pargaraph(value)
    }
}

impl From<Heading> for AccordionTabNodes {
    fn from(value: Heading) -> Self {
        Self::Heading(value)
    }
}

impl From<Image> for AccordionTabNodes {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<ImageGallery> for AccordionTabNodes {
    fn from(value: ImageGallery) -> Self {
        Self::ImageGallery(value)
    }
}

impl From<CloudinaryImageGallery> for AccordionTabNodes {
    fn from(value: CloudinaryImageGallery) -> Self {
        Self::CloudinaryImageGallery(value)
    }
}

impl From<List> for AccordionTabNodes {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<Embed> for AccordionTabNodes {
    fn from(value: Embed) -> Self {
        Self::Embed(value)
    }
}

impl From<Accordion> for AccordionTabNodes {
    fn from(value: Accordion) -> Self {
        Self::Accordion(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct AccordionTab {
    header: Option<String>,
    nodes: Vec<AccordionTabNodes>,
    consumed_all_input: bool,
}

impl AccordionTab {
    pub fn new<S: Into<String>>(consumed_all_input: bool, header: Option<S>) -> Self {
        Self::new_with_nodes(consumed_all_input, header, vec![])
    }
    pub fn new_with_nodes<S: Into<String>>(
        consumed_all_input: bool,
        header: Option<S>,
        nodes: Vec<AccordionTabNodes>,
    ) -> Self {
        Self {
            nodes,
            consumed_all_input,
            header: header.map(|s| s.into()),
        }
    }
}

impl Node for AccordionTab {
    fn serialize(&self) -> String {
        format!(
            "//\n{header}{nodes}\n\\\\{end}",
            header = self
                .header
                .as_ref()
                .map_or("".to_string(), |header| format!("/ {}\n", header)),
            nodes = self
                .nodes
                .iter()
                .map(|node| node.serialize())
                .collect::<Vec<String>>()
                .join(""),
            end = if self.consumed_all_input { "" } else { "\n" }
        )
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl Branch<AccordionTabNodes> for AccordionTab {
    fn push<CanBeNode: Into<AccordionTabNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<AccordionTabNodes>> {
        vec![
            Heading::maybe_node(),
            Image::maybe_node(),
            ImageGallery::maybe_node(),
            CloudinaryImageGallery::maybe_node(),
            List::maybe_node(),
            Embed::maybe_node(),
            Accordion::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<AccordionTabNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        6 + self.header.as_ref().map_or(0, |header| header.len() + 3)
            + if self.consumed_all_input { 0 } else { 1 }
    }
}

impl Deserializer for AccordionTab {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(tab) = matcher.get_match(
            &[RepeatTimes(2, '/'), Once('\n')],
            &[Once('\n'), RepeatTimes(2, '\\')],
            false,
        ) {
            let mut inner_matcher = Matcher::new(tab.body);
            let header = inner_matcher
                .get_match(&[Once('/'), RepeatTimes(1, ' ')], &[Once('\n')], false)
                .map(|header| header.body);

            let consumed_all_input = matcher.get_match(&[Once('\n')], &[], false).is_none();
            return Self::parse_branch(
                inner_matcher.get_rest(),
                Self::new(consumed_all_input, header),
            );
        }
        None
    }
}

#[cfg(test)]
mod cfg {
    use pretty_assertions::assert_eq;

    use crate::{
        nodes::{
            accordion_tab::AccordionTab, heading::Heading, image::Image, paragraph::Paragraph,
            text::Text,
        },
        toolkit::{deserializer::Deserializer, node::Node},
    };

    #[test]
    fn test_accordion_tab_deserialize() {
        assert_eq!(
            AccordionTab::deserialize("//\n/ Header\n# Heading\n\\\\\n\n"),
            Some(AccordionTab::new_with_nodes(
                false,
                Some("Header"),
                vec![Heading::new(true, "Heading", 1).into()]
            ))
        );
    }

    #[test]
    fn test_accordion_tab_deserialize_with_no_header() {
        assert_eq!(
            AccordionTab::deserialize("//\nI am regular text\n\\\\\n\n"),
            Some(AccordionTab::new_with_nodes::<&str>(
                false,
                None,
                vec![
                    Paragraph::new_with_nodes(true, vec![Text::new("I am regular text").into()])
                        .into()
                ]
            ))
        );
    }

    #[test]
    fn test_accordion_tab_deserialize_with_no_header_and_no_newline() {
        assert_eq!(
            AccordionTab::deserialize("//\n![alt](url)\n\n\\\\"),
            Some(AccordionTab::new_with_nodes::<&str>(
                true,
                None,
                vec![Image::new(true, "alt", "url").into()]
            ))
        );
    }

    #[test]
    fn test_accordion_tab_len() {
        assert_eq!(
            AccordionTab::new_with_nodes(
                false,
                Some("Header"),
                vec![Heading::new(true, "Heading", 1).into()]
            )
            .len(),
            25
        );
        assert_eq!(
            AccordionTab::new_with_nodes(
                true,
                Some("Header"),
                vec![Heading::new(true, "Heading", 1).into()]
            )
            .len(),
            24
        );
        assert_eq!(AccordionTab::new(true, Some("Header")).len(), 15);
        assert_eq!(AccordionTab::new(false, Some("Header")).len(), 16);
    }

    #[test]
    fn test_accordion_tab_serialize() {
        assert_eq!(
            AccordionTab::new_with_nodes(
                false,
                Some("Header"),
                vec![Heading::new(true, "Heading", 1).into()]
            )
            .serialize(),
            "//\n/ Header\n# Heading\n\\\\\n"
        );
    }
}
