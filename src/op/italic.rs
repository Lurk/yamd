use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_underscore(t: &Token) -> bool {
    t.kind == TokenKind::Underscore && t.range.len() == 1
}

pub fn italic(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_underscore)?;
    let Some((body, end_token)) = p.advance_until(is_underscore) else {
        p.replace_position(start);
        return None;
    };
    Some(vec![
        Op::new_start(Node::Italic, start_token),
        Op::new_value(body),
        Op::new_end(Node::Italic, end_token),
    ])
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, italic::italic, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p: Parser = "_happy_".into();
        assert_eq!(
            italic(&p),
            Some(vec![
                Op::new_start(Node::Italic, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::Italic, p.slice(2..3))
            ])
        );
    }

    #[test]
    fn no_closing_token() {
        let p: Parser = "_happy".into();
        assert_eq!(italic(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Underscore, 0..1, Position::default()),
            ))
        )
    }

    #[test]
    fn terminator() {
        let p: Parser = "_ha\n\nppy_".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(italic(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Underscore, 0..1, Position::default())
            ))
        );
    }
}
