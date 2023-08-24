use crate::{
    nodes::heading::Heading,
    nodes::paragraph::Paragraph,
    toolkit::deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    toolkit::{context::Context, node::Node},
};

use super::{
    accordion::Accordion, cloudinary_image_gallery::CloudinaryImageGallery, code::Code,
    divider::Divider, embed::Embed, highlight::Highlight, image::Image,
    image_gallery::ImageGallery, list::List, metadata::Metadata,
};

#[derive(Debug, PartialEq)]
pub enum YamdNodes<'text> {
    P(Paragraph<'text>),
    H(Heading<'text>),
    Image(Image),
    Code(Code<'text>),
    List(List<'text>),
    ImageGallery(ImageGallery),
    Highlight(Highlight<'text>),
    Divider(Divider),
    Embed(Embed<'text>),
    CloudinaryImageGallery(CloudinaryImageGallery<'text>),
    Accordion(Accordion<'text>),
}

impl<'text> From<Paragraph<'text>> for YamdNodes<'text> {
    fn from(value: Paragraph<'text>) -> Self {
        YamdNodes::P(value)
    }
}

impl<'text> From<Heading<'text>> for YamdNodes<'text> {
    fn from(value: Heading<'text>) -> Self {
        YamdNodes::H(value)
    }
}

impl From<Image> for YamdNodes<'_> {
    fn from(value: Image) -> Self {
        YamdNodes::Image(value)
    }
}

impl<'text> From<Code<'text>> for YamdNodes<'text> {
    fn from(value: Code<'text>) -> Self {
        YamdNodes::Code(value)
    }
}

impl<'text> From<List<'text>> for YamdNodes<'text> {
    fn from(value: List<'text>) -> Self {
        YamdNodes::List(value)
    }
}

impl From<ImageGallery> for YamdNodes<'_> {
    fn from(value: ImageGallery) -> Self {
        YamdNodes::ImageGallery(value)
    }
}

impl<'text> From<Highlight<'text>> for YamdNodes<'text> {
    fn from(value: Highlight<'text>) -> Self {
        YamdNodes::Highlight(value)
    }
}

impl From<Divider> for YamdNodes<'_> {
    fn from(value: Divider) -> Self {
        YamdNodes::Divider(value)
    }
}

impl<'text> From<Embed<'text>> for YamdNodes<'text> {
    fn from(value: Embed<'text>) -> Self {
        YamdNodes::Embed(value)
    }
}

impl<'text> From<CloudinaryImageGallery<'text>> for YamdNodes<'text> {
    fn from(value: CloudinaryImageGallery<'text>) -> Self {
        YamdNodes::CloudinaryImageGallery(value)
    }
}

impl<'text> From<Accordion<'text>> for YamdNodes<'text> {
    fn from(value: Accordion<'text>) -> Self {
        YamdNodes::Accordion(value)
    }
}

impl Node<'_> for YamdNodes<'_> {
    fn serialize(&self) -> String {
        match self {
            YamdNodes::P(node) => node.serialize(),
            YamdNodes::H(node) => node.serialize(),
            YamdNodes::Image(node) => node.serialize(),
            YamdNodes::Code(node) => node.serialize(),
            YamdNodes::List(node) => node.serialize(),
            YamdNodes::ImageGallery(node) => node.serialize(),
            YamdNodes::Highlight(node) => node.serialize(),
            YamdNodes::Divider(node) => node.serialize(),
            YamdNodes::Embed(node) => node.serialize(),
            YamdNodes::CloudinaryImageGallery(node) => node.serialize(),
            YamdNodes::Accordion(node) => node.serialize(),
        }
    }
    fn len(&self) -> usize {
        match self {
            YamdNodes::P(node) => node.len(),
            YamdNodes::H(node) => node.len(),
            YamdNodes::Image(node) => node.len(),
            YamdNodes::Code(node) => node.len(),
            YamdNodes::List(node) => node.len(),
            YamdNodes::ImageGallery(node) => node.len(),
            YamdNodes::Highlight(node) => node.len(),
            YamdNodes::Divider(node) => node.len(),
            YamdNodes::Embed(node) => node.len(),
            YamdNodes::CloudinaryImageGallery(node) => node.len(),
            YamdNodes::Accordion(node) => node.len(),
        }
    }
}

