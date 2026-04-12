use std::ops::Range;

use crate::lexer::Token;
pub use crate::op::parser::Parser;

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
pub mod parser;
mod strikethrough;
mod thematic_break;
mod title;
mod to_yamd;
pub use to_yamd::to_yamd;

/// Text content extracted from the source input.
///
/// `Content` provides zero-copy access to source text when possible. Use [`Span`](Content::Span)
/// when the text maps to a contiguous byte range in the source. Use
/// [`Materialized`](Content::Materialized) when the text was assembled from non-contiguous tokens
/// (e.g., after escape processing removed backslashes).
#[derive(Debug, PartialEq)]
pub enum Content {
    /// A contiguous byte range in the source string. Avoids allocation by referencing the original input directly.
    Span(Range<usize>),
    /// An owned string for text assembled from non-contiguous tokens.
    Materialized(String),
}

impl Content {
    /// Returns the text this content represents, borrowing from `source` for [`Span`](Content::Span) variants.
    pub fn as_str<'a>(&'a self, source: &'a str) -> &'a str {
        match self {
            Content::Span(range) => {
                if range.is_empty() {
                    ""
                } else {
                    &source[range.clone()]
                }
            }
            Content::Materialized(s) => s.as_str(),
        }
    }

    /// Returns an owned copy of the text this content represents.
    pub fn to_string(&self, source: &str) -> String {
        self.as_str(source).to_owned()
    }

    /// Returns `true` if this content represents an empty string.
    pub fn is_empty(&self) -> bool {
        match self {
            Content::Span(range) => range.is_empty(),
            Content::Materialized(s) => s.is_empty(),
        }
    }

    /// Builds `Content` from a token slice. Produces a [`Span`](Content::Span) when tokens are contiguous in `source`, or [`Materialized`](Content::Materialized) when gaps exist (e.g., escape characters were removed).
    pub fn from_tokens(tokens: &[Token], source: &str) -> Self {
        if tokens.is_empty() {
            return Content::Span(0..0);
        }
        let is_contiguous = tokens
            .windows(2)
            .all(|w| w[0].range.end == w[1].range.start);
        if is_contiguous {
            let start = tokens.first().unwrap().range.start;
            let end = tokens.last().unwrap().range.end;
            Content::Span(start..end)
        } else {
            let s: String = tokens.iter().map(|t| &source[t.range.clone()]).collect();
            Content::Materialized(s)
        }
    }
}

impl From<&[Token]> for Content {
    fn from(tokens: &[Token]) -> Self {
        if tokens.is_empty() {
            Content::Span(0..0)
        } else {
            let start = tokens.first().unwrap().range.start;
            let end = tokens.last().unwrap().range.end;
            Content::Span(start..end)
        }
    }
}

impl<const N: usize> From<&[Token; N]> for Content {
    fn from(tokens: &[Token; N]) -> Self {
        Content::from(tokens.as_slice())
    }
}

impl From<Vec<Token>> for Content {
    fn from(tokens: Vec<Token>) -> Self {
        Content::from(tokens.as_slice())
    }
}

impl From<String> for Content {
    fn from(s: String) -> Self {
        Content::Materialized(s)
    }
}

/// Identifies which AST node type an [`Op`] refers to.
///
/// Used in [`OpKind::Start`] and [`OpKind::End`] to mark the boundaries of nested structures
/// in the flat operation stream.
#[derive(Debug, PartialEq)]
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
pub struct Op {
    pub kind: OpKind,
    pub content: Content,
}

impl Op {
    /// Creates a [`Value`](OpKind::Value) operation with the given content.
    pub fn new_value<T: Into<Content>>(tokens: T) -> Self {
        Self {
            kind: OpKind::Value,
            content: tokens.into(),
        }
    }

    /// Creates a [`Start`](OpKind::Start) operation for the given node type.
    pub fn new_start<T: Into<Content>>(node: Node, tokens: T) -> Self {
        Self {
            kind: OpKind::Start(node),
            content: tokens.into(),
        }
    }

