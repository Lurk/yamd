use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, op::Node, parser::Query},
};

pub fn destination<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(&join!(is!(t = TokenKind::LeftParenthesis, el = 1,)), false)?;

    let mut paren_count = 1;
    let mut end_pos = start;
    while let Some((i, token)) = p.peek() {
        if p.chain(eof, true).is_none() {
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

    let Some(end_token) = p.get(end_pos) else {
        p.replace_position(start);
        return None;
    };

    let ops = vec![
        Op::new_start(Node::Destination, Vec::from_iter(start_token)),
        Op::new_value(Vec::from_iter(p.slice(start + 1..end_pos))),
        Op::new_end(Node::Destination, vec![end_token]),
    ];

    p.replace_position(end_pos + 1);
    Some(ops)
}
