//! A flat, streaming alternative to [`deserialize`](crate::deserialize): [`parse`] turns source
//! text into a `Vec<`[`Op`]`>` of Start/End/Value events instead of a nested tree.
//!
//! - [`Op`] pairs an [`OpKind`] with its [`Content`].
//! - [`OpKind::Start`]`(`[`Node`]`)` / [`OpKind::End`]`(`[`Node`]`)` bracket a node; everything
//!   between belongs to it. [`OpKind::Value`] is leaf content belonging to the innermost open
//!   node.
//! - [`Content`] is a byte range into the source plus a count of the `\`-escapes it contains.
//!
//! [`to_yamd`] consumes an event stream and promotes it to the [`Yamd`](crate::nodes::Yamd) tree
//! form used by [`deserialize`](crate::deserialize).

use std::borrow::Cow;
use std::ops::Range;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::lexer::Token;
pub(crate) use crate::op::parser::Parser;

mod anchor;
mod bold;
mod code;
mod code_span;
mod collapsible;
mod destination;
mod document;
mod embed;
mod emphasis;
mod heading;
mod highlight;
mod image;
mod images;
mod italic;
mod list;
mod metadata;
mod modifier;
mod paragraph;
mod parser;
mod strikethrough;
mod thematic_break;
mod title;
mod to_yamd;
pub use to_yamd::{UnbalancedOpStream, to_yamd, try_to_yamd};

/// Text content extracted from the source input: a byte range plus how many `\`-escapes it
/// contains.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Content {
    span: Range<usize>,
    escaped: usize,
}

impl Content {
    /// General constructor
    pub(crate) fn new(span: Range<usize>, escaped: usize) -> Self {
        Self { span, escaped }
    }

    /// Constructor for an unescaped span (the common case).
    pub fn span(range: Range<usize>) -> Self {
        Content::new(range, 0)
    }

    /// Constructor for empty case
    pub fn empty() -> Self {
        Content::new(0..0, 0)
    }

    /// Returns the text this content represents. Borrows directly from `source` when there's
    /// nothing to unescape; allocates and strips `\` otherwise.
    pub fn as_str<'a>(&'a self, source: &'a str) -> Cow<'a, str> {
        let raw = if self.span.is_empty() {
            ""
        } else {
            &source[self.span.clone()]
        };
        if self.escaped == 0 {
            Cow::Borrowed(raw)
        } else {
            Cow::Owned(unescape(raw, self.escaped))
        }
    }

    /// Returns an owned copy of the text this content represents.
    pub fn to_string(&self, source: &str) -> String {
        self.as_str(source).into_owned()
    }

    /// Returns `true` if this content represents an empty string.
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Builds `Content` from a token slice.
    pub fn from_tokens(tokens: &[Token]) -> Self {
        if tokens.is_empty() {
            return Content::empty();
        }
        let start = tokens.first().unwrap().range.start;
        let end = tokens.last().unwrap().range.end;
        let escaped = tokens.iter().map(|t| t.escaped as usize).sum();
        Content::new(start..end, escaped)
    }
}

fn unescape(raw: &str, escaped_count: usize) -> String {
    let mut out = String::with_capacity(raw.len().saturating_sub(escaped_count));
    let mut rest = raw;
    for _ in 0..escaped_count {
        let Some(idx) = rest.find('\\') else {
            break;
        };
        out.push_str(&rest[..idx]);
        rest = &rest[idx + 1..];
        let Some(c) = rest.chars().next() else {
            out.push('\\');
            return out;
        };
        out.push(c);
        rest = &rest[c.len_utf8()..];
    }
    out.push_str(rest);
    out
}

/// Identifies which AST node type an [`Op`] refers to.
///
/// Used in [`OpKind::Start`] and [`OpKind::End`] to mark the boundaries of nested structures
/// in the flat operation stream.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Node {
    Anchor,
    Bold,
    Code,
    CodeSpan,
    Collapsible,
    Destination,
    Document,
    Embed,
    Emphasis,
    Heading,
    Highlight,
    Icon,
    Image,
    Images,
    Italic,
    ListItem,
    Modifier,
    Metadata,
    OrderedList,
    Paragraph,
    Strikethrough,
    ThematicBreak,
    Title,
    UnorderedList,
}

