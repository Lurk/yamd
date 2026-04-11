use crate::{
    lexer::{Token, TokenKind},
    op::{Content, Node, Op, Parser, destination::destination, title::title},
};

fn is_bang(t: &Token) -> bool {
    t.kind == TokenKind::Bang && t.range.len() == 1
}

fn is_eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

pub fn image(p: &mut Parser) -> bool {
    let start = p.pos;
    let snap = p.ops.len();
    let Some(start_range) = p.eat(is_bang) else {
        return false;
    };

    let start_content = p.span(start_range);
    p.ops.push(Op::new_start(Node::Image, start_content));

    if !title(p) {
        p.pos = start;
        p.ops.truncate(snap);
        return false;
    }

    if !destination(p) {
        p.pos = start;
        p.ops.truncate(snap);
        return false;
    }

    if let Some(eol_range) = p.eat(is_eol) {
        let end_content = p.span(eol_range);
        p.ops.push(Op::new_end(Node::Image, end_content));
        return true;
    } else if p.at_eof() {
        p.ops.push(Op::new_end(Node::Image, Content::Span(0..0)));
        return true;
    }

    p.pos = start;
    p.ops.truncate(snap);
    false
}
