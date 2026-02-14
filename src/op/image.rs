use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser, destination::destination, title::title},
};

fn is_bang(t: &Token) -> bool {
    t.kind == TokenKind::Bang && t.range.len() == 1
}

fn is_eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

pub fn image(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_bang)?;

    let Some(alt_ops) = title(p) else {
        p.replace_position(start);
        return None;
    };

    let Some(dest_ops) = destination(p) else {
        p.replace_position(start);
        return None;
    };

    let mut ops = vec![Op::new_start(Node::Image, start_token)];
    ops.extend(alt_ops);
    ops.extend(dest_ops);

    if let Some(t) = p.eat(is_eol) {
        ops.push(Op::new_end(Node::Image, t));
        return Some(ops);
    } else if p.at_eof() {
        ops.push(Op::new_end(Node::Image, &[]));
        return Some(ops);
    }

    p.replace_position(start);
    None
}
