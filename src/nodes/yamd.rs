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
pub enum YamdNodes {
    P(Paragraph),
    H(Heading),
    Image(Image),
    Code(Code),
    List(List),
    ImageGallery(ImageGallery),
    Highlight(Highlight),
    Divider(Divider),
    Embed(Embed),
    CloudinaryImageGallery(CloudinaryImageGallery),
    Accordion(Accordion),
}

impl From<Paragraph> for YamdNodes {
    fn from(value: Paragraph) -> Self {
        YamdNodes::P(value)
    }
}

impl From<Heading> for YamdNodes {
    fn from(value: Heading) -> Self {
        YamdNodes::H(value)
    }
}

impl From<Image> for YamdNodes {
    fn from(value: Image) -> Self {
        YamdNodes::Image(value)
    }
}

impl From<Code> for YamdNodes {
    fn from(value: Code) -> Self {
        YamdNodes::Code(value)
    }
}

impl From<List> for YamdNodes {
    fn from(value: List) -> Self {
        YamdNodes::List(value)
    }
}

impl From<ImageGallery> for YamdNodes {
    fn from(value: ImageGallery) -> Self {
        YamdNodes::ImageGallery(value)
    }
}

impl From<Highlight> for YamdNodes {
    fn from(value: Highlight) -> Self {
        YamdNodes::Highlight(value)
    }
}

impl From<Divider> for YamdNodes {
    fn from(value: Divider) -> Self {
        YamdNodes::Divider(value)
    }
}

impl From<Embed> for YamdNodes {
    fn from(value: Embed) -> Self {
        YamdNodes::Embed(value)
    }
}

impl From<CloudinaryImageGallery> for YamdNodes {
    fn from(value: CloudinaryImageGallery) -> Self {
        YamdNodes::CloudinaryImageGallery(value)
    }
}

impl From<Accordion> for YamdNodes {
    fn from(value: Accordion) -> Self {
        YamdNodes::Accordion(value)
    }
}

impl Node for YamdNodes {
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
pub struct Yamd {
    metadata: Option<Metadata>,
    nodes: Vec<YamdNodes>,
}

impl Yamd {
    pub fn new(metadata: Option<Metadata>) -> Self {
        Self::new_with_nodes(metadata, vec![])
    }

    pub fn new_with_nodes(metadata: Option<Metadata>, nodes: Vec<YamdNodes>) -> Self {
        Self { metadata, nodes }
    }
}
impl Branch<YamdNodes> for Yamd {
    fn push<TC: Into<YamdNodes>>(&mut self, element: TC) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<YamdNodes>> {
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

    fn get_fallback_node() -> Option<DefinitelyNode<YamdNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        self.metadata.as_ref().map(|m| m.len()).unwrap_or(0)
    }
}

impl Deserializer for Yamd {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let metadata = Metadata::deserialize(input);
        let metadata_len = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        Self::parse_branch(&input[metadata_len..], Self::new(metadata))
    }
}

impl Default for Yamd {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Node for Yamd {
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
                    Code::new("rust", "let a=1;", false).into(),
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
                        Some("H"),
                        Some("I"),
                        false,
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
                    Embed::new("youtube", "123", false).into(),
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
                    Code::new("rust", "let a=1;", false).into(),
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
                        Some("H"),
                        Some("I"),
                        false,
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
                    Embed::new("youtube", "123", false).into(),
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
