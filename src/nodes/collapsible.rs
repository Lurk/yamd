use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::{
    code::Code, divider::Divider, embed::Embed, heading::Heading, image::Image,
    image_gallery::ImageGallery, list::List, paragraph::Paragraph,
};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum CollapsibleNodes {
    P(Paragraph),
    H(Heading),
    Image(Image),
    ImageGallery(ImageGallery),
    List(List),
    Embed(Embed),
    Divider(Divider),
    Code(Code),
    Collapsible(Collapsible),
}

impl Display for CollapsibleNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollapsibleNodes::P(node) => write!(f, "{}", node),
            CollapsibleNodes::H(node) => write!(f, "{}", node),
            CollapsibleNodes::Image(node) => write!(f, "{}", node),
            CollapsibleNodes::ImageGallery(node) => write!(f, "{}", node),
            CollapsibleNodes::List(node) => write!(f, "{}", node),
            CollapsibleNodes::Embed(node) => write!(f, "{}", node),
            CollapsibleNodes::Divider(node) => write!(f, "{}", node),
            CollapsibleNodes::Code(node) => write!(f, "{}", node),
            CollapsibleNodes::Collapsible(node) => write!(f, "{}", node),
        }
    }
}

impl Node for CollapsibleNodes {
    fn len(&self) -> usize {
        match self {
            CollapsibleNodes::P(node) => node.len(),
            CollapsibleNodes::H(node) => node.len(),
            CollapsibleNodes::Image(node) => node.len(),
            CollapsibleNodes::ImageGallery(node) => node.len(),
            CollapsibleNodes::List(node) => node.len(),
            CollapsibleNodes::Embed(node) => node.len(),
            CollapsibleNodes::Divider(node) => node.len(),
            CollapsibleNodes::Code(node) => node.len(),
            CollapsibleNodes::Collapsible(node) => node.len(),
        }
    }
}

impl From<Paragraph> for CollapsibleNodes {
    fn from(value: Paragraph) -> Self {
        Self::P(value)
    }
}

impl From<Heading> for CollapsibleNodes {
    fn from(value: Heading) -> Self {
        Self::H(value)
    }
}

impl From<Image> for CollapsibleNodes {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<ImageGallery> for CollapsibleNodes {
    fn from(value: ImageGallery) -> Self {
        Self::ImageGallery(value)
    }
}

impl From<List> for CollapsibleNodes {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<Embed> for CollapsibleNodes {
    fn from(value: Embed) -> Self {
        Self::Embed(value)
    }
}

impl From<Divider> for CollapsibleNodes {
    fn from(value: Divider) -> Self {
        Self::Divider(value)
    }
}

impl From<Code> for CollapsibleNodes {
    fn from(value: Code) -> Self {
        Self::Code(value)
    }
}

impl From<Collapsible> for CollapsibleNodes {
    fn from(value: Collapsible) -> Self {
        Self::Collapsible(value)
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Collapsible {
    pub title: String,
    pub nodes: Vec<CollapsibleNodes>,
}

impl Collapsible {
    pub fn new<S: Into<String>>(title: S, nodes: Vec<CollapsibleNodes>) -> Self {
        Self {
            nodes,
            title: title.into(),
        }
    }
}

impl Display for Collapsible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{% {title}\n{nodes}\n%}}",
            title = self.title,
            nodes = self
                .nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}

impl Node for Collapsible {
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

impl Branch<CollapsibleNodes> for Collapsible {
    fn push<CanBeNode: Into<CollapsibleNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<CollapsibleNodes>> {
        vec![
            Heading::maybe_node(),
            Image::maybe_node(),
            ImageGallery::maybe_node(),
            List::maybe_node(),
            Embed::maybe_node(),
            Divider::maybe_node(),
            Code::maybe_node(),
            Collapsible::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<CollapsibleNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        7 + self.title.len()
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Deserializer for Collapsible {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(tab) = matcher.get_match("{% ", "\n%}", false) {
            let mut inner_matcher = Matcher::new(tab.body);
            if let Some(title) = inner_matcher.get_match("", "\n", false) {
                return Self::parse_branch(
                    inner_matcher.get_rest(),
                    "\n\n",
                    Self::new(title.body, vec![]),
                );
            }
        }
        None
    }
}

#[cfg(test)]
mod cfg {
    use pretty_assertions::assert_eq;

    use crate::{
        nodes::{
            bold::Bold, code::Code, collapsible::Collapsible, divider::Divider, embed::Embed,
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
    fn test_collapsible_deserialize() {
        assert_eq!(
            Collapsible::deserialize("{% Title\n# Heading\n%}"),
            Some(Collapsible::new(
                "Title",
                vec![Heading::new("Heading", 1).into()]
            ))
        );
    }

    #[test]
    fn test_collapsible_len() {
        assert_eq!(
            Collapsible::new("Title", vec![Heading::new("Heading", 1).into()]).len(),
            21
        );
        assert_eq!(Collapsible::new("Title", vec![]).len(), 12);
    }

    #[test]
    fn test_collapsible_serialize() {
        assert_eq!(
            Collapsible::new("Title", vec![Heading::new("Heading", 1).into()]).to_string(),
            "{% Title\n# Heading\n%}"
        );
    }

    #[test]
    fn fail_to_deseiralize_collapsible() {
        assert_eq!(Collapsible::deserialize("I am not an accordion tab"), None);
        assert_eq!(Collapsible::deserialize("{% \n%}"), None);
    }

    #[test]
    fn with_all_nodes() {
        let input = r#"{% Title
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

{% nested collapsible
# nested
%}
%}"#;
        let tab = Collapsible::new(
            "Title",
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
                Collapsible::new("nested collapsible", vec![Heading::new("nested", 1).into()])
                    .into(),
            ],
        );
        assert_eq!(tab.to_string(), input);
        assert_eq!(Collapsible::deserialize(input), Some(tab));
    }

    #[test]
    fn empty_tab() {
        let tab = Collapsible::new::<&str>("", vec![]);
        assert_eq!(tab.len(), 7);
        assert_eq!(tab.is_empty(), true);
    }
}