/// Describes the role of an [`Op`] in the operation stream.
///
/// The operation stream represents nested document structure as a flat sequence using
/// Start/End pairs with Value nodes for leaf content:
///
/// ```text
/// Start(Paragraph) -> Value("hello ") -> Start(Bold) -> Value("world") -> End(Bold) -> End(Paragraph)
/// ```
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum OpKind {
    /// Opens a new node. Everything until the matching [`End`](OpKind::End) is a child.
    Start(Node),
    /// Closes the most recently opened node of this type.
    End(Node),
    /// Leaf content belonging to the innermost open node.
    Value,
}

/// A single operation in the intermediate representation between lexer tokens and the final
/// [`Yamd`](crate::nodes::Yamd) AST.
///
/// The parser produces a `Vec<Op>` where Start/End pairs encode nesting and Value ops carry
/// text content. [`to_yamd`] converts this flat stream into the tree-shaped AST.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Op {
    pub kind: OpKind,
    pub content: Content,
}

impl Op {
    /// Creates a [`Value`](OpKind::Value) operation with the given content.
    pub fn new_value(content: Content) -> Self {
        Self {
            kind: OpKind::Value,
            content,
        }
    }

    /// Creates a [`Start`](OpKind::Start) operation for the given node type.
    pub fn new_start(node: Node, content: Content) -> Self {
        Self {
            kind: OpKind::Start(node),
            content,
        }
    }

    /// Creates an [`End`](OpKind::End) operation for the given node type.
    pub fn new_end(node: Node, content: Content) -> Self {
        Self {
            kind: OpKind::End(node),
            content,
        }
    }
}

