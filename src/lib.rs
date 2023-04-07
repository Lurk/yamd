//! # Yamd - yet another markdown flavour
//!
//! ## Elements:
//!
//! ### Heading
//!
//! element that starts with one to seven "#" characters followed by space, followed by text, and ends with a new line or EOF
//!
//! Example: ```# header``` or ```###### header```
//!
//! ### List
//!
//! Can contain nested lists. Each nesting level equals to number of spaces before list item.
//!
//! #### Unordered list
//!
//! Starts with "- " and ends with a new line
//!
//! #### Ordered list
//!
//! Starts with "+ " and ends with a new line
//!
//! Example:
//! ```text
//! - item 1
//! - item 2
//!  + ordered nested list
//!   - can have nested unordered list inside ordered list
//!  + one more ordered item
//! - item 3
//! ```
//!
//! ### Code
//!
//! Element that starts with "\`\`\`lang\n", ends with "\n```" and has code in between.
//!
//! Example:
//! ```text
//! \```rust
//! let x = 1;
//! \```
//! ```
//! ^ did not figured out how to escape \`\`\` in rustdoc
//!
//! ### Image
//!
//! Element that starts with "!" has image alt text in [] and followed by image url in ()
//!
//! Example:
//! ```text
//! ![alt text](url)
//! ```
//!
//! ### ImageGallery
//!
//! Element that starts with "!!!\n", ends with "\n!!!", and has image elements in between
//!
//! Example:
//! ```text
//! !!!
//! ![alt text](url)
//! ![alt text](url)
//! !!!
//! ```
//!
//! ### Highlight
//!
//! Element that starts with ">>>\n", followed by optional header that starts with ">> " and ends with a new line,
//! followed by optional icon specifier that starts with "> " and ends with a new line, followed by body that can
//! contain any number of paragraph elements
//!
//! Example:
//! ```text
//! >>>
//! >> Header
//! > icon
//! body
//!
//! can be multiple paragraphs long
//! >>>
//! ```
//! no header and no icon:
//! ```text
//! >>>
//! body
//! >>>
//! ```
//!
//! ### Divider
//!
//! Element that consist of five "-" characters in a row and ends with a new line or EOF.
//!
//! Example: ```-----```
//!
//! ### Embed
//!
//! Element that starts with "{{" followed by embed type, followed by "|" followed by embed url, followed by "}}"
//! and ends with a new line or EOF.
//!
//! Example: ```{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}```
//!
//!
//! ### Paragraph
//!
//! Element that starts with any character that is not a special character and ends with a new line or EOF.
//! Can contain text, bold text, italic text, strikethrough text, anchors, and inline code.
//!
//! #### Anchor
//!
//! element that starts with "[" followed by text, followed by "]" followed by "(" followed by url, followed by ")"
//!
//! example: ```[Yamd repo](https://github.com/Lurk/yamd)```
//!
//! #### Inline code
//!
//! element that starts with "\`" followed by text and ends with "\`"
//!
//! example: ``` `inline code` ```
//!
//! #### Italic text    
//!
//! element that starts with "\_" followed by text and ends with "\_"
//!
//! example: ``` _italic text_ ```
//!
//! #### Strikethrough text
//!
//! element that starts with "\~\~" followed by text and ends with "\~\~"
//!
//! example: ``` ~~strikethrough text~~ ```
//!
//! #### Bold text
//!
//! element that starts with "\*\*" followed by text and ends with "\*\*"
//!
//! example: ``` **bold text** ```
//!
//! Bold text can also contain italic text and strikethrough text
//!
//! example: ``` **bold _italic_ text** ``` or ``` **bold ~~strikethrough~~ text** ```
//!
//! Altogether: ``` text **bold _italic_ text** ~~strikethrough~~ text `inline code` [Yamd repo](url) ``` will be parsed into Paragraph
//!
use nodes::yamd::Yamd;
use toolkit::{deserializer::Deserializer, node::Node};

pub mod nodes;
mod toolkit;

/// Deserialize a string into a Yamd struct
/// # Example
/// ```
/// use yamd::deserialize;
/// let input = "# header";
/// let yamd = deserialize(input).unwrap();
/// ```
pub fn deserialize(input: &str) -> Option<Yamd> {
    Yamd::deserialize(input)
}

/// Serialize a Yamd struct into a string
/// # Example
/// ```
/// use yamd::{deserialize, serialize};
/// let input = "# header";
/// let yamd = deserialize(input).unwrap();
/// let output = serialize(&yamd);
/// ```
pub fn serialize(input: &Yamd) -> String {
    input.serialize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::heading::Heading;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize() {
        let input = "# header";
        let expected = Yamd::new_with_nodes(vec![Heading::new("header", 1, true).into()]);
        let actual = deserialize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_serialize() {
        let input = Yamd::new_with_nodes(vec![Heading::new("header", 1, true).into()]);
        let expected = "# header";
        let actual = serialize(&input);
        assert_eq!(expected, actual);
    }
}
