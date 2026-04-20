use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_left_bracket(t: &Token) -> bool {
    t.kind == TokenKind::LeftSquareBracket && t.range.len() == 1
}

fn is_right_bracket(t: &Token) -> bool {
    t.kind == TokenKind::RightSquareBracket && t.range.len() == 1
}

pub fn title(p: &mut Parser) -> bool {
    let start = p.pos;
    let Some(start_range) = p.eat(is_left_bracket) else {
        return false;
    };
    let Some((body_range, end_range)) = p.eat_until(is_right_bracket) else {
        p.pos = start;
        return false;
    };
    let start_content = p.span(start_range);
    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops.push(Op::new_start(Node::Title, start_content));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Title, end_content));
    true
}
