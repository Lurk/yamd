use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Image;

/// # Images
///
/// One or more [Image]'s separated by [EOL](type@crate::lexer::TokenKind::Eol). There is
/// no 1:1 match for that in HTML.
///
/// Example:
///
/// ```text
/// ![alt](src)
/// ![alt](src)
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <div class="images">
///     <img src="url" alt="alt"/>
///     <img src="url" alt="alt"/>
/// </div>
/// ```

#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Images {
    pub body: Vec<Image>,
}

impl Images {
    pub fn new(body: Vec<Image>) -> Self {
        Self { body }
    }
}

impl Display for Images {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|image| image.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Image;

    #[test]
    fn images() {
        let images = Images::new(vec![Image::new("alt", "src"), Image::new("alt", "src")]);
        assert_eq!(images.to_string(), "![alt](src)\n![alt](src)");
    }
}
