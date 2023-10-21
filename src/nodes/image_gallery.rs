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
    #[serde(skip_serializing)]
    consumed_all_input: bool,
}

impl ImageGallery {
    pub fn new(consumed_all_input: bool) -> Self {
        Self::new_with_nodes(vec![], consumed_all_input)
    }

    pub fn new_with_nodes(nodes: Vec<ImageGalleryNodes>, consumed_all_input: bool) -> Self {
        Self {
            nodes,
            consumed_all_input,
        }
    }
}

impl Display for ImageGallery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        write!(
            f,
            "!!!\n{}!!!{end}",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Node for ImageGallery {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl Deserializer for ImageGallery {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(image_gallery) = matcher.get_match("!!!\n", "!!!", false) {
            return Self::parse_branch(
                image_gallery.body,
                Self::new(matcher.get_match("\n\n", "", false).is_none()),
            );
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
        if self.consumed_all_input {
            7
        } else {
            9
        }
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
            ImageGallery::new_with_nodes(
                vec![
                    Image::new(true, "a", "u").into(),
                    Image::new(true, "a2", "u2").into()
                ],
                true
            )
            .to_string(),
            "!!!\n![a](u)\n![a2](u2)\n!!!"
        );
        assert_eq!(
            ImageGallery::new_with_nodes(
                vec![
                    Image::new(true, "a", "u").into(),
                    Image::new(true, "a2", "u2").into()
                ],
                false
            )
            .to_string(),
            "!!!\n![a](u)\n![a2](u2)\n!!!\n\n"
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ImageGallery::new_with_nodes(
                vec![
                    Image::new(true, "a", "u").into(),
                    Image::new(true, "a2", "u2").into()
                ],
                true
            )
            .len(),
            25
        );
        assert_eq!(
            ImageGallery::new_with_nodes(
                vec![
                    Image::new(true, "a", "u").into(),
                    Image::new(true, "a2", "u2").into()
                ],
                false
            )
            .len(),
            27
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            ImageGallery::deserialize("!!!\n![a](u)\n![a2](u2)\n!!!"),
            Some(ImageGallery::new_with_nodes(
                vec![
                    Image::new(true, "a", "u").into(),
                    Image::new(true, "a2", "u2").into()
                ],
                true
            ))
        );
        assert_eq!(
            ImageGallery::deserialize("!!!\n![a](u)\n![a2](u2)\n!!!\n\n"),
            Some(ImageGallery::new_with_nodes(
                vec![
                    Image::new(true, "a", "u").into(),
                    Image::new(true, "a2", "u2").into()
                ],
                false
            ))
        );
    }
}
