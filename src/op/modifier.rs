use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_eol_or_terminator(t: &Token) -> bool {
    t.kind == TokenKind::Eol || t.kind == TokenKind::Terminator
}

pub fn modifier(p: &Parser) -> Option<Vec<Op>> {
    if p.peek().is_some_and(|(_, t)| t.position.column == 0) {
        return None;
    }

    let (body, end_token) = p.advance_until(is_eol_or_terminator)?;

    Some(vec![
        Op::new_start(Node::Modifier, &[]),
        Op::new_value(body),
        Op::new_end(Node::Modifier, end_token),
    ])
}
