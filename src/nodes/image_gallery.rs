use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    node::Node,
    tokenizer::{
        Pattern::{Once, RepeatTimes},
        Tokenizer,
    },
};

use super::image::Image;

#[derive(Debug, PartialEq)]
pub enum ImageGaleryNodes {
    Image(Image),
}

impl Node for ImageGaleryNodes {
    fn len(&self) -> usize {
        match self {
            ImageGaleryNodes::Image(node) => node.len(),
        }
    }
    fn serialize(&self) -> String {
        match self {
            ImageGaleryNodes::Image(node) => node.serialize(),
        }
    }
}

impl From<Image> for ImageGaleryNodes {
    fn from(value: Image) -> Self {
        ImageGaleryNodes::Image(value)
    }
}

/// Image Gallery node is a node that contains multiple Image nodes
/// it starts with `!!!\n` and ends with `\n!!!`
#[derive(Debug, PartialEq)]
pub struct ImageGalery {
    nodes: Vec<ImageGaleryNodes>,
}

impl ImageGalery {
    pub fn new() -> Self {
        Self::new_with_nodes(vec![])
    }

    pub fn new_with_nodes(nodes: Vec<ImageGaleryNodes>) -> Self {
        Self { nodes }
    }
}

impl Default for ImageGalery {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for ImageGalery {
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
}

impl Deserializer for ImageGalery {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_node_body(
            &[RepeatTimes(3, '!'), Once('\n')],
            &[Once('\n'), Once('!'), Once('!'), Once('!')],
        ) {
            return Self::parse_branch(body, Self::new());
        }
        None
    }
}

impl Branch<ImageGaleryNodes> for ImageGalery {
    fn push<CanBeNode: Into<ImageGaleryNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into())
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ImageGaleryNodes>> {
        vec![Image::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ImageGaleryNodes>> {
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

    use super::ImageGalery;

    #[test]
    fn serialize() {
        assert_eq!(
            ImageGalery::new_with_nodes(vec![
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
            ImageGalery::new_with_nodes(vec![
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
            ImageGalery::deserialize("!!!\n![a](u)\n![a2](u2)\n!!!"),
            Some(ImageGalery::new_with_nodes(vec![
                Image::new("a", "u").into(),
                Image::new("a2", "u2").into()
            ]))
        );
    }

    #[test]
    fn default() {
        assert_eq!(ImageGalery::default().len(), 8)
    }
}
