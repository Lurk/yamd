use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    parser::{parse_to_consumer, parse_to_parser, Branch, Consumer, Parse, Parser},
};

use super::{
    code::Code, collapsible::Collapsible, divider::Divider, embed::Embed, heading::Heading,
    highlight::Highlight, image::Image, image_gallery::ImageGallery, list::List,
    metadata::Metadata, paragraph::Paragraph,
};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
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
    Collapsible(Collapsible),
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

impl From<Collapsible> for YamdNodes {
    fn from(value: Collapsible) -> Self {
        YamdNodes::Collapsible(value)
    }
}

impl Display for YamdNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YamdNodes::P(node) => write!(f, "{}", node),
            YamdNodes::H(node) => write!(f, "{}", node),
            YamdNodes::Image(node) => write!(f, "{}", node),
            YamdNodes::Code(node) => write!(f, "{}", node),
            YamdNodes::List(node) => write!(f, "{}", node),
            YamdNodes::ImageGallery(node) => write!(f, "{}", node),
            YamdNodes::Highlight(node) => write!(f, "{}", node),
            YamdNodes::Divider(node) => write!(f, "{}", node),
            YamdNodes::Embed(node) => write!(f, "{}", node),
            YamdNodes::Collapsible(node) => write!(f, "{}", node),
        }
    }
}

/// Yamd is a parent node for every node.
#[derive(Debug, PartialEq, Serialize, Clone, Default)]
pub struct Yamd {
    pub metadata: Option<Metadata>,
    pub nodes: Vec<YamdNodes>,
}

impl Yamd {
    pub fn new(metadata: Option<Metadata>, nodes: Vec<YamdNodes>) -> Self {
        Self { metadata, nodes }
    }
}

impl Display for Yamd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.metadata
                .as_ref()
                .map_or(String::new(), |m| format!("{m}\n\n")),
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}

impl Branch<YamdNodes> for Yamd {
    fn get_parsers(&self) -> Vec<Parser<YamdNodes>> {
        vec![
            parse_to_parser::<YamdNodes, Heading>(),
            parse_to_parser::<YamdNodes, Image>(),
            parse_to_parser::<YamdNodes, ImageGallery>(),
            parse_to_parser::<YamdNodes, List>(),
            parse_to_parser::<YamdNodes, Highlight>(),
            parse_to_parser::<YamdNodes, Divider>(),
            parse_to_parser::<YamdNodes, Embed>(),
            parse_to_parser::<YamdNodes, Collapsible>(),
            parse_to_parser::<YamdNodes, Code>(),
        ]
    }

    fn get_consumer(&self) -> Option<Consumer<YamdNodes>> {
        Some(parse_to_consumer::<YamdNodes, Paragraph>())
    }

    fn push_node(&mut self, node: YamdNodes) {
        self.nodes.push(node);
    }
}

impl Parse for Yamd {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        let (metadata, consumed_length) = Metadata::parse(input, current_position, None)
            .map_or((None, 0), |(m, l)| (Some(m), l + 2));

        let yamd = Self::new(metadata, vec![]);
        let yamd = yamd
            .parse_branch(&input[current_position + consumed_length..], "\n\n", None)
            .expect("yamd should never fail");
        Some((yamd, input.len() - current_position))
    }
}

#[cfg(test)]
mod tests {
    use super::Yamd;
    use crate::{
        nodes::{
            bold::Bold,
            code::Code,
            collapsible::Collapsible,
            divider::Divider,
            embed::Embed,
            heading::Heading,
            highlight::Highlight,
            image::Image,
            image_gallery::ImageGallery,
            italic::Italic,
            list::{List, ListTypes::Unordered},
            list_item::ListItem,
            metadata::Metadata,
            paragraph::Paragraph,
            strikethrough::Strikethrough,
            text::Text,
        },
        toolkit::parser::{Branch, Parse},
    };
    use chrono::DateTime;
    use pretty_assertions::assert_eq;
    const TEST_CASE: &str = r#"---
title: test
date: 2022-01-01T00:00:00+02:00
image: image
preview: preview
tags:
- tag1
- tag2
---

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

{{cloudinary_gallery|cloud_name&tag}}

{% collapsible

%}

{% one more collapsible

%}

end"#;

