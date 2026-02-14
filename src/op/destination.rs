use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_left_paren(t: &Token) -> bool {
    t.kind == TokenKind::LeftParenthesis && t.range.len() == 1
}

pub fn destination(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_left_paren)?;

    let mut paren_count = 1;
    let mut end_pos = start;
    while let Some((i, token)) = p.peek() {
        if p.at_eof() {
            break;
        }
        match token.kind {
            TokenKind::LeftParenthesis => {
                paren_count += 1;
                p.replace_position(i + 1);
            }
            TokenKind::RightParenthesis => {
                paren_count -= 1;
                end_pos = i;
                p.replace_position(i + 1);
                if paren_count == 0 {
                    break;
                }
            }
            _ => {
                p.replace_position(i + 1);
            }
        }
    }

    if start == end_pos {
        p.replace_position(start);
        return None;
    }

    let Some(_end_token) = p.get(end_pos) else {
        p.replace_position(start);
        return None;
    };

    let ops = vec![
        Op::new_start(Node::Destination, start_token),
        Op::new_value(p.slice(start + 1..end_pos)),
        Op::new_end(Node::Destination, p.slice(end_pos..end_pos + 1)),
    ];

    p.replace_position(end_pos + 1);
    Some(ops)
}
