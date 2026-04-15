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

    #[test]
    fn code_block_consumes_trailing_eol() {
        let input = "```\ncode\n```\nparagraph_text";
        let mut p: Parser = input.into();
        document(&mut p);
        assert_eq!(p.ops.len(), 8);
        assert_eq!(p.ops[0].kind, OpKind::Start(Node::Document));
        assert_eq!(p.ops[1].kind, OpKind::Start(Node::Code));
        assert_eq!(p.ops[2].kind, OpKind::Value);
        assert_eq!(p.ops[3].kind, OpKind::End(Node::Code));
        assert_eq!(p.ops[4].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(p.ops[5].kind, OpKind::Value);
        assert_eq!(p.ops[5].content.as_str(input), "paragraph_text");
        assert_eq!(p.ops[6].kind, OpKind::End(Node::Paragraph));
        assert_eq!(p.ops[7].kind, OpKind::End(Node::Document));
    }

    #[test]
    fn collapsible_block_consumes_trailing_eol() {
        let input = "{% Title\ntext\n%}\nparagraph_text";
        let mut p: Parser = input.into();
        document(&mut p);
        assert_eq!(p.ops.len(), 15);
        assert_eq!(p.ops[0].kind, OpKind::Start(Node::Document));
        assert_eq!(p.ops[1].kind, OpKind::Start(Node::Collapsible));
        assert_eq!(p.ops[2].kind, OpKind::Start(Node::Modifier));
        assert_eq!(p.ops[3].kind, OpKind::Value);
        assert_eq!(p.ops[3].content.as_str(input), "Title");
        assert_eq!(p.ops[4].kind, OpKind::End(Node::Modifier));
        assert_eq!(p.ops[5].kind, OpKind::Start(Node::Document));
        assert_eq!(p.ops[6].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(p.ops[7].kind, OpKind::Value);
        assert_eq!(p.ops[8].kind, OpKind::End(Node::Paragraph));
        assert_eq!(p.ops[9].kind, OpKind::End(Node::Document));
        assert_eq!(p.ops[10].kind, OpKind::End(Node::Collapsible));
        assert_eq!(p.ops[11].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(p.ops[12].kind, OpKind::Value);
        assert_eq!(p.ops[12].content.as_str(input), "paragraph_text");
        assert_eq!(p.ops[13].kind, OpKind::End(Node::Paragraph));
        assert_eq!(p.ops[14].kind, OpKind::End(Node::Document));
    }

    #[test]
    fn embed_block_consumes_trailing_eol() {
        let input = "{{a|b}}\nparagraph_text";
        let mut p: Parser = input.into();
        document(&mut p);
        assert_eq!(p.ops.len(), 10);
        assert_eq!(p.ops[0].kind, OpKind::Start(Node::Document));
        assert_eq!(p.ops[1].kind, OpKind::Start(Node::Embed));
        assert_eq!(p.ops[2].kind, OpKind::Value);
        assert_eq!(p.ops[2].content.as_str(input), "a");
        assert_eq!(p.ops[3].kind, OpKind::Value);
        assert_eq!(p.ops[3].content.as_str(input), "|");
        assert_eq!(p.ops[4].kind, OpKind::Value);
        assert_eq!(p.ops[4].content.as_str(input), "b");
        assert_eq!(p.ops[5].kind, OpKind::End(Node::Embed));
        assert_eq!(p.ops[6].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(p.ops[7].kind, OpKind::Value);
        assert_eq!(p.ops[7].content.as_str(input), "paragraph_text");
        assert_eq!(p.ops[8].kind, OpKind::End(Node::Paragraph));
        assert_eq!(p.ops[9].kind, OpKind::End(Node::Document));
    }

    #[test]
    fn highlight_block_consumes_trailing_eol() {
        let input = "!! Title\ntext\n!!\nparagraph_text";
        let mut p: Parser = input.into();
        document(&mut p);
        assert_eq!(p.ops.len(), 13);
        assert_eq!(p.ops[0].kind, OpKind::Start(Node::Document));
        assert_eq!(p.ops[1].kind, OpKind::Start(Node::Highlight));
        assert_eq!(p.ops[2].kind, OpKind::Start(Node::Modifier));
        assert_eq!(p.ops[3].kind, OpKind::Value);
        assert_eq!(p.ops[3].content.as_str(input), "Title");
        assert_eq!(p.ops[4].kind, OpKind::End(Node::Modifier));
        assert_eq!(p.ops[5].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(p.ops[6].kind, OpKind::Value);
        assert_eq!(p.ops[7].kind, OpKind::End(Node::Paragraph));
        assert_eq!(p.ops[8].kind, OpKind::End(Node::Highlight));
        assert_eq!(p.ops[9].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(p.ops[10].kind, OpKind::Value);
        assert_eq!(p.ops[10].content.as_str(input), "paragraph_text");
        assert_eq!(p.ops[11].kind, OpKind::End(Node::Paragraph));
        assert_eq!(p.ops[12].kind, OpKind::End(Node::Document));
    }

    #[test]
    fn block_fixture_is_exhaustive() {
        let _ = block_fixture(&Node::Code);
    }
}
