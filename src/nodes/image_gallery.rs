use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    node::Node,
    tokenizer::{
        Matcher,
        Quantifiers::{Once, RepeatTimes},
    },
};

use super::image::Image;

#[derive(Debug, PartialEq)]
pub enum ImageGalleryNodes {
    Image(Image),
}

impl Node for ImageGalleryNodes {
    fn serialize(&self) -> String {
        match self {
            ImageGalleryNodes::Image(node) => node.serialize(),
        }
    }
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
#[derive(Debug, PartialEq)]
pub struct ImageGallery {
    nodes: Vec<ImageGalleryNodes>,
}

impl ImageGallery {
    pub fn new() -> Self {
        Self::new_with_nodes(vec![])
    }

    pub fn new_with_nodes(nodes: Vec<ImageGalleryNodes>) -> Self {
        Self { nodes }
    }
}

impl Default for ImageGallery {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for ImageGallery {
    fn serialize(&self) -> String {
        format!(
            "!!!\n{}\n!!!",
            self.nodes
                .iter()
                .map(|node| node.serialize())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
    fn len(&self) -> usize {
        let spacing_len = if self.nodes.is_empty() {
            0
        } else {
            self.nodes.len() - 1
        };

        self.nodes.iter().map(|node| node.len()).sum::<usize>()
            + spacing_len
            + self.get_outer_token_length()
    }
}

impl Deserializer for ImageGallery {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(image_gallery) = matcher.get_match(
            &[RepeatTimes(3, '!'), Once('\n')],
            &[Once('\n'), Once('!'), Once('!'), Once('!')],
            false,
        ) {
            return Self::parse_branch(image_gallery.body, Self::new());
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
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::image::Image,
        toolkit::{deserializer::Deserializer, node::Node},
    };

    use super::ImageGallery;

    #[test]
    fn serialize() {
        assert_eq!(
            ImageGallery::new_with_nodes(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ])
            .serialize(),
            "!!!\n![a](u)\n![a2](u2)\n!!!"
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ImageGallery::new_with_nodes(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ])
            .len(),
            25
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            ImageGallery::deserialize("!!!\n![a](u)\n![a2](u2)\n!!!"),
            Some(ImageGallery::new_with_nodes(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ]))
        );
    }

    #[test]
    fn default() {
        assert_eq!(ImageGallery::default().len(), 8)
    }
}
