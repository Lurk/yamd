use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_star(t: &Token) -> bool {
    t.kind == TokenKind::Star && t.range.len() == 1
}

pub fn emphasis(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_star)?;
    let Some((body, end_token)) = p.advance_until(is_star) else {
        p.replace_position(start);
        return None;
    };
    Some(vec![
        Op::new_start(Node::Emphasis, start_token),
        Op::new_value(body),
        Op::new_end(Node::Emphasis, end_token),
    ])
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, emphasis::emphasis, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p: Parser = "*happy*".into();
        assert_eq!(
            emphasis(&p),
            Some(vec![
                Op::new_start(Node::Emphasis, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::Emphasis, p.slice(2..3)),
            ])
        );
    }

    #[test]
    fn no_closing_token() {
        let p: Parser = "*happy".into();
        assert_eq!(emphasis(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..1, Position::default())))
        )
    }

    #[test]
    fn terminator() {
        let p: Parser = "*ha\n\nppy*".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(emphasis(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..1, Position::default())))
        );
    }
}
