use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_left_paren(t: &Token) -> bool {
    t.kind == TokenKind::LeftParenthesis && t.range.len() == 1
}

pub fn destination(p: &mut Parser) -> bool {
    let start = p.pos;
    let Some(start_range) = p.eat(is_left_paren) else {
        return false;
    };

    let mut paren_count = 1;
    let mut end_pos = start;
    while let Some((i, token)) = p.peek() {
        if p.at_eof() {
            break;
        }
        match token.kind {
            TokenKind::LeftParenthesis => {
                paren_count += 1;
                p.pos = i + 1;
            }
            TokenKind::RightParenthesis => {
                paren_count -= 1;
                end_pos = i;
                p.pos = i + 1;
                if paren_count == 0 {
                    break;
                }
            }
            _ => {
                p.pos = i + 1;
            }
        }
    }

    if start == end_pos {
        p.pos = start;
        return false;
    }

    if p.get(end_pos).is_none() {
        p.pos = start;
        return false;
    }

    let start_content = p.span(start_range);
    let body_range = start + 1..end_pos;
    let body_content = p.span(body_range);
    let end_content = p.span(end_pos..end_pos + 1);
    p.pos = end_pos + 1;
    p.ops.push(Op::new_start(Node::Destination, start_content));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Destination, end_content));
    true
}
