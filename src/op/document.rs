use crate::{
    lexer::{Token, TokenKind},
    op::{
        Content, Node, Op, Parser, code::code, collapsible::collapsible, embed::embed,
        heading::heading, highlight::highlight, images::images, list::list, paragraph::paragraph,
        parser::StopCondition, thematic_break::thematic_break,
    },
};

/// Minimal fixture for each Node variant that parses as a standalone block outside
/// `with_eof(Terminator)` in the document loop.
///
/// The exhaustive match is the point: adding a new `Node` variant triggers a compile
/// error here, which forces you to decide whether it needs EOL-consumption coverage.
/// If it does, return `Some("…")` and the test below will verify it. If not, add it
/// to the `None` arm.
#[cfg(test)]
fn block_fixture(node: &Node) -> Option<&'static str> {
    match node {
        Node::Code => Some("```\ncode\n```"),
        Node::Collapsible => Some("{% Title\ntext\n%}"),
        Node::Embed => Some("{{a|b}}"),
        Node::Highlight => Some("!! Title\ntext\n!!"),
        Node::Anchor
        | Node::Bold
        | Node::CodeSpan
        | Node::Destination
        | Node::Document
        | Node::Emphasis
        | Node::Heading
        | Node::Icon
        | Node::Image
        | Node::Images
        | Node::Italic
        | Node::ListItem
        | Node::Metadata
        | Node::Modifier
        | Node::OrderedList
        | Node::Paragraph
        | Node::Strikethrough
        | Node::ThematicBreak
        | Node::Title
        | Node::UnorderedList => None,
    }
}

fn is_terminator(t: &Token) -> bool {
    t.kind == TokenKind::Terminator
}

pub fn document(p: &mut Parser) {
    p.ops
        .push(Op::new_start(Node::Document, Content::Span(0..0)));

    while !p.at_eof() {
        let before = p.pos;

        if let Some(range) = p.eat(is_terminator) {
            let content = p.span(range);
            p.ops.push(Op::new_value(content));
        } else if code(p) || collapsible(p) || embed(p) || highlight(p) {
        } else {
            p.with_eof(StopCondition::Terminator, |p| {
                if !list(p, 0) && !images(p) && !thematic_break(p) && !heading(p) {
                    paragraph(p);
                }
            });
        }

        debug_assert!(
            p.pos > before,
            "document loop made no progress at token {before}"
        );
    }
    p.ops.push(Op::new_end(Node::Document, Content::Span(0..0)));
}

#[cfg(test)]
mod tests {
    use super::block_fixture;
    use super::*;
    use crate::op::OpKind;

    /// Verifies that every block element listed in [`block_fixture`] consumes its
    /// trailing EOL (or stops at a block boundary) so that the next element does not
    /// inherit a stray newline.
    ///
    /// If this test fails for a new block type, the parser for that block needs to
    /// eat the trailing EOL before returning success — see `code.rs` or `embed.rs`
    /// for the canonical pattern.
    #[test]
    fn block_elements_consume_trailing_eol() {
        let nodes_with_fixtures = [Node::Code, Node::Collapsible, Node::Embed, Node::Highlight];

        for node in &nodes_with_fixtures {
            let fixture = block_fixture(node).expect("node listed in test must have a fixture");

            let input = format!("{fixture}\nparagraph_text");
            let mut p: Parser = input.as_str().into();
            document(&mut p);

            // Find the last Paragraph in the op stream — that's the one after
            // the block, not one nested inside it (collapsible/highlight contain
            // inner paragraphs).
            let last_para_start = p
                .ops
                .iter()
                .rposition(|op| op.kind == OpKind::Start(Node::Paragraph))
                .expect("expected a trailing paragraph");
            let mut paragraph_text = String::new();
            for op in &p.ops[last_para_start + 1..] {
                match &op.kind {
                    OpKind::End(Node::Paragraph) => break,
                    OpKind::Value => {
                        paragraph_text.push_str(op.content.as_str(input.as_str()));
                    }
                    _ => {}
                }
            }

            assert_eq!(
                paragraph_text, "paragraph_text",
                "{node:?} block did not consume its trailing EOL — \
                 the following paragraph inherited a stray newline"
            );
        }
    }
}
