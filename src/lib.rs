//! # Yamd - yet another markdown document flavour
//!
//! Yamd is a markdown document flavour that allows to create rich documents with images, code, and more.
//!
//! ## Syntax
//!
//! Each yamd document starts with metadata section  which is YAML document surrounded by "---". Metadata has next
//! fields: title, date, image, preview, is_draft, and tags.
//!
//! Timestamp format: "%Y-%m-%dT%H:%M:%S%z" ([specifiers description](https://docs.rs/chrono/latest/chrono/format/strftime/index.html))
//!
//! Tags are array of strings.
//!
//! is_draft is a boolean value.
//!
//! Example:
//! ```text
//! ---
//! title: Yamd - yet another markdown document flavour
//! date: 2023-01-01 00:00:00 +0000
//! image: /image.png
//! preview: Here you can find out more about yamd
//! is_draft: true
//! tags:
//! - markdown
//! - rust
//! ---
//!
//! ```
//!
//! ## Elements:
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
//! Element that starts with ">>>\n", followed by optional title that starts with ">> " and ends with a new line,
//! followed by optional icon specifier that starts with "> " and ends with a new line, followed by body that can
//! contain any number of paragraph elements
//!
//! Example:
//! ```text
//! >>>
//! >> Title
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
//! ### Collapsible
//!
//! Collapsible element can contain all from the above
//!
//! Example:
//!
//! ```text
//! {% Title
//! some random text
//! %}
//! ```
//!
//! Nested collapsible elements example:
//!
//! ```text
//! {% Title
//! some random text
//!
//! {% Nested title
//! some random text
//! %}
//! %}
//! ```
//!
//! ### Heading
//!
//! Element that starts with one to seven "#" characters followed by space, followed by text and/or link, and ends
//! with a new line or EOF
//!
//! Example: ```# header``` or ```###### header``` or ```# [header](url)``` or ```# header [link](url)```
//!

use nodes::yamd::Yamd;
use toolkit::parser::Parse;

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
    Yamd::parse(input, 0, None).map(|(yamd, _)| yamd)
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
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::{anchor::Anchor, heading::Heading, paragraph::Paragraph, text::Text};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize() {
        let input = "# header";
        let expected = Yamd::new(
            None,
            vec![Heading::new(1, vec![Text::new("header").into()]).into()],
        );
        let actual = deserialize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_serialize() {
        let input = Yamd::new(
            None,
            vec![Heading::new(1, vec![Text::new("header").into()]).into()],
        );
        let expected = "# header";
        let actual = serialize(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_text_containing_utf8() {
        let input = "## 🤔\n\n[link 😉](url)";
        let expected = Yamd::new(
            None,
            vec![
                Heading::new(2, vec![Text::new("🤔").into()]).into(),
                Paragraph::new(vec![Anchor::new("link 😉", "url").into()]).into(),
            ],
        );
        let actual = deserialize(input).unwrap();
        assert_eq!(expected, actual);
    }
}
