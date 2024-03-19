use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    parser::{parse_to_parser, Branch, Consumer, Parse, Parser},
};

use super::image::Image;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ImageGalleryNodes {
    Image(Image),
}

impl Display for ImageGalleryNodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageGalleryNodes::Image(node) => write!(f, "{}", node),
        }
    }
}

impl From<Image> for ImageGalleryNodes {
    fn from(value: Image) -> Self {
        ImageGalleryNodes::Image(value)
    }
}

/// Image Gallery node is a node that contains multiple Image nodes
/// it starts with `!!!\n` and ends with `\n!!!`
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct ImageGallery {
    pub nodes: Vec<ImageGalleryNodes>,
}

impl ImageGallery {
    pub fn new(nodes: Vec<ImageGalleryNodes>) -> Self {
        Self { nodes }
    }
}

impl Default for ImageGallery {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Display for ImageGallery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "!!!\n{}\n!!!",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Branch<ImageGalleryNodes> for ImageGallery {
    fn get_parsers(&self) -> Vec<Parser<ImageGalleryNodes>> {
        vec![parse_to_parser::<ImageGalleryNodes, Image>()]
    }

    fn get_consumer(&self) -> Option<Consumer<ImageGalleryNodes>> {
        None
    }

    fn push_node(&mut self, node: ImageGalleryNodes) {
        self.nodes.push(node);
    }
}

impl Parse for ImageGallery {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if input[current_position..].starts_with("!!!\n") {
            if let Some(end) = input[current_position + 4..].find("\n!!!") {
                let gallery = ImageGallery::new(vec![]);
                if let Some(node) =
                    gallery.parse_branch(&input[current_position + 4..end], "\n", None)
                {
                    return Some((node, end + 4 - current_position));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::ImageGallery;
    use crate::{nodes::image::Image, toolkit::parser::Parse};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize() {
        assert_eq!(
            ImageGallery::new(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ],)
            .to_string(),
            "!!!\n![a](u)\n![a2](u2)\n!!!"
        );
        assert_eq!(
            ImageGallery::new(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ],)
            .to_string(),
            "!!!\n![a](u)\n![a2](u2)\n!!!"
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            ImageGallery::parse("!!!\n![a](u)\n![a2](u2)\n!!!", 0, None),
            Some((
                ImageGallery::new(vec![
                    Image::new("a", "u").into(),
                    Image::new("a2", "u2").into()
                ]),
                20
            ))
        );
    }
}