/// Yamd is a parent node for every node.
#[derive(Debug, PartialEq)]
pub struct Yamd<'text> {
    pub metadata: Option<Metadata>,
    pub nodes: Vec<YamdNodes<'text>>,
}

impl<'text> Yamd<'text> {
    pub fn new(metadata: Option<Metadata>) -> Self {
        Self::new_with_nodes(metadata, vec![])
    }

    pub fn new_with_nodes(metadata: Option<Metadata>, nodes: Vec<YamdNodes<'text>>) -> Self {
        Self { metadata, nodes }
    }
}
impl<'text> Branch<'text, YamdNodes<'text>> for Yamd<'text> {
    fn push<TC: Into<YamdNodes<'text>>>(&mut self, element: TC) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<'text, YamdNodes<'text>>> {
        vec![
            Heading::maybe_node(),
            Image::maybe_node(),
            Code::maybe_node(),
            List::maybe_node(),
            ImageGallery::maybe_node(),
            Highlight::maybe_node(),
            Divider::maybe_node(),
            Embed::maybe_node(),
            CloudinaryImageGallery::maybe_node(),
            Accordion::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<'text, YamdNodes<'text>>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        self.metadata.as_ref().map(|m| m.len()).unwrap_or(0)
    }
}

impl<'text> Deserializer<'text> for Yamd<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let metadata = Metadata::deserialize(input);
        let metadata_len = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        Self::parse_branch(&input[metadata_len..], Self::new(metadata))
    }
}