/// Parses markdown source text into a flat operation stream.
///
/// This is the entry point of the op-based parser. It tokenizes the input via the
/// [`Lexer`](crate::lexer::Lexer), then runs the metadata and document parsers to produce
/// a sequence of [`Op`]s. Use [`to_yamd`] to convert the result into the final AST.
///
/// ```
/// let ops = yamd::parse("# hello\n\nworld");
/// assert!(!ops.is_empty());
/// ```
pub fn parse(input: &str) -> Vec<Op> {
    let mut parser = Parser::from(input);
    metadata::metadata(&mut parser);
    document::document(&mut parser);
    parser.into_ops()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::TokenKind;

    const TEST_CASE: &str = r#"---
title: test
date: 2022-01-01T00:00:00+02:00
image: image
preview: preview
tags:
- tag1
- tag2
---

# hello

```rust
let a=1;
```

t**b**

![a](u)

![a](u)
![a2](u2)

!! H
! I
~~s~~

_I_
!!

-----

- one
 - two

+ first
 + second

{{youtube|123}}

{{cloudinary_gallery|cloud_name&tag}}

{% collapsible

%}

{% one more collapsible

%}

+

-

![](

```

end

\*escaped\*"#;

    #[test]
    fn all_source_bytes_are_covered_exactly_once() {
        let ops = parse(TEST_CASE);
        let mut covered = vec![false; TEST_CASE.len()];

        for op in &ops {
            for i in op.content.span.clone() {
                assert!(
                    !covered[i],
                    "byte {i} covered by multiple ops (char: {:?})",
                    &TEST_CASE[i..i + 1]
                );
                covered[i] = true;
            }
        }
        let uncovered: Vec<usize> = covered
            .iter()
            .enumerate()
            .filter(|&(_, is_covered)| !is_covered)
            .map(|(i, _)| i)
            .collect();
        assert!(
            uncovered.is_empty(),
            "uncovered byte positions: {uncovered:?}"
        );
    }

    #[test]
    fn start_end_balance() {
        let ops = parse(TEST_CASE);
        let mut stack: Vec<&Node> = vec![];
        for op in &ops {
            match &op.kind {
                OpKind::Start(node) => stack.push(node),
                OpKind::End(node) => {
                    let top = stack.pop().expect("End without matching Start");
                    assert_eq!(top, node, "mismatched Start({top:?}) / End({node:?})");
                }
                OpKind::Value => {}
            }
        }
        assert!(stack.is_empty(), "unmatched Start nodes: {stack:?}");
    }

    #[test]
    fn document_level_block_sequence() {
        let ops = parse(TEST_CASE);
        assert_eq!(ops.len(), 158);
        assert_eq!(ops[5].kind, OpKind::Start(Node::Heading)); // # hello
        assert_eq!(ops[9].kind, OpKind::Start(Node::Code)); // ```rust ... ```
        assert_eq!(ops[16].kind, OpKind::Start(Node::Paragraph)); // t**b**
        assert_eq!(ops[23].kind, OpKind::Start(Node::Image)); // ![a](u)
        assert_eq!(ops[32].kind, OpKind::Start(Node::Images)); // ![a](u)\n![a2](u2)
        assert_eq!(ops[51].kind, OpKind::Start(Node::Highlight)); // !! H ... !!
        assert_eq!(ops[72].kind, OpKind::Start(Node::ThematicBreak)); // -----
        assert_eq!(ops[76].kind, OpKind::Start(Node::UnorderedList)); // - one\n - two
        assert_eq!(ops[91].kind, OpKind::Start(Node::OrderedList)); // + first\n + second
        assert_eq!(ops[106].kind, OpKind::Start(Node::Embed)); // {{youtube|123}}
        assert_eq!(ops[112].kind, OpKind::Start(Node::Embed)); // {{cloudinary_gallery|...}}
        assert_eq!(ops[118].kind, OpKind::Start(Node::Collapsible)); // {% collapsible ... %}
        assert_eq!(ops[126].kind, OpKind::Start(Node::Collapsible)); // {% one more collapsible ... %}
        assert_eq!(ops[134].kind, OpKind::Start(Node::Paragraph)); // + (fallback)
        assert_eq!(ops[138].kind, OpKind::Start(Node::Paragraph)); // - (fallback)
        assert_eq!(ops[142].kind, OpKind::Start(Node::Paragraph)); // ![]( (fallback)
        assert_eq!(ops[146].kind, OpKind::Start(Node::Paragraph)); // ``` (fallback)
        assert_eq!(ops[150].kind, OpKind::Start(Node::Paragraph)); // end (fallback)
    }

    #[test]
    fn metadata_is_parsed() {
        let ops = parse(TEST_CASE);

        assert_eq!(ops[0].kind, OpKind::Start(Node::Metadata));
        let metadata_value = ops[1].content.as_str(TEST_CASE);
        assert!(
            metadata_value.contains("title: test"),
            "metadata should contain 'title: test', got: {metadata_value}"
        );
        assert_eq!(ops[2].kind, OpKind::End(Node::Metadata));
    }

    #[test]
    fn document_wraps_body() {
        let ops = parse(TEST_CASE);

        assert_eq!(ops[3].kind, OpKind::Start(Node::Document));
        assert_eq!(ops.last().unwrap().kind, OpKind::End(Node::Document));
    }

    #[test]
    fn fallback_paragraphs_have_correct_content() {
        let ops = parse(TEST_CASE);

        // fallback paragraph: +
        assert_eq!(ops[134].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops[135].kind, OpKind::Value);
        assert_eq!(ops[135].content.as_str(TEST_CASE), "+");
        assert_eq!(ops[136].kind, OpKind::End(Node::Paragraph));

        // fallback paragraph: -
        assert_eq!(ops[138].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops[139].kind, OpKind::Value);
        assert_eq!(ops[139].content.as_str(TEST_CASE), "-");
        assert_eq!(ops[140].kind, OpKind::End(Node::Paragraph));

        // fallback paragraph: ![](
        assert_eq!(ops[142].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops[143].kind, OpKind::Value);
        assert_eq!(ops[143].content.as_str(TEST_CASE), "![](");
        assert_eq!(ops[144].kind, OpKind::End(Node::Paragraph));

        // fallback paragraph: ```
        assert_eq!(ops[146].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops[147].kind, OpKind::Value);
        assert_eq!(ops[147].content.as_str(TEST_CASE), "```");
        assert_eq!(ops[148].kind, OpKind::End(Node::Paragraph));

        // fallback paragraph: end
        assert_eq!(ops[150].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops[151].kind, OpKind::Value);
        assert_eq!(ops[151].content.as_str(TEST_CASE), "end");
        assert_eq!(ops[152].kind, OpKind::End(Node::Paragraph));
    }

    #[test]
    fn content_span_as_str() {
        let source = "hello world";
        let content = Content::span(0..5);
        assert_eq!(content.as_str(source), "hello");
    }

    #[test]
    fn content_escaped_as_str() {
        let source = "a\\!b";
        let content = Content::new(0..source.len(), 1);
        assert_eq!(content.as_str(source), "a!b");
    }

    #[test]
    fn content_span_to_string() {
        let source = "hello world";
        let content = Content::span(0..5);
        assert_eq!(content.to_string(source), "hello");
    }

    #[test]
    fn content_from_tokens_sums_escaped_across_slice() {
        let source = "a\\!b c\\!d";
        let tokens = vec![
            Token {
                kind: TokenKind::Literal,
                range: 0..4,
                is_line_start: true,
                escaped: 1,
            },
            Token::new(TokenKind::Space, 4..5, false),
            Token {
                kind: TokenKind::Literal,
                range: 5..9,
                is_line_start: false,
                escaped: 1,
            },
        ];
        let content = Content::from_tokens(&tokens);
        assert_eq!(content, Content::new(0..9, 2));
        assert_eq!(content.as_str(source), "a!b c!d");
    }

    #[test]
    fn unescape_no_escapes() {
        assert_eq!(unescape("hello", 0), "hello");
    }

    #[test]
    fn unescape_single_escape() {
        assert_eq!(unescape("a\\!b", 1), "a!b");
    }

    #[test]
    fn unescape_double_backslash_is_one() {
        assert_eq!(unescape("\\\\", 1), "\\");
    }

    #[test]
    fn unescape_multibyte_escaped_char() {
        assert_eq!(unescape("\\ツ", 1), "ツ");
    }

    #[test]
    fn unescape_dangling_backslash_after_budget_spent_is_kept() {
        assert_eq!(unescape("a\\!\\", 1), "a!\\");
    }

    #[test]
    fn unescape_count_too_high_with_no_backslash_left_stops_gracefully() {
        assert_eq!(unescape("ab", 2), "ab");
    }

    #[test]
    fn unescape_count_too_high_with_dangling_backslash_stops_gracefully() {
        assert_eq!(unescape("a\\", 2), "a\\");
    }

    #[test]
    fn unescape_count_too_low_leaves_remainder_untouched() {
        assert_eq!(unescape("a\\!b\\!c", 1), "a!b\\!c");
    }

    #[test]
    fn content_empty_span_as_str() {
        let content = Content::empty();
        assert_eq!(content.as_str("anything"), "");
    }

    #[test]
    fn content_is_empty() {
        assert!(Content::empty().is_empty());
        assert!(!Content::span(0..5).is_empty());
    }

    #[test]
    fn content_from_empty_tokens() {
        let content = Content::from_tokens(&[]);
        assert_eq!(content, Content::empty());
    }

    #[test]
    fn escape() {
        let source = "¯\\\\\\_(ツ)\\_/¯";
        let ops = parse(source);
        assert_eq!(ops[2].content.as_str(source), "¯\\_(ツ)_/¯");
        assert_eq!(
            ops,
            vec![
                Op::new_start(Node::Document, Content::empty()),
                Op::new_start(Node::Paragraph, Content::empty()),
                Op::new_value(Content::new(0..16, 3)),
                Op::new_end(Node::Paragraph, Content::empty()),
                Op::new_end(Node::Document, Content::empty())
            ]
        );
    }
}
