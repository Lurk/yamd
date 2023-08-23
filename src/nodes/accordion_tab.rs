use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::{
    accordion::Accordion, cloudinary_image_gallery::CloudinaryImageGallery, code::Code,
    divider::Divider, embed::Embed, heading::Heading, image::Image, image_gallery::ImageGallery,
    list::List, paragraph::Paragraph,
};

#[derive(Debug, PartialEq)]
pub enum AccordionTabNodes<'text> {
    Pargaraph(Paragraph),
    Heading(Heading<'text>),
    Image(Image),
    ImageGallery(ImageGallery),
    CloudinaryImageGallery(CloudinaryImageGallery),
    List(List),
    Embed(Embed),
    Accordion(Accordion<'text>),
    Divider(Divider),
    Code(Code),
}

impl Node<'_> for AccordionTabNodes<'_> {
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
            AccordionTabNodes::Divider(node) => node.serialize(),
            AccordionTabNodes::Code(node) => node.serialize(),
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
            AccordionTabNodes::Divider(node) => node.len(),
            AccordionTabNodes::Code(node) => node.len(),
        }
    }
}

impl From<Paragraph> for AccordionTabNodes<'_> {
    fn from(value: Paragraph) -> Self {
        Self::Pargaraph(value)
    }
}

impl<'text> From<Heading<'text>> for AccordionTabNodes<'text> {
    fn from(value: Heading<'text>) -> Self {
        Self::Heading(value)
    }
}

impl From<Image> for AccordionTabNodes<'_> {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<ImageGallery> for AccordionTabNodes<'_> {
    fn from(value: ImageGallery) -> Self {
        Self::ImageGallery(value)
    }
}

impl From<CloudinaryImageGallery> for AccordionTabNodes<'_> {
    fn from(value: CloudinaryImageGallery) -> Self {
        Self::CloudinaryImageGallery(value)
    }
}

impl From<List> for AccordionTabNodes<'_> {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<Embed> for AccordionTabNodes<'_> {
    fn from(value: Embed) -> Self {
        Self::Embed(value)
    }
}

impl<'text> From<Accordion<'text>> for AccordionTabNodes<'text> {
    fn from(value: Accordion<'text>) -> Self {
        Self::Accordion(value)
    }
}

impl From<Divider> for AccordionTabNodes<'_> {
    fn from(value: Divider) -> Self {
        Self::Divider(value)
    }
}

impl From<Code> for AccordionTabNodes<'_> {
    fn from(value: Code) -> Self {
        Self::Code(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct AccordionTab<'text> {
    pub header: Option<&'text str>,
    pub nodes: Vec<AccordionTabNodes<'text>>,
    consumed_all_input: bool,
}

impl<'text> AccordionTab<'text> {
    pub fn new(consumed_all_input: bool, header: Option<&'text str>) -> Self {
        Self::new_with_nodes(consumed_all_input, header, vec![])
    }
    pub fn new_with_nodes(
        consumed_all_input: bool,
        header: Option<&'text str>,
        nodes: Vec<AccordionTabNodes<'text>>,
    ) -> Self {
        Self {
            nodes,
            consumed_all_input,
            header,
        }
    }
}

impl Node<'_> for AccordionTab<'_> {
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

impl<'text> Branch<'text, AccordionTabNodes<'text>> for AccordionTab<'text> {
    fn push<CanBeNode: Into<AccordionTabNodes<'text>>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<'text, AccordionTabNodes<'text>>> {
        vec![
            Heading::maybe_node(),
            Image::maybe_node(),
            ImageGallery::maybe_node(),
            CloudinaryImageGallery::maybe_node(),
            List::maybe_node(),
            Embed::maybe_node(),
            Accordion::maybe_node(),
            Divider::maybe_node(),
            Code::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<AccordionTabNodes<'text>>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        6 + self.header.as_ref().map_or(0, |header| header.len() + 3)
            + if self.consumed_all_input { 0 } else { 1 }
    }
}

impl<'text> Deserializer<'text> for AccordionTab<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(tab) = matcher.get_match("//\n", "\n\\\\", false) {
            let mut inner_matcher = Matcher::new(tab.body);
            let header = inner_matcher
                .get_match("/ ", "\n", false)
                .map(|header| header.body);

            let consumed_all_input = matcher.get_match("\n", "", false).is_none();
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
            accordion_tab::AccordionTab, bold::Bold,
            cloudinary_image_gallery::CloudinaryImageGallery, code::Code, divider::Divider,
            embed::Embed, heading::Heading, image::Image, image_gallery::ImageGallery, list::List,
            list::ListTypes::*, list_item::ListItem, list_item_content::ListItemContent,
            paragraph::Paragraph, text::Text,
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
            Some(AccordionTab::new_with_nodes(
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
            Some(AccordionTab::new_with_nodes(
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

    #[test]
    fn fail_to_deseiralize_accordion_tab() {
        assert_eq!(AccordionTab::deserialize("I am not an accordion tab"), None);
    }

    #[test]
    fn with_all_nodes() {
        let input = r#"//
/ Header
# hello

```rust
let a=1;
```

t**b**

![a](u)

!!!
![a](u)
![a2](u2)
!!!

-----

- one
 - two

{{youtube|123}}

!!!!
! username
! tag
!!!!
\\"#;
        let tab = AccordionTab::new_with_nodes(
            true,
            Some("Header"),
            vec![
                Heading::new(false, "hello", 1).into(),
                Code::new(false, "rust", "let a=1;").into(),
                Paragraph::new_with_nodes(
                    false,
                    vec![
                        Text::new("t").into(),
                        Bold::new_with_nodes(vec![Text::new("b").into()]).into(),
                    ],
                )
                .into(),
                Image::new(false, 'a', 'u').into(),
                ImageGallery::new_with_nodes(
                    vec![
                        Image::new(true, "a", "u").into(),
                        Image::new(true, "a2", "u2").into(),
                    ],
                    false,
                )
                .into(),
                Divider::new(false).into(),
                List::new_with_nodes(
                    false,
                    Unordered,
                    0,
                    vec![ListItem::new_with_nested_list(
                        Unordered,
                        0,
                        ListItemContent::new_with_nodes(false, vec![Text::new("one").into()]),
                        Some(List::new_with_nodes(
                            true,
                            Unordered,
                            1,
                            vec![ListItem::new(
                                Unordered,
                                1,
                                ListItemContent::new_with_nodes(
                                    true,
                                    vec![Text::new("two").into()],
                                ),
                            )
                            .into()],
                        )),
                    )
                    .into()],
                )
                .into(),
                Embed::new("youtube", "123", false).into(),
                CloudinaryImageGallery::new("username", "tag", true).into(),
            ],
        );
        assert_eq!(tab.serialize(), input);
        assert_eq!(AccordionTab::deserialize(input), Some(tab));
    }
}
