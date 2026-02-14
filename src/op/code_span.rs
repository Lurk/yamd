use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_backtick(t: &Token) -> bool {
    t.kind == TokenKind::Backtick && t.range.len() == 1
}

pub fn code_span(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_backtick)?;
    let Some((body, end_token)) = p.advance_until(is_backtick) else {
        p.replace_position(start);
        return None;
    };
    Some(vec![
        Op::new_start(Node::CodeSpan, start_token),
        Op::new_value(body),
        Op::new_end(Node::CodeSpan, end_token),
    ])
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, code_span::code_span, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p: Parser = "`happy path`".into();
        assert_eq!(
            code_span(&p),
            Some(vec![
                Op::new_start(Node::CodeSpan, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::CodeSpan, p.slice(2..3)),
            ])
        );
    }

    #[test]
    fn terminator() {
        let p: Parser = "`happy\n\npath`".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(code_span(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..1, Position::default())
            ))
        )
    }

    #[test]
    fn no_rhs() {
        let p: Parser = "`happy path".into();
        assert_eq!(code_span(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..1, Position::default())
            ))
        )
    }
}
