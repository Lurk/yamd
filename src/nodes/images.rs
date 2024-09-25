use serde::Serialize;

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

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Images {
    pub body: Vec<Image>,
}

impl Images {
    pub fn new(body: Vec<Image>) -> Self {
        Self { body }
    }
}

impl Default for Images {
    fn default() -> Self {
        Self::new(vec![])
    }
}
