//! YAMD - Yet Another Markdown Document (flavour)
//!
//! Simplified version of [CommonMark](https://spec.commonmark.org/).
//!
//! For formatting check [YAMD](crate::nodes::Yamd) struct documentation.
//!
//! # Reasoning
//!
//! Simplified set of rules allows to have simpler, more efficient, parser and renderer.
//! [YAMD](crate::nodes::Yamd) does not provide render functionality, instead it is a [serde]
//! serializable structure that allows you to write any renderer for that structure. All HTML
//! equivalents in this doc are provided as an example to what it can be rendered.
//!
//! # Difference from CommonMark
//!
//! While YAMD tries to utilize as much CommonMark syntax as possible, there are differences.
//!
//! ## Escaping
//!
//! Escaping done on a [lexer] level. Every symbol following the `\` symbol will be treated as a
//! [literal](crate::lexer::TokenKind::Literal).
//!
//! Example:
//!
//! | YAMD      | HTML equivalent |
//! |-----------|-----------------|
//! | `\**foo**`|`<p>**foo**</p>` |
//!
//! ## Precedence
//!
//! [CommonMark](https://spec.commonmark.org/0.31.2/#precedence) defines container blocks and leaf
//! blocks. And that container block indicator has higher precedence. YAMD does not discriminate by
//! block type, every node (block) is the same. In practice, there are no additional rules to encode
//! and remember.
//!
//! Example:
//!
//! | YAMD                  | HTML equivalent                               |
//! |-----------------------|-----------------------------------------------|
//! | ``- `one\n- two` ``   | `<ol><li><code>one\n- two</code></li></ol>`   |
//!
//!
//! If you want to have two [ListItem](crate::nodes::ListItem)'s use escaping:
//!
//! | YAMD                      | HTML equivalent                           |
//! |---------------------------|-------------------------------------------|
//! | ``- \`one\n- two\` ``     | ``<ol><li>`one</li><li>two`</li><ol>``    |
//!
//! The reasoning is that those kind issues can be caught with tooling like linters/lsp. That tooling
//! does not exist yet.
//!
//! ## Nodes
//!
//! List of supported [nodes] and their formatting. The best starting point is [YAMD](crate::nodes::Yamd).
//!
//! # MSRV
//!
//! YAMD minimal supported Rust version is 1.80.0 due to [Option::take_if] usage
//!
pub mod lexer;
pub mod nodes;
mod parser;

#[doc(inline)]
pub use nodes::Yamd;
use parser::{yamd, Parser};

/// Deserialize a string into a Yamd struct
/// # Example
/// ```
/// use yamd::deserialize;
/// let input = "# header";
/// let yamd = deserialize(input);
/// ```
pub fn deserialize(str: &str) -> Yamd {
    let mut p = Parser::new(str);
    yamd(&mut p, |_| false)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        deserialize,
        nodes::{Anchor, Heading, Paragraph, Yamd},
    };

    #[test]
    fn test_deserialize() {
        let input = "# header";
        let expected = Yamd::new(
            None,
            vec![Heading::new(1, vec![String::from("header").into()]).into()],
        );
        let actual = deserialize(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_text_containing_utf8() {
        let input = "## 🤔\n\n[link 😉](url)";
        let expected = Yamd::new(
            None,
            vec![
                Heading::new(2, vec![String::from("🤔").into()]).into(),
                Paragraph::new(vec![Anchor::new("link 😉", "url").into()]).into(),
            ],
        );
        let actual = deserialize(input);
        assert_eq!(expected, actual);
    }
}
