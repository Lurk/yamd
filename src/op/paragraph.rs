use crate::op::{
    anchor::anchor,
    bold::bold,
    code_span::code_span,
    emphasis::emphasis,
    italic::italic,
    op::{Node, Op, OpKind},
    parser::{Parser, Query},
    strikethrough::strikethrough,
};

pub fn paragraph<'a>(p: &'a Parser<'a>, eof: &Query) -> Vec<Op<'a>> {
    let mut ops = vec![Op {
        kind: OpKind::Start(Node::Paragraph),
        tokens: vec![],
    }];
    let mut text = Op::new_value(vec![]);
    while p.chain(eof, true).is_some() {
        if let Some(nested_ops) = strikethrough(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = italic(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = bold(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = anchor(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = code_span(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = emphasis(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else if let Some((_, token)) = p.advance() {
            text.tokens.push(token);
        } else {
            break;
        }
    }

    if !text.tokens.is_empty() {
        ops.push(text);
    }
    ops.push(Op {
        kind: OpKind::End(Node::Paragraph),
        tokens: vec![],
    });

    ops
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{
            Op,
            op::{Node, OpKind},
            paragraph::paragraph,
            parser::{Condition, Parser, Query},
        },
    };
    #[test]
    fn test_paragraph() {
        let input = Parser::from(
            "Hello _world_ ~~this is strikethrough~~. **very bold and _italic_ at the same time**",
        );
        let ops = paragraph(&input, &Query::Eof);
        println!("{:#?}", ops);
        assert_eq!(ops.len(), 18);
        assert_eq!(ops[0].kind, OpKind::Start(Node::Paragraph));
        assert_eq!(ops[1].kind, OpKind::Value);
        assert_eq!(ops[2].kind, OpKind::Start(Node::Italic));
        assert_eq!(ops[3].kind, OpKind::Value);
        assert_eq!(ops[4].kind, OpKind::End(Node::Italic));
        assert_eq!(ops[5].kind, OpKind::Value);
        assert_eq!(ops[6].kind, OpKind::Start(Node::Strikethrough));
        assert_eq!(ops[7].kind, OpKind::Value);
        assert_eq!(ops[8].kind, OpKind::End(Node::Strikethrough));
        assert_eq!(ops[9].kind, OpKind::Value);
        assert_eq!(ops[10].kind, OpKind::Start(Node::Bold));
        assert_eq!(ops[11].kind, OpKind::Value);
        assert_eq!(ops[12].kind, OpKind::Start(Node::Italic));
        assert_eq!(ops[13].kind, OpKind::Value);
        assert_eq!(ops[14].kind, OpKind::End(Node::Italic));
        assert_eq!(ops[15].kind, OpKind::Value);
        assert_eq!(ops[16].kind, OpKind::End(Node::Bold));
        assert_eq!(ops[17].kind, OpKind::End(Node::Paragraph));
    }

    #[test]
    pub fn terminated() {
        let p: Parser = "**b** _i_ ~~s~~ [a](u) `c` *e* \n\n".into();
        assert_eq!(
            paragraph(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_start(Node::Bold, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap()]),
                Op::new_end(Node::Bold, vec![p.get(2).unwrap()]),
                Op::new_value(vec![p.get(3).unwrap()]),
                Op::new_start(Node::Italic, vec![p.get(4).unwrap()]),
                Op::new_value(vec![p.get(5).unwrap()]),
                Op::new_end(Node::Italic, vec![p.get(6).unwrap()]),
                Op::new_value(vec![p.get(7).unwrap()]),
                Op::new_start(Node::Strikethrough, vec![p.get(8).unwrap()]),
                Op::new_value(vec![p.get(9).unwrap()]),
                Op::new_end(Node::Strikethrough, vec![p.get(10).unwrap()]),
                Op::new_value(vec![p.get(11).unwrap()]),
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, vec![p.get(12).unwrap()]),
                Op::new_value(vec![p.get(13).unwrap()]),
                Op::new_end(Node::Title, vec![p.get(14).unwrap()]),
                Op::new_start(Node::Destination, vec![p.get(15).unwrap()]),
                Op::new_value(vec![p.get(16).unwrap()]),
                Op::new_end(Node::Destination, vec![p.get(17).unwrap()]),
                Op::new_end(Node::Anchor, vec![]),
                Op::new_value(vec![p.get(18).unwrap()]),
                Op::new_start(Node::CodeSpan, vec![p.get(19).unwrap()]),
                Op::new_value(vec![p.get(20).unwrap()]),
                Op::new_end(Node::CodeSpan, vec![p.get(21).unwrap()]),
                Op::new_value(vec![p.get(22).unwrap()]),
                Op::new_start(Node::Emphasis, vec![p.get(23).unwrap()]),
                Op::new_value(vec![p.get(24).unwrap()]),
                Op::new_end(Node::Emphasis, vec![p.get(25).unwrap()]),
                Op::new_value(vec![p.get(26).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }

    #[test]
    pub fn unterminated() {
        let p: Parser = "_i_ ~~s~~ **b**[a](u) `c` *e* ".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_start(Node::Italic, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap()]),
                Op::new_end(Node::Italic, vec![p.get(2).unwrap()]),
                Op::new_value(vec![p.get(3).unwrap()]),
                Op::new_start(Node::Strikethrough, vec![p.get(4).unwrap()]),
                Op::new_value(vec![p.get(5).unwrap()]),
                Op::new_end(Node::Strikethrough, vec![p.get(6).unwrap()]),
                Op::new_value(vec![p.get(7).unwrap()]),
                Op::new_start(Node::Bold, vec![p.get(8).unwrap()]),
                Op::new_value(vec![p.get(9).unwrap()]),
                Op::new_end(Node::Bold, vec![p.get(10).unwrap()]),
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, vec![p.get(11).unwrap()]),
                Op::new_value(vec![p.get(12).unwrap()]),
                Op::new_end(Node::Title, vec![p.get(13).unwrap()]),
                Op::new_start(Node::Destination, vec![p.get(14).unwrap()]),
                Op::new_value(vec![p.get(15).unwrap()]),
                Op::new_end(Node::Destination, vec![p.get(16).unwrap()]),
                Op::new_end(Node::Anchor, vec![]),
                Op::new_value(vec![p.get(17).unwrap()]),
                Op::new_start(Node::CodeSpan, vec![p.get(18).unwrap()]),
                Op::new_value(vec![p.get(19).unwrap()]),
                Op::new_end(Node::CodeSpan, vec![p.get(20).unwrap()]),
                Op::new_value(vec![p.get(21).unwrap()]),
                Op::new_start(Node::Emphasis, vec![p.get(22).unwrap()]),
                Op::new_value(vec![p.get(23).unwrap()]),
                Op::new_end(Node::Emphasis, vec![p.get(24).unwrap()]),
                Op::new_value(vec![p.get(25).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        );
    }

    #[test]
    pub fn fallback() {
        let p: Parser = "_i_ ~~s~~ **b[a](u) `c` ".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_start(Node::Italic, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap()]),
                Op::new_end(Node::Italic, vec![p.get(2).unwrap()]),
                Op::new_value(vec![p.get(3).unwrap()]),
                Op::new_start(Node::Strikethrough, vec![p.get(4).unwrap()]),
                Op::new_value(vec![p.get(5).unwrap()]),
                Op::new_end(Node::Strikethrough, vec![p.get(6).unwrap()]),
                Op::new_value(Vec::from_iter(p.slice(7..10))),
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, vec![p.get(10).unwrap()]),
                Op::new_value(vec![p.get(11).unwrap()]),
                Op::new_end(Node::Title, vec![p.get(12).unwrap()]),
                Op::new_start(Node::Destination, vec![p.get(13).unwrap()]),
                Op::new_value(vec![p.get(14).unwrap()]),
                Op::new_end(Node::Destination, vec![p.get(15).unwrap()]),
                Op::new_end(Node::Anchor, vec![]),
                Op::new_value(vec![p.get(16).unwrap()]),
                Op::new_start(Node::CodeSpan, vec![p.get(17).unwrap()]),
                Op::new_value(vec![p.get(18).unwrap()]),
                Op::new_end(Node::CodeSpan, vec![p.get(19).unwrap()]),
                Op::new_value(vec![p.get(20).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }

    #[test]
    pub fn eol_before_cb() {
        let p: Parser = "\n%} `c` ".into();
        let q = Query::Is(Condition::new().kind(TokenKind::CollapsibleEnd));
        assert_eq!(
            paragraph(&p, &q),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(vec![p.get(0).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        );
        assert_eq!(
            p.peek(),
            Some((
                1,
                &Token::new(
                    TokenKind::CollapsibleEnd,
                    "%}",
                    Position {
                        byte_index: 1,
                        column: 0,
                        row: 1,
                    }
                )
            ))
        );
    }

    #[test]
    fn eol_at_start() {
        let p: Parser = "\nt".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(vec![p.get(0).unwrap(), p.get(1).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }

    #[test]
    fn eol_after_node() {
        let p: Parser = "~~s~~\nt".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_start(Node::Strikethrough, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap()]),
                Op::new_end(Node::Strikethrough, vec![p.get(2).unwrap()]),
                Op::new_value(vec![p.get(3).unwrap(), p.get(4).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }

    #[test]
    fn not_closed_code_span() {
        let p: Parser = "`".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(vec![p.get(0).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }

    #[test]
    fn not_anchor() {
        let p: Parser = "[]".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(vec![p.get(0).unwrap(), p.get(1).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }

    #[test]
    fn not_italic() {
        let p: Parser = "_".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(vec![p.get(0).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }

    #[test]
    fn not_strikethrough() {
        let p = "~~".into();
        assert_eq!(
            paragraph(&p, &Query::Eof),
            vec![
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(vec![p.get(0).unwrap()]),
                Op::new_end(Node::Paragraph, vec![]),
            ]
        )
    }
}
