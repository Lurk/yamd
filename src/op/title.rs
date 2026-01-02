use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, op::Node, parser::Query},
};

pub fn title<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(
        &join!(is!(t = TokenKind::LeftSquareBracket, el = 1,)),
        false,
    )?;
    let Some((body, end_token)) =
        p.advance_until(&join!(is!(t = TokenKind::RightSquareBracket, el = 1,)), eof)
    else {
        p.replace_position(start);
        return None;
    };
    let ops = vec![
        Op::new_start(Node::Title, Vec::from_iter(start_token)),
        Op::new_value(Vec::from_iter(body)),
        Op::new_end(Node::Title, Vec::from_iter(end_token)),
    ];
    Some(ops)
}
