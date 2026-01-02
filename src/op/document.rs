use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        Op, Parser, code::code, collapsible::collapsible, embed::embed, heading::heading,
        highlight::highlight, images::images, list::list, op::Node, paragraph::paragraph,
        parser::Query, thematic_break::thematic_break,
    },
    or,
};

pub fn document<'a>(p: &'a Parser, eof: &Query) -> Vec<Op<'a>> {
    let mut ops = vec![Op::new_start(Node::Document, vec![])];

    let next_eof = or!(eof.clone(), is!(t = TokenKind::Terminator,));

    while p.chain(eof, true).is_some() {
        if let Some(token) = p.chain(&join!(is!(t = TokenKind::Terminator,)), false) {
            ops.push(Op::new_value(Vec::from_iter(token)));
        } else if let Some(nested_ops) = code(p, eof) {
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = collapsible(p, eof) {
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = embed(p, eof) {
            ops.extend(nested_ops);
        } else if let Some(list_ops) = list(p, 0, eof) {
            ops.extend(list_ops);
        } else if let Some(nested_ops) = heading(p, &next_eof) {
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = highlight(p, eof) {
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = images(p, eof) {
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = thematic_break(p) {
            ops.extend(nested_ops);
        } else {
            let paragraph_ops = paragraph(p, &next_eof);
            ops.extend(paragraph_ops);
        }
    }
    ops.push(Op::new_end(Node::Document, vec![]));
    ops
}
