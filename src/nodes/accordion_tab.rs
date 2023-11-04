use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::{
    accordion::Accordion, code::Code, divider::Divider, embed::Embed, heading::Heading,
    image::Image, image_gallery::ImageGallery, list::List, paragraph::Paragraph,
};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum AccordionTabNodes {
    Pargaraph(Paragraph),
    Heading(Heading),
    Image(Image),
    ImageGallery(ImageGallery),
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

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct AccordionTab {
    pub title: Option<String>,
    pub nodes: Vec<AccordionTabNodes>,
}

impl AccordionTab {
    pub fn new<S: Into<String>>(title: Option<S>, nodes: Vec<AccordionTabNodes>) -> Self {
        Self {
            nodes,
            title: title.map(|s| s.into()),
        }
    }
}

impl Display for AccordionTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "//\n{title}{nodes}\n\\\\",
            title = self
                .title
                .as_ref()
                .map_or("".to_string(), |title| format!("/ {}\n", title)),
            nodes = self
                .nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}

impl Node for AccordionTab {
    fn len(&self) -> usize {
        let delimeter_len = if self.is_empty() {
            0
        } else {
            (self.nodes.len() - 1) * 2
        };
        self.nodes.iter().map(|node| node.len()).sum::<usize>()
            + delimeter_len
            + self.get_outer_token_length()
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
        6 + self.title.as_ref().map_or(0, |header| header.len() + 3)
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Deserializer for AccordionTab {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(tab) = matcher.get_match("//\n", "\n\\\\", false) {
            let mut inner_matcher = Matcher::new(tab.body);
            let title = inner_matcher
                .get_match("/ ", "\n", false)
                .map(|header| header.body);

            return Self::parse_branch(inner_matcher.get_rest(), "\n\n", Self::new(title, vec![]));
        }
        None
    }
}

#[cfg(test)]
mod cfg {
    use pretty_assertions::assert_eq;

    use crate::{
        nodes::{
            accordion_tab::AccordionTab, bold::Bold, code::Code, divider::Divider, embed::Embed,
            heading::Heading, image::Image, image_gallery::ImageGallery, list::List,
            list::ListTypes::*, list_item::ListItem, list_item_content::ListItemContent,
            paragraph::Paragraph, text::Text,
        },
        toolkit::{
            deserializer::{Branch, Deserializer},
            node::Node,
        },
    };

    #[test]
    fn test_accordion_tab_deserialize() {
        assert_eq!(
            AccordionTab::deserialize("//\n/ Title\n# Heading\n\\\\\n\n"),
            Some(AccordionTab::new(
                Some("Title"),
                vec![Heading::new("Heading", 1).into()]
            ))
        );
    }

    #[test]
    fn test_accordion_tab_deserialize_with_no_header() {
        assert_eq!(
            AccordionTab::deserialize("//\nI am regular text\n\\\\\n\n"),
            Some(AccordionTab::new::<&str>(
                None,
                vec![Paragraph::new(vec![Text::new("I am regular text").into()]).into()]
            ))
        );
    }

    #[test]
    fn test_accordion_tab_deserialize_with_no_header_and_no_newline() {
        assert_eq!(
            AccordionTab::deserialize("//\n![alt](url)\n\\\\"),
            Some(AccordionTab::new::<&str>(
                None,
                vec![Image::new("alt", "url").into()]
            ))
        );
    }

    #[test]
    fn test_accordion_tab_len() {
        assert_eq!(
            AccordionTab::new(Some("Title"), vec![Heading::new("Heading", 1).into()]).len(),
            23
        );
        assert_eq!(AccordionTab::new(Some("Title"), vec![]).len(), 14);
    }

    #[test]
    fn test_accordion_tab_serialize() {
        assert_eq!(
            AccordionTab::new(Some("Title"), vec![Heading::new("Heading", 1).into()]).to_string(),
            "//\n/ Title\n# Heading\n\\\\"
        );
    }

    #[test]
    fn fail_to_deseiralize_accordion_tab() {
        assert_eq!(AccordionTab::deserialize("I am not an accordion tab"), None);
    }

    #[test]
    fn with_all_nodes() {
        let input = r#"//
/ Title
# hello

```rust
let a=1;
```

t**b**

![a](u)

!!!
![a2](u2)
![a3](u3)
!!!

-----

- one
 - two

{{youtube|123}}

{{cloudinary_gallery|cloud_name&tag}}
\\"#;
        let tab = AccordionTab::new(
            Some("Title"),
            vec![
                Heading::new("hello", 1).into(),
                Code::new("rust", "let a=1;").into(),
                Paragraph::new(vec![
                    Text::new("t").into(),
                    Bold::new(vec![Text::new("b").into()]).into(),
                ])
                .into(),
                Image::new('a', 'u').into(),
                ImageGallery::new(vec![
                    Image::new("a2", "u2").into(),
                    Image::new("a3", "u3").into(),
                ])
                .into(),
                Divider::new().into(),
                List::new(
                    Unordered,
                    0,
                    vec![ListItem::new_with_nested_list(
                        Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("one").into()]),
                        Some(List::new(
                            Unordered,
                            1,
                            vec![ListItem::new(
                                Unordered,
                                1,
                                ListItemContent::new(vec![Text::new("two").into()]),
                            )
                            .into()],
                        )),
                    )
                    .into()],
                )
                .into(),
                Embed::new("youtube", "123").into(),
                Embed::new("cloudinary_gallery", "cloud_name&tag").into(),
            ],
        );
        assert_eq!(tab.to_string(), input);
        assert_eq!(AccordionTab::deserialize(input), Some(tab));
    }

    #[test]
    fn empty_tab() {
        let tab = AccordionTab::new::<&str>(None, vec![]);
        assert_eq!(tab.len(), 6);
        assert_eq!(tab.is_empty(), true);
    }
}
