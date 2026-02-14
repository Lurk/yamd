use crate::op::{
    Node, Op, anchor::anchor, bold::bold, code_span::code_span, emphasis::emphasis, italic::italic,
    parser::Parser, strikethrough::strikethrough,
};

pub fn paragraph(p: &Parser) -> Vec<Op> {
    let mut ops = vec![Op::new_start(Node::Paragraph, &[])];
    let mut text_start: Option<usize> = None;
    while !p.at_eof() {
        let pos = p.pos();
        if let Some(nested_ops) = strikethrough(p) {
            if let Some(start) = text_start {
                ops.push(Op::new_value(p.slice(start..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = italic(p) {
            if let Some(start) = text_start {
                ops.push(Op::new_value(p.slice(start..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = bold(p) {
            if let Some(start) = text_start {
                ops.push(Op::new_value(p.slice(start..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = anchor(p) {
            if let Some(start) = text_start {
                ops.push(Op::new_value(p.slice(start..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = code_span(p) {
            if let Some(start) = text_start {
                ops.push(Op::new_value(p.slice(start..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = emphasis(p) {
            if let Some(start) = text_start {
                ops.push(Op::new_value(p.slice(start..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else if let Some((pos, _)) = p.advance() {
            text_start.get_or_insert(pos);
        } else {
            break;
        }
    }

    if let Some(start) = text_start {
        let end = p.pos();
        if start < end {
            ops.push(Op::new_value(p.slice(start..end)));
        }
    }
    ops.push(Op::new_end(Node::Paragraph, &[]));
    ops
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::TokenKind,
        op::{
            Node, Op, OpKind,
            paragraph::paragraph,
            parser::{Parser, StopCondition},
        },
    };

    #[test]
    fn happy_path() {
        let p: Parser = "hello **world** _foo_ ~~bar~~ `code` *em* [a](u)".into();

        let ops = paragraph(&p);

        assert_eq!(ops.first().unwrap().kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops.last().unwrap().kind, OpKind::End(Node::Paragraph));
        assert!(ops.len() > 2);
    }

    #[test]
    fn terminator() {
        let p: Parser = "hello\n\nworld".into();
        let _g = p.push_eof(StopCondition::Terminator);
        let ops = paragraph(&p);
        assert_eq!(ops.first().unwrap().kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops.last().unwrap().kind, OpKind::End(Node::Paragraph));
        let (_, token) = p.peek().unwrap();
        assert_eq!(token.kind, TokenKind::Terminator);
    }

    #[test]
    fn text_only() {
        let p: Parser = "hello world".into();
        let ops = paragraph(&p);
        assert_eq!(
            ops,
            vec![
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(0..p.len())),
                Op::new_end(Node::Paragraph, &[]),
            ]
        );
    }
}
