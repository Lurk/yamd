use crate::{
    lexer::{Token, TokenKind},
    op::{
        Content, Node, Op, Parser, code::code, collapsible::collapsible, embed::embed,
        heading::heading, highlight::highlight, images::images, list::list, paragraph::paragraph,
        parser::StopCondition, thematic_break::thematic_break,
    },
};

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
