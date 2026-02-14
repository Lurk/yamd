use crate::{
    lexer::{Token, TokenKind},
    op::{
        Node, Op, Parser, code::code, collapsible::collapsible, embed::embed, heading::heading,
        highlight::highlight, images::images, list::list, paragraph::paragraph,
        parser::StopCondition, thematic_break::thematic_break,
    },
};

fn is_terminator(t: &Token) -> bool {
    t.kind == TokenKind::Terminator
}

pub fn document(p: &Parser) -> Vec<Op> {
    let mut ops = vec![Op::new_start(Node::Document, &[])];

    while !p.at_eof() {
        if let Some(token) = p.eat(is_terminator) {
            ops.push(Op::new_value(token));
        } else if let Some(nested_ops) = code(p) {
            // code, collapsible, embed, and highlight are tried without Terminator
            // on the stack because they need to scan through \n\n in their content.
            // code and embed use at_block_boundary() for their final boundary check.
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = collapsible(p) {
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = embed(p) {
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = highlight(p) {
            ops.extend(nested_ops);
        } else {
            let _g = p.push_eof(StopCondition::Terminator);
            if let Some(list_ops) = list(p, 0) {
                ops.extend(list_ops);
            } else if let Some(nested_ops) = images(p) {
                ops.extend(nested_ops);
            } else if let Some(nested_ops) = thematic_break(p) {
                ops.extend(nested_ops);
            } else if let Some(nested_ops) = heading(p) {
                ops.extend(nested_ops);
            } else {
                ops.extend(paragraph(p));
            }
        }
    }
    ops.push(Op::new_end(Node::Document, &[]));
    ops
}
