use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    matcher::Matcher,
    node::Node,
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

impl Node for ImageGalleryNodes {
    fn len(&self) -> usize {
        match self {
            ImageGalleryNodes::Image(node) => node.len(),
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

impl Node for ImageGallery {
    fn len(&self) -> usize {
        let delimiter_len = if self.nodes.is_empty() {
            0
        } else {
            self.nodes.len() - 1
        };
        self.nodes.iter().map(|node| node.len()).sum::<usize>()
            + delimiter_len
            + self.get_outer_token_length()
    }
}

impl Deserializer for ImageGallery {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(image_gallery) = matcher.get_match("!!!\n", "\n!!!", false) {
            return Self::parse_branch(image_gallery.body, "\n", Self::default());
        }
        None
    }
}

impl Branch<ImageGalleryNodes> for ImageGallery {
    fn push<CanBeNode: Into<ImageGalleryNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into())
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ImageGalleryNodes>> {
        vec![Image::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ImageGalleryNodes>> {
        None
    }

    fn get_outer_token_length(&self) -> usize {
        8
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::ImageGallery;
    use crate::{
        nodes::image::Image,
        toolkit::{deserializer::Deserializer, node::Node},
    };
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
    fn len() {
        assert_eq!(
            ImageGallery::new(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ],)
            .len(),
            25
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            ImageGallery::deserialize("!!!\n![a](u)\n![a2](u2)\n!!!"),
            Some(ImageGallery::new(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ],))
        );
        assert_eq!(
            ImageGallery::deserialize("!!!\n![a](u)\n![a2](u2)\n!!!\n\n"),
            Some(ImageGallery::new(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ],))
        );
    }
}