    /// Creates an [`End`](OpKind::End) operation for the given node type.
    pub fn new_end<T: Into<Content>>(node: Node, tokens: T) -> Self {
        Self {
            kind: OpKind::End(node),
            content: tokens.into(),
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
    use crate::lexer::{Position, TokenKind};

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

end"#;

    #[test]
    fn all_source_bytes_are_covered_exactly_once() {
        let ops = parse(TEST_CASE);
        let mut covered = vec![false; TEST_CASE.len()];
        for op in &ops {
            match &op.content {
                Content::Span(range) => {
                    for i in range.clone() {
                        assert!(
                            !covered[i],
                            "byte {i} covered by multiple ops (char: {:?})",
                            &TEST_CASE[i..i + 1]
                        );
                        covered[i] = true;
                    }
                }
                Content::Materialized(_) => {}
            }
        }
        let uncovered: Vec<usize> = covered
            .iter()
            .enumerate()
            .filter(|&(_, b)| !b)
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

        let mut block_nodes: Vec<&Node> = vec![];
        let mut depth: usize = 0;
        for op in &ops {
            match &op.kind {
                OpKind::Start(node) => {
                    if depth == 1 {
                        block_nodes.push(node);
                    }
                    depth += 1;
                }
                OpKind::End(_) => {
                    depth -= 1;
                }
                OpKind::Value => {}
            }
        }

        assert_eq!(
            block_nodes,
            vec![
                &Node::Heading,       // # hello
                &Node::Code,          // ```rust ... ```
                &Node::Paragraph,     // t**b**
                &Node::Image,         // ![a](u)
                &Node::Images,        // ![a](u)\n![a2](u2)
                &Node::Highlight,     // !! H ... !!
                &Node::ThematicBreak, // -----
                &Node::UnorderedList, // - one\n - two
                &Node::OrderedList,   // + first\n + second
                &Node::Embed,         // {{youtube|123}}
                &Node::Embed,         // {{cloudinary_gallery|...}}
                &Node::Collapsible,   // {% collapsible ... %}
                &Node::Collapsible,   // {% one more collapsible ... %}
                &Node::Paragraph,     // + (fallback)
                &Node::Paragraph,     // - (fallback)
                &Node::Paragraph,     // ![]( (fallback)
                &Node::Paragraph,     // ``` (fallback)
                &Node::Paragraph,     // end (fallback)
            ]
        );
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

        let mut fallback_texts: Vec<String> = vec![];
        let mut i = 0;
        while i < ops.len() {
            if ops[i].kind == OpKind::Start(Node::Paragraph)
                && i + 2 < ops.len()
                && ops[i + 1].kind == OpKind::Value
                && ops[i + 2].kind == OpKind::End(Node::Paragraph)
            {
                fallback_texts.push(ops[i + 1].content.to_string(TEST_CASE));
            }
            i += 1;
        }

        let tail: Vec<&str> = fallback_texts.iter().map(|s| s.as_str()).collect();
        assert!(
            tail.contains(&"+"),
            "should contain '+' fallback paragraph, got: {tail:?}"
        );
        assert!(
            tail.contains(&"-"),
            "should contain '-' fallback paragraph, got: {tail:?}"
        );
        assert!(
            tail.contains(&"end"),
            "should contain 'end' fallback paragraph, got: {tail:?}"
        );
    }

    #[test]
    fn content_span_as_str() {
        let source = "hello world";
        let content = Content::Span(0..5);
        assert_eq!(content.as_str(source), "hello");
    }

    #[test]
    fn content_materialized_as_str() {
        let content = Content::Materialized(String::from("hello"));
        assert_eq!(content.as_str("ignored source"), "hello");
    }

    #[test]
    fn content_span_to_string() {
        let source = "hello world";
        let content = Content::Span(0..5);
        assert_eq!(content.to_string(source), "hello");
    }

    #[test]
    fn content_from_non_contiguous_tokens_materializes() {
        let source = "a\\!b";
        let tokens = vec![
            Token::new(TokenKind::Literal, 0..1, Position::default()),
            Token {
                kind: TokenKind::Literal,
                range: 2..3,
                position: Position {
                    byte_index: 2,
                    column: 1,
                    row: 0,
                },
                escaped: true,
            },
            Token::new(
                TokenKind::Literal,
                3..4,
                Position {
                    byte_index: 3,
                    column: 2,
                    row: 0,
                },
            ),
        ];
        let content = Content::from_tokens(&tokens, source);
        assert_eq!(content, Content::Materialized(String::from("a!b")));
    }

    #[test]
    fn content_from_contiguous_tokens_stays_span() {
        let source = "hello";
        let tokens = vec![Token::new(TokenKind::Literal, 0..5, Position::default())];
        let content = Content::from_tokens(&tokens, source);
        assert_eq!(content, Content::Span(0..5));
    }
}
