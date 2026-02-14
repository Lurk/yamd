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

pub fn title(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_left_bracket)?;
    let Some((body, end_token)) = p.advance_until(is_right_bracket) else {
        p.replace_position(start);
        return None;
    };
    Some(vec![
        Op::new_start(Node::Title, start_token),
        Op::new_value(body),
        Op::new_end(Node::Title, end_token),
    ])
}
