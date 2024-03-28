use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    parser::{parse_to_consumer, parse_to_parser, Branch, Consumer, Parse, Parser},
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

impl Branch<CollapsibleNodes> for Collapsible {
    fn get_parsers(&self) -> Vec<Parser<CollapsibleNodes>> {
        vec![
            parse_to_parser::<CollapsibleNodes, Heading>(),
            parse_to_parser::<CollapsibleNodes, Image>(),
            parse_to_parser::<CollapsibleNodes, ImageGallery>(),
            parse_to_parser::<CollapsibleNodes, List>(),
            parse_to_parser::<CollapsibleNodes, Embed>(),
            parse_to_parser::<CollapsibleNodes, Divider>(),
            parse_to_parser::<CollapsibleNodes, Code>(),
            parse_to_parser::<CollapsibleNodes, Collapsible>(),
        ]
    }

    fn get_consumer(&self) -> Option<Consumer<CollapsibleNodes>> {
        Some(parse_to_consumer::<CollapsibleNodes, Paragraph>())
    }

    fn push_node(&mut self, node: CollapsibleNodes) {
        self.nodes.push(node);
    }
}

impl Parse for Collapsible {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        if input[current_position..].starts_with("{% ") {
            let start = current_position + 3;
            if let Some(end_of_title) = input[start..].find('\n') {
                let title = &input[start..start + end_of_title];
                let mut level = 1;
                for (index, _) in input[start + end_of_title..].char_indices() {
                    if input[index + start + end_of_title + 1..].starts_with("{% ") {
                        level += 1;
                    } else if input[index + start + end_of_title + 1..].starts_with("\n%}") {
                        level -= 1;
                    }
                    if level == 0 {
                        let colapsible = Collapsible::new(title, vec![]);

                        return Some((
                            colapsible
                                .parse_branch(
                                    &input[start + end_of_title + 1
                                        ..start + end_of_title + 1 + index],
                                    "\n\n",
                                    None,
                                )
                                .expect("collapsible branch should always succeed"),
                            3 + end_of_title + 1 + index + 3,
                        ));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        nodes::{
            bold::Bold,
            code::Code,
            collapsible::Collapsible,
            divider::Divider,
            embed::Embed,
            heading::Heading,
            image::Image,
            image_gallery::ImageGallery,
            list::{List, ListTypes::*},
            list_item::ListItem,
            paragraph::Paragraph,
            text::Text,
        },
        toolkit::parser::Parse,
    };

    #[test]
    fn test_collapsible_parse() {
        assert_eq!(
            Collapsible::parse("{% Title\n# Heading\n%}", 0, None),
            Some((
                Collapsible::new(
                    "Title",
                    vec![Heading::new(1, vec![Text::new("Heading").into()]).into()]
                ),
                21
            ))
        );
    }

    #[test]
    fn test_collapsible_serialize() {
        assert_eq!(
            Collapsible::new(
                "Title",
                vec![Heading::new(1, vec![Text::new("Heading").into()]).into()]
            )
            .to_string(),
            "{% Title\n# Heading\n%}"
        );
    }

    #[test]
    fn fail_to_parse_collapsible() {
        assert_eq!(
            Collapsible::parse("I am not an accordion tab", 0, None),
            None
        );
        assert_eq!(Collapsible::parse("{% \n%}", 0, None), None);
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
                Heading::new(1, vec![Text::new("hello").into()]).into(),
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
                                None,
                            )
                            .into()],
                        )),
                    )
                    .into()],
                )
                .into(),
                Embed::new("youtube", "123").into(),
                Embed::new("cloudinary_gallery", "cloud_name&tag").into(),
                Collapsible::new(
                    "nested collapsible",
                    vec![Heading::new(1, vec![Text::new("nested").into()]).into()],
                )
                .into(),
            ],
        );
        assert_eq!(tab.to_string(), input);
        assert_eq!(Collapsible::parse(input, 0, None), Some((tab, input.len())));
    }

    #[test]
    fn parse_empty() {
        let input = "{% Title\n\n%}";
        assert_eq!(
            Collapsible::parse(input, 0, None),
            Some((Collapsible::new("Title", vec![]), input.len()))
        );
    }
}
