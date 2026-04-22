//! YAMD - Yet Another Markdown Document (flavour)
//!
//! Simplified version of [CommonMark](https://spec.commonmark.org/).
//!
//! For formatting check [`YAMD`](nodes::Yamd) struct documentation.
//!
//! # Quick start
//!
//! ```rust
//! use yamd::deserialize;
//!
//! let input = "# Hello\n\nA paragraph with **bold** text.";
//! let yamd = deserialize(input);
//!
//! // Access the AST
//! assert_eq!(yamd.body.len(), 2);
//!
//! // Round-trip back to markdown
//! assert_eq!(yamd.to_string(), input);
//! ```
//!
//! # Two APIs
//!
//! - [`deserialize`] returns a nested [`Yamd`](nodes::Yamd) document — a tree of typed nodes,
//!   suitable for walking, pattern-matching, or round-tripping back to markdown via
//!   [`Display`](std::fmt::Display). The AST makes invalid nestings unrepresentable, and
//!   `deserialize` is fuzz-tested for panic-freedom and property-tested for round-trip fidelity.
//! - [`parse`] returns a flat `Vec<`[`Op`](op::Op)`>` of Start/End/Value events, where
//!   [`Content`](op::Content) borrows from the source when possible. Reach for it when you want
//!   streaming rendering or zero-copy text processing without materializing the full tree.
//!   [`to_yamd`] promotes an event stream to the tree form. Fuzz-tested for panic-freedom
//!   (transitively, via `deserialize`); the AST's type-level invariants and round-trip property
//!   do not apply at this layer.
//!
//! # Reasoning
//!
//! YAMD exchanges CommonMark's context-dependent rules for a uniform set: every node is treated
//! the same, and escaping is resolved at the lexer. The goal is a parser that's easier to reason
//! about locally, with fewer special cases to remember.
//!
//! Rendering is out of scope; [`Yamd`](nodes::Yamd) is an AST you walk and render however you
//! like. With the `serde` feature enabled, the AST is also serde-serializable.
//!
//! # Difference from CommonMark
//!
//! YAMD reuses most of CommonMark's syntax but diverges in a few places.
//!
//! ## Escaping
//!
//! Escaping is handled at the [lexer] level: any character following `\` is treated as a
//! [literal](lexer::TokenKind::Literal).
//!
//! Example:
//!
//! | YAMD      | HTML equivalent |
//! |-----------|-----------------|
//! | `\**foo**`|`<p>**foo**</p>` |
//!
//! ## Precedence
//!
//! [CommonMark](https://spec.commonmark.org/0.31.2/#precedence) distinguishes container blocks from
//! leaf blocks and gives container-block markers higher precedence. YAMD does not distinguish block
//! types — every node is treated the same, so there are no precedence rules to remember.
//!
//! Example:
//!
//! | YAMD                  | HTML equivalent                               |
//! |-----------------------|-----------------------------------------------|
//! | ``- `one\n- two` ``   | `<ol><li><code>one\n- two</code></li></ol>`   |
//!
//!
//! To get two separate [ListItem](nodes::ListItem)s, escape the backticks:
//!
//! | YAMD                      | HTML equivalent                           |
//! |---------------------------|-------------------------------------------|
//! | ``- \`one\n- two\` ``     | ``<ol><li>`one</li><li>two`</li><ol>``    |
//!
//! The reasoning: issues like this should be caught by tooling such as linters or language servers
//! — that tooling doesn't exist yet.
//!
//! ## Nodes
//!
//! See [nodes] for the full list of supported nodes and their formatting. Start with [YAMD](nodes::Yamd).
//!
//! # MSRV
//!
//! YAMD minimal supported Rust version is 1.87.

#[deny(missing_docs, rustdoc::broken_intra_doc_links)]
pub mod lexer;
pub mod nodes;
pub mod op;

#[doc(inline)]
pub use nodes::Yamd;
pub use op::parse;
pub use op::to_yamd;

/// Deserialize a string into a Yamd struct
/// # Example
/// ```
/// use yamd::deserialize;
/// let input = "# header";
/// let yamd = deserialize(input);
/// ```
pub fn deserialize(input: &str) -> Yamd {
    let ops = op::parse(input);
    op::to_yamd(&ops, input)
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
