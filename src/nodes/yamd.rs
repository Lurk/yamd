use crate::{
    nodes::heading::Heading,
    nodes::paragraph::Paragraph,
    toolkit::deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    toolkit::{context::Context, node::Node},
};

use super::{
    cloudinary_image_gallery::CloudinaryImageGallery, code::Code, divider::Divider, embed::Embed,
    highlight::Highlight, image::Image, image_gallery::ImageGallery, list::List,
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
        }
    }
}

/// Yamd is a parent node for every node.
#[derive(Debug, PartialEq)]
pub struct Yamd {
    nodes: Vec<YamdNodes>,
}

impl Yamd {
    pub fn new() -> Self {
        Self::new_with_nodes(vec![])
    }

    pub fn new_with_nodes(nodes: Vec<YamdNodes>) -> Self {
        Self { nodes }
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
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<YamdNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        0
    }
}

impl Deserializer for Yamd {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        Self::parse_branch(input, Self::new())
    }
}

impl Default for Yamd {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Yamd {
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .join("")
    }
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Yamd;
    use crate::{
        nodes::heading::Heading,
        nodes::paragraph::Paragraph,
        nodes::{
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
            strikethrough::Strikethrough,
            text::Text,
        },
        toolkit::deserializer::Branch,
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;
    const TEST_CASE: &str = r#"# hello

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

end"#;

    #[test]
    fn push() {
        let mut t = Yamd::new();
        t.push(Heading::new("header", 1, false));
        t.push(Paragraph::new_with_nodes(
            true,
            vec![Text::new("text").into()],
        ));

        assert_eq!(t.serialize(), "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Yamd::new_with_nodes(vec![
            Heading::new("header", 1, false).into(),
            Paragraph::new_with_nodes(true, vec![Text::new("text").into()]).into(),
        ])
        .serialize();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Yamd::deserialize(TEST_CASE),
            Some(Yamd::new_with_nodes(vec![
                Heading::new("hello", 1, false).into(),
                Code::new("rust", "let a=1;", false).into(),
                Paragraph::new_with_nodes(
                    false,
                    vec![
                        Text::new("t").into(),
                        Bold::new_with_nodes(vec![Text::new("b").into()]).into()
                    ]
                )
                .into(),
                Image::new('a', 'u', false).into(),
                ImageGallery::new_with_nodes(
                    vec![
                        Image::new("a", "u", true).into(),
                        Image::new("a2", "u2", true).into()
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
                    Unordered,
                    0,
                    false,
                    vec![ListItem::new_with_nodes(
                        Unordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("one").into()]).into(),
                            List::new_with_nodes(
                                Unordered,
                                1,
                                true,
                                vec![ListItem::new_with_nodes(
                                    Unordered,
                                    1,
                                    vec![Paragraph::new_with_nodes(
                                        true,
                                        vec![Text::new("two").into()]
                                    )
                                    .into()]
                                )
                                .into()]
                            )
                            .into()
                        ]
                    )
                    .into()]
                )
                .into(),
                Embed::new("youtube", "123", false).into(),
                CloudinaryImageGallery::new("username", "tag", false).into(),
                Paragraph::new_with_nodes(true, vec![Text::new("end").into()]).into()
            ]))
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Yamd::new_with_nodes(vec![
                Heading::new("hello", 1, false).into(),
                Code::new("rust", "let a=1;", false).into(),
                Paragraph::new_with_nodes(
                    false,
                    vec![
                        Text::new("t").into(),
                        Bold::new_with_nodes(vec![Text::new("b").into()]).into()
                    ]
                )
                .into(),
                Image::new('a', 'u', false).into(),
                ImageGallery::new_with_nodes(
                    vec![
                        Image::new("a", "u", true).into(),
                        Image::new("a2", "u2", true).into()
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
                    Unordered,
                    0,
                    false,
                    vec![ListItem::new_with_nodes(
                        Unordered,
                        0,
                        vec![
                            Paragraph::new_with_nodes(true, vec![Text::new("one").into()]).into(),
                            List::new_with_nodes(
                                Unordered,
                                1,
                                true,
                                vec![ListItem::new_with_nodes(
                                    Unordered,
                                    1,
                                    vec![Paragraph::new_with_nodes(
                                        true,
                                        vec![Text::new("two").into()]
                                    )
                                    .into()]
                                )
                                .into()]
                            )
                            .into()
                        ]
                    )
                    .into()]
                )
                .into(),
                Embed::new("youtube", "123", false).into(),
                CloudinaryImageGallery::new("username", "tag", false).into(),
                Paragraph::new_with_nodes(true, vec![Text::new("end").into()]).into()
            ])
            .serialize(),
            String::from(TEST_CASE)
        )
    }
}
