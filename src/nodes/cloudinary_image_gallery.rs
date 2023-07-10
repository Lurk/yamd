use crate::toolkit::{
    context::Context, deserializer::Deserializer, matcher::Matcher, node::Node,
    pattern::Quantifiers::*,
};

#[derive(Debug, PartialEq, Clone)]
pub struct CloudinaryImageGallery {
    username: String,
    tag: String,
    consumed_all_input: bool,
}

impl CloudinaryImageGallery {
    pub fn new<S: Into<String>>(username: S, tag: S, consumed_all_input: bool) -> Self {
        Self {
            username: username.into(),
            tag: tag.into(),
            consumed_all_input,
        }
    }
}

impl Node for CloudinaryImageGallery {
    fn serialize(&self) -> String {
        format!(
            "!!!!\n! {username}\n! {tag}\n!!!!{end}",
            username = self.username,
            tag = self.tag,
            end = if self.consumed_all_input { "" } else { "\n\n" }
        )
    }

    fn len(&self) -> usize {
        self.username.len() + self.tag.len() + 15 + if self.consumed_all_input { 0 } else { 2 }
    }
}

impl Deserializer for CloudinaryImageGallery {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(cloudinary_image_gallery) = matcher.get_match(
            &[RepeatTimes(4, '!'), Once('\n')],
            &[Once('\n'), RepeatTimes(4, '!')],
            false,
        ) {
            let mut inner_matcher = Matcher::new(cloudinary_image_gallery.body);
            if let Some(username) =
                inner_matcher.get_match(&[Once('!'), Once(' ')], &[Once('\n')], false)
            {
                if let Some(tag) =
                    inner_matcher.get_match(&[Once('!'), Once(' ')], &[Once('\n')], true)
                {
                    let consumed_all_input = matcher
                        .get_match(&[RepeatTimes(2, '\n')], &[], false)
                        .is_none();
                    return Some(Self::new(username.body, tag.body, consumed_all_input));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{
        nodes::cloudinary_image_gallery::CloudinaryImageGallery,
        toolkit::{deserializer::Deserializer, node::Node},
    };

    #[test]
    fn test_cloudinary_image_gallery() {
        let input = "!!!!\n! username\n! tag\n!!!!\n\n";
        let expected = CloudinaryImageGallery::new("username", "tag", false);
        assert_eq!(
            CloudinaryImageGallery::deserialize(input),
            Some(expected.clone()),
        );
        assert_eq!(expected.serialize(), input);
    }

    #[test]
    fn cloudinary_image_gallery_len() {
        let input = "!!!!\n! username\n! tag\n!!!!\n\n";
        assert_eq!(
            CloudinaryImageGallery::deserialize(input).unwrap().len(),
            input.len(),
        );
    }
}