    #[test]
    fn push() {
        let mut t = Yamd::new(None, vec![]);
        t.push_node(Heading::new(1, vec![Text::new("header").into()]).into());
        t.push_node(Paragraph::new(vec![Text::new("text").into()]).into());

        assert_eq!(t.to_string(), "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Yamd::new(
            None,
            vec![
                Heading::new(1, vec![Text::new("header").into()]).into(),
                Paragraph::new(vec![Text::new("text").into()]).into(),
            ],
        )
        .to_string();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn parse() {
        assert_eq!(
            Yamd::parse(TEST_CASE, 0, None),
            Some((
                Yamd::new(
                    Some(Metadata {
                        title: Some("test".to_string()),
                        date: Some(
                            DateTime::parse_from_str(
                                "2022-01-01 00:00:00 +02:00",
                                "%Y-%m-%d %H:%M:%S %z"
                            )
                            .unwrap()
                        ),
                        image: Some("image".to_string()),
                        preview: Some("preview".to_string()),
                        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
                        is_draft: None,
                    }),
                    vec![
                        Heading::new(1, vec![Text::new("hello").into()]).into(),
                        Code::new("rust", "let a=1;").into(),
                        Paragraph::new(vec![
                            Text::new("t").into(),
                            Bold::new(vec![Text::new("b").into()]).into()
                        ])
                        .into(),
                        Image::new('a', 'u').into(),
                        ImageGallery::new(vec![
                            Image::new("a", "u").into(),
                            Image::new("a2", "u2").into()
                        ],)
                        .into(),
                        Highlight::new(
                            Some("H"),
                            Some("I"),
                            vec![
                                Paragraph::new(vec![Strikethrough::new("s").into()]),
                                Paragraph::new(vec![Italic::new("I").into()])
                            ]
                        )
                        .into(),
                        Divider::new().into(),
                        List::new(
                            Unordered,
                            0,
                            vec![ListItem::new(
                                Unordered,
                                0,
                                Paragraph::new(vec![Text::new("one").into()]),
                                Some(List::new(
                                    Unordered,
                                    1,
                                    vec![ListItem::new(
                                        Unordered,
                                        1,
                                        Paragraph::new(vec![Text::new("two").into()]),
                                        None
                                    )]
                                ))
                            )]
                        )
                        .into(),
                        Embed::new("youtube", "123",).into(),
                        Embed::new("cloudinary_gallery", "cloud_name&tag",).into(),
                        Collapsible::new("collapsible", vec![]).into(),
                        Collapsible::new("one more collapsible", vec![]).into(),
                        Paragraph::new(vec![Text::new("end").into()]).into()
                    ]
                ),
                TEST_CASE.len()
            ))
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Yamd::new(
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
                    Some(vec!["tag1".to_string(), "tag2".to_string()]),
                )),
                vec![
                    Heading::new(1, vec![Text::new("hello").into()]).into(),
                    Code::new("rust", "let a=1;").into(),
                    Paragraph::new(vec![
                        Text::new("t").into(),
                        Bold::new(vec![Text::new("b").into()]).into()
                    ])
                    .into(),
                    Image::new('a', 'u').into(),
                    ImageGallery::new(vec![
                        Image::new("a", "u").into(),
                        Image::new("a2", "u2").into()
                    ],)
                    .into(),
                    Highlight::new(
                        Some("H"),
                        Some("I"),
                        vec![
                            Paragraph::new(vec![Strikethrough::new("s").into()]),
                            Paragraph::new(vec![Italic::new("I").into()])
                        ]
                    )
                    .into(),
                    Divider::new().into(),
                    List::new(
                        Unordered,
                        0,
                        vec![ListItem::new(
                            Unordered,
                            0,
                            Paragraph::new(vec![Text::new("one").into()]),
                            List::new(
                                Unordered,
                                1,
                                vec![ListItem::new(
                                    Unordered,
                                    1,
                                    Paragraph::new(vec![Text::new("two").into()]),
                                    None
                                )]
                            )
                            .into()
                        )]
                    )
                    .into(),
                    Embed::new("youtube", "123",).into(),
                    Embed::new("cloudinary_gallery", "cloud_name&tag",).into(),
                    Collapsible::new("collapsible", vec![]).into(),
                    Collapsible::new("one more collapsible", vec![]).into(),
                    Paragraph::new(vec![Text::new("end").into()]).into()
                ]
            )
            .to_string(),
            String::from(TEST_CASE)
        )
    }

    #[test]
    fn default() {
        assert_eq!(Yamd::default().to_string(), String::new());
    }

    #[test]
    fn multiple_fallbacks_in_a_row() {
        let input = "1\n\n2\n\n3";
        let expected = Yamd::new(
            None,
            vec![
                Paragraph::new(vec![Text::new("1").into()]).into(),
                Paragraph::new(vec![Text::new("2").into()]).into(),
                Paragraph::new(vec![Text::new("3").into()]).into(),
            ],
        );
        let actual = Yamd::parse(input, 0, None).unwrap();
        assert_eq!(expected, actual.0);
    }

    #[test]
    fn multiple_fallbacks_in_a_row_before_non_fallback() {
        let input = "1\n\n2\n\n3\n\n# header";
        let expected = Yamd::new(
            None,
            vec![
                Paragraph::new(vec![Text::new("1").into()]).into(),
                Paragraph::new(vec![Text::new("2").into()]).into(),
                Paragraph::new(vec![Text::new("3").into()]).into(),
                Heading::new(1, vec![Text::new("header").into()]).into(),
            ],
        );
        let actual = Yamd::parse(input, 0, None).unwrap();
        assert_eq!(expected, actual.0);
    }

    #[test]
    fn node_should_start_from_delimiter() {
        let input = "text - text";
        let expected = Yamd::new(
            None,
            vec![Paragraph::new(vec![Text::new("text - text").into()]).into()],
        );
        let actual = Yamd::parse(input, 0, None).unwrap();
        assert_eq!(expected, actual.0);
    }

    #[test]
    fn last_delimiter() {
        let input = "text\n\n";
        let expected = Yamd::new(
            None,
            vec![
                Paragraph::new(vec![Text::new("text").into()]).into(),
                Paragraph::new(vec![]).into(),
            ],
        );
        let actual = Yamd::parse(input, 0, None).unwrap();
        assert_eq!(expected, actual.0);
    }
}
