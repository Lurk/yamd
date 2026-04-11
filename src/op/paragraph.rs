use crate::op::{
    Content, Node, Op, anchor::anchor, bold::bold, code_span::code_span, emphasis::emphasis,
    italic::italic, parser::Parser, strikethrough::strikethrough,
};

pub fn paragraph(p: &mut Parser) {
    p.ops
        .push(Op::new_start(Node::Paragraph, Content::Span(0..0)));
    let mut text_start: Option<usize> = None;
    while !p.at_eof() {
        let pos = p.pos;
        let snap = p.ops.len();
        let matched =
            strikethrough(p) || italic(p) || bold(p) || anchor(p) || code_span(p) || emphasis(p);
        if matched {
            if let Some(start) = text_start.take() {
                let content = p.span(start..pos);
                p.ops.insert(snap, Op::new_value(content));
            }
        } else if let Some(pos) = p.advance() {
            text_start.get_or_insert(pos);
        } else {
            break;
        }
    }

    if let Some(start) = text_start {
        let end = p.pos;
        if start < end {
            let content = p.span(start..end);
            p.ops.push(Op::new_value(content));
        }
    }
    p.ops
        .push(Op::new_end(Node::Paragraph, Content::Span(0..0)));
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::TokenKind,
        op::{
            Content, Node, Op, OpKind,
            paragraph::paragraph,
            parser::{Parser, StopCondition},
        },
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "hello **world** _foo_ ~~bar~~ `code` *em* [a](u)".into();

        paragraph(&mut p);

        assert_eq!(p.ops.first().unwrap().kind, OpKind::Start(Node::Paragraph));
        assert_eq!(p.ops.last().unwrap().kind, OpKind::End(Node::Paragraph));
        assert!(p.ops.len() > 2);
    }

    #[test]
    fn terminator() {
        let mut p: Parser = "hello\n\nworld".into();
        p.with_eof(StopCondition::Terminator, |p| {
            paragraph(p);
            assert_eq!(p.ops.first().unwrap().kind, OpKind::Start(Node::Paragraph));
            assert_eq!(p.ops.last().unwrap().kind, OpKind::End(Node::Paragraph));
            let (_, token) = p.peek().unwrap();
            assert_eq!(token.kind, TokenKind::Terminator);
        });
    }

    #[test]
    fn text_only() {
        let mut p: Parser = "hello world".into();
        paragraph(&mut p);
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(0..p.len())),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
            ]
        );
    }
}
