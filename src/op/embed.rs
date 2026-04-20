use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_left_curly2(t: &Token) -> bool {
    t.kind == TokenKind::LeftCurlyBrace && t.position.column == 0 && t.range.len() == 2
}

fn is_pipe(t: &Token) -> bool {
    t.kind == TokenKind::Pipe && t.range.len() == 1
}

fn is_right_curly2(t: &Token) -> bool {
    t.kind == TokenKind::RightCurlyBrace && t.range.len() == 2
}

fn is_eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

pub fn embed(p: &mut Parser) -> bool {
    let start = p.pos;
    let Some(start_range) = p.eat(is_left_curly2) else {
        return false;
    };

    let Some((lhs_range, sep_range)) = p.eat_until(is_pipe) else {
        p.pos = start;
        return false;
    };

    let Some((rhs_range, close_range)) = p.eat_until(is_right_curly2) else {
        p.pos = start;
        return false;
    };

    let end_range = if let Some(eol_range) = p.eat(is_eol) {
        close_range.start..eol_range.end
    } else if p.at_block_boundary() {
        close_range
    } else {
        p.pos = start;
        return false;
    };

    let start_content = p.span(start_range);
    let lhs_content = p.span(lhs_range);
    let sep_content = p.span(sep_range);
    let rhs_content = p.span(rhs_range);
    let end_content = p.span(end_range);
    p.ops.push(Op::new_start(Node::Embed, start_content));
    p.ops.push(Op::new_value(lhs_content));
    p.ops.push(Op::new_value(sep_content));
    p.ops.push(Op::new_value(rhs_content));
    p.ops.push(Op::new_end(Node::Embed, end_content));
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, embed::embed, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "{{happy|path}}".into();
        assert!(embed(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Embed, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_value(p.span(2..3)),
                Op::new_value(p.span(3..4)),
                Op::new_end(Node::Embed, p.span(4..5)),
            ]
        );
    }

    #[test]
    fn terminator() {
        let mut p: Parser = "{{\n\n|path}}".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!embed(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((
                    0,
                    &Token::new(TokenKind::LeftCurlyBrace, 0..2, Position::default()),
                ))
            );
        });
    }

    #[test]
    fn do_not_have_closing_token() {
        let mut p: Parser = "{{happy|path}".into();
        assert!(!embed(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, 0..2, Position::default()),
            ))
        );
    }

    #[test]
    fn closing_braces_not_at_block_boundary() {
        let mut p: Parser = "{{happy|path}}extra".into();
        assert!(!embed(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(p.pos, 0);
    }

    #[test]
    fn trailing_eol_consumed() {
        let mut p: Parser = "{{happy|path}}\n".into();
        assert!(embed(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Embed, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_value(p.span(2..3)),
                Op::new_value(p.span(3..4)),
                Op::new_end(Node::Embed, p.span(4..6)),
            ]
        );
        assert!(p.at_eof());
    }

    #[test]
    fn no_pipe() {
        let mut p: Parser = "{{happy}}".into();
        assert!(!embed(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, 0..2, Position::default()),
            ))
        )
    }
}
