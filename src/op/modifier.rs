use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, op::Node, parser::Query},
};

pub fn modifier<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    p.chain(&is!(c = 0,), true)?;

    let (body, end_token) = p.advance_until(&join!(is!(t = TokenKind::Eol,)), eof)?;

    Some(vec![
        Op::new_start(Node::Modifier, vec![]),
        Op::new_value(Vec::from_iter(body)),
        Op::new_end(Node::Modifier, Vec::from_iter(end_token)),
    ])
}
