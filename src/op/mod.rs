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

#[derive(Debug, PartialEq)]
pub enum OpKind {
    Start(Node),
    End(Node),
    Value,
}

#[derive(Debug, PartialEq)]
pub struct Op {
    pub kind: OpKind,
    pub tokens: Vec<Token>,
}

impl Op {
    pub fn new_value<T: Into<Vec<Token>>>(tokens: T) -> Self {
        Self {
            kind: OpKind::Value,
            tokens: tokens.into(),
        }
    }

    pub fn new_start<T: Into<Vec<Token>>>(node: Node, tokens: T) -> Self {
        Self {
            kind: OpKind::Start(node),
            tokens: tokens.into(),
        }
    }

    pub fn new_end<T: Into<Vec<Token>>>(node: Node, tokens: T) -> Self {
        Self {
            kind: OpKind::End(node),
            tokens: tokens.into(),
        }
    }

    pub fn materialize(&self, source: &str) -> String {
        self.tokens
            .iter()
            .map(|t| &source[t.range.start..t.range.end])
            .collect()
    }
}

pub fn parse<P: Into<Parser>>(parser: P) -> Vec<Op> {
    let parser = parser.into();
    let mut ops = Vec::new();
    if let Some(mut metadata_ops) = metadata::metadata(&parser) {
        ops.append(&mut metadata_ops);
    }
    ops.append(&mut document::document(&parser));
    ops
}

#[cfg(test)]
mod tests {
    use super::*;

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
            for token in &op.tokens {
                for i in token.range.start..token.range.end {
                    assert!(
                        !covered[i],
                        "byte {i} covered by multiple ops (char: {:?})",
                        &TEST_CASE[i..i + 1]
                    );
                    covered[i] = true;
                }
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
            "uncovered byte positions: {:?}",
            uncovered
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
        let metadata_value = ops[1].materialize(TEST_CASE);
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
                fallback_texts.push(ops[i + 1].materialize(TEST_CASE));
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
}
