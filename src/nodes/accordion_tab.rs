use std::fmt::Display;

use serde::Serialize;

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

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum AccordionTabNodes {
    Pargaraph(Paragraph),
    Heading(Heading),
    Image(Image),
    ImageGallery(ImageGallery),
    CloudinaryImageGallery(CloudinaryImageGallery),
    List(List),
    Embed(Embed),
    Accordion(Accordion),
    Divider(Divider),
    Code(Code),
}

impl Display for AccordionTabNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccordionTabNodes::Pargaraph(node) => write!(f, "{}", node),
            AccordionTabNodes::Heading(node) => write!(f, "{}", node),
            AccordionTabNodes::Image(node) => write!(f, "{}", node),
            AccordionTabNodes::ImageGallery(node) => write!(f, "{}", node),
            AccordionTabNodes::CloudinaryImageGallery(node) => write!(f, "{}", node),
            AccordionTabNodes::List(node) => write!(f, "{}", node),
            AccordionTabNodes::Embed(node) => write!(f, "{}", node),
            AccordionTabNodes::Accordion(node) => write!(f, "{}", node),
            AccordionTabNodes::Divider(node) => write!(f, "{}", node),
            AccordionTabNodes::Code(node) => write!(f, "{}", node),
        }
    }
}

impl Node for AccordionTabNodes {
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

impl From<Divider> for AccordionTabNodes {
    fn from(value: Divider) -> Self {
        Self::Divider(value)
    }
}

impl From<Code> for AccordionTabNodes {
    fn from(value: Code) -> Self {
        Self::Code(value)
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct AccordionTab {
    pub header: Option<String>,
    pub nodes: Vec<AccordionTabNodes>,
    #[serde(skip_serializing)]
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

impl Display for AccordionTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "//\n{header}{nodes}\n\\\\{end}",
            header = self
                .header
                .as_ref()
                .map_or("".to_string(), |header| format!("/ {}\n", header)),
            nodes = self
                .nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join(""),
            end = if self.consumed_all_input { "" } else { "\n" }
        )
    }
}

impl Node for AccordionTab {
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
            Divider::maybe_node(),
            Code::maybe_node(),
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
            .to_string(),
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
        assert_eq!(tab.to_string(), input);
        assert_eq!(AccordionTab::deserialize(input), Some(tab));
    }
}
