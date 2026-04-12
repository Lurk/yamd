use crate::{
    lexer::{Token, TokenKind},
    op::{Content, Node, Op, Parser},
};

fn is_eol_or_terminator(t: &Token) -> bool {
    t.kind == TokenKind::Eol || t.kind == TokenKind::Terminator
}

pub fn modifier(p: &mut Parser) -> bool {
    if p.peek().is_some_and(|(_, t)| t.position.column == 0) {
        return false;
    }

    let Some((body_range, end_range)) = p.advance_until(is_eol_or_terminator) else {
        return false;
    };

    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops
        .push(Op::new_start(Node::Modifier, Content::Span(0..0)));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Modifier, end_content));
    true
}