impl Default for Yamd<'_> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Node<'_> for Yamd<'_> {
    fn serialize(&self) -> String {
        format!(
            "{}{}",
            self.metadata
                .as_ref()
                .map(|m| m.serialize())
                .unwrap_or("".to_string()),
            self.nodes
                .iter()
                .map(|node| node.serialize())
                .collect::<Vec<String>>()
                .join("")
        )
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

#[cfg(test)]
mod tests {
    use super::Yamd;
    use crate::{
        nodes::heading::Heading,
        nodes::paragraph::Paragraph,
        nodes::{
            accordion::Accordion,
            accordion_tab::AccordionTab,
            bold::Bold,
            cloudinary_image_gallery::CloudinaryImageGallery,
            code::Code,
            divider::Divider,
            embed::Embed,
            highlight::Highlight,
            image::Image,
            image_gallery::ImageGallery,
            italic::Italic,
            list::{List, ListTypes::Unordered},
            list_item::ListItem,
            list_item_content::ListItemContent,
            metadata::Metadata,
            strikethrough::Strikethrough,
            text::Text,
        },
        toolkit::deserializer::Branch,
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use chrono::DateTime;
    use pretty_assertions::assert_eq;
    const TEST_CASE: &str = r#"header: test
timestamp: 2022-01-01 00:00:00 +02:00
image: image
preview: preview
tags: tag1, tag2
^^^

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

>>>
>> H
> I
~~s~~

_I_
>>>

-----

- one
 - two

{{youtube|123}}

!!!!
! username
! tag
!!!!

///
//
/ accordeon tab

\\
//
/ one more accordeon tab

\\
\\\

end"#;

    #[test]
    fn push() {
        let mut t = Yamd::new(None);
        t.push(Heading::new(false, "header", 1));
        t.push(Paragraph::new_with_nodes(
            true,
            vec![Text::new("text").into()],
        ));

        assert_eq!(t.serialize(), "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Yamd::new_with_nodes(
            None,
            vec![
                Heading::new(false, "header", 1).into(),
                Paragraph::new_with_nodes(true, vec![Text::new("text").into()]).into(),
            ],
        )
        .serialize();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Yamd::deserialize(TEST_CASE),
            Some(Yamd::new_with_nodes(
                Some(Metadata::new(
                    Some("test"),
                    Some(
                        DateTime::parse_from_str(
                            "2022-01-01 00:00:00 +02:00",
                            "%Y-%m-%d %H:%M:%S %z"
                        )
                        .unwrap()
                    ),
                    Some("image"),
                    Some("preview"),
                    Some(vec!["tag1", "tag2"]),
                )),
                vec![
                    Heading::new(false, "hello", 1).into(),
                    Code::new(false, "rust", "let a=1;").into(),
                    Paragraph::new_with_nodes(
                        false,
                        vec![
                            Text::new("t").into(),
                            Bold::new_with_nodes(vec![Text::new("b").into()]).into()
                        ]
                    )
                    .into(),
                    Image::new(false, 'a', 'u').into(),
                    ImageGallery::new_with_nodes(
                        vec![
                            Image::new(true, "a", "u").into(),
                            Image::new(true, "a2", "u2").into()
                        ],
                        false
                    )
                    .into(),
                    Highlight::new_with_nodes(
                        false,
                        Some("H"),
                        Some("I"),
                        vec![
                            Paragraph::new_with_nodes(false, vec![Strikethrough::new("s").into()])
                                .into(),
                            Paragraph::new_with_nodes(true, vec![Italic::new("I").into()]).into()
                        ]
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
                                        vec![Text::new("two").into()]
                                    )
                                )
                                .into()]
                            ))
                        )
                        .into()]
                    )
                    .into(),
                    Embed::new(false, "youtube", "123").into(),
                    CloudinaryImageGallery::new("username", "tag", false).into(),
                    Accordion::new_with_nodes(
                        false,
                        vec![
                            AccordionTab::new(false, Some("accordeon tab"),).into(),
                            AccordionTab::new(true, Some("one more accordeon tab"),).into()
                        ]
                    )
                    .into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("end").into()]).into()
                ]
            ))
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Yamd::new_with_nodes(
                Some(Metadata::new(
                    Some("test"),
                    Some(
                        DateTime::parse_from_str(
                            "2022-01-01 00:00:00 +02:00",
                            "%Y-%m-%d %H:%M:%S %z"
                        )
                        .unwrap()
                    ),
                    Some("image"),
                    Some("preview"),
                    Some(vec!["tag1", "tag2"]),
                )),
                vec![
                    Heading::new(false, "hello", 1).into(),
                    Code::new(false, "rust", "let a=1;").into(),
                    Paragraph::new_with_nodes(
                        false,
                        vec![
                            Text::new("t").into(),
                            Bold::new_with_nodes(vec![Text::new("b").into()]).into()
                        ]
                    )
                    .into(),
                    Image::new(false, 'a', 'u').into(),
                    ImageGallery::new_with_nodes(
                        vec![
                            Image::new(true, "a", "u").into(),
                            Image::new(true, "a2", "u2").into()
                        ],
                        false
                    )
                    .into(),
                    Highlight::new_with_nodes(
                        false,
                        Some("H"),
                        Some("I"),
                        vec![
                            Paragraph::new_with_nodes(false, vec![Strikethrough::new("s").into()])
                                .into(),
                            Paragraph::new_with_nodes(true, vec![Italic::new("I").into()]).into()
                        ]
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
                            ListItemContent::new_with_nodes(false, vec![Text::new("one").into()])
                                .into(),
                            List::new_with_nodes(
                                true,
                                Unordered,
                                1,
                                vec![ListItem::new(
                                    Unordered,
                                    1,
                                    ListItemContent::new_with_nodes(
                                        true,
                                        vec![Text::new("two").into()]
                                    )
                                )
                                .into()]
                            )
                            .into()
                        )
                        .into()]
                    )
                    .into(),
                    Embed::new(false, "youtube", "123").into(),
                    CloudinaryImageGallery::new("username", "tag", false).into(),
                    Accordion::new_with_nodes(
                        false,
                        vec![
                            AccordionTab::new(false, Some("accordeon tab"),).into(),
                            AccordionTab::new(true, Some("one more accordeon tab"),).into()
                        ]
                    )
                    .into(),
                    Paragraph::new_with_nodes(true, vec![Text::new("end").into()]).into()
                ]
            )
            .serialize(),
            String::from(TEST_CASE)
        )
    }

    #[test]
    fn default() {
        assert_eq!(Yamd::default().serialize(), String::new());
    }
}
