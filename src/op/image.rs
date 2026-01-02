use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, destination::destination, op::Node, parser::Query, title::title},
};

pub fn image<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(&join!(is!(t = TokenKind::Bang, el = 1,)), false)?;

    let Some(alt_ops) = title(p, eof) else {
        p.replace_position(start);
        return None;
    };

    let Some(dest_ops) = destination(p, eof) else {
        p.replace_position(start);
        return None;
    };

    let mut ops = vec![Op::new_start(Node::Image, Vec::from_iter(start_token))];
    ops.extend(alt_ops);
    ops.extend(dest_ops);

    if let Some(t) = p.chain(&join!(is!(t = TokenKind::Eol,)), false) {
        ops.push(Op::new_end(Node::Image, Vec::from_iter(t)));
        return Some(ops);
    } else if p.chain(eof, true).is_none() {
        ops.push(Op::new_end(Node::Image, vec![]));
        return Some(ops);
    }

    p.replace_position(start);
    None
}
