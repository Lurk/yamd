use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_tilde(t: &Token) -> bool {
    t.kind == TokenKind::Tilde && t.range.len() == 2
}

pub fn strikethrough(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_tilde)?;
    let Some((body, end_token)) = p.advance_until(is_tilde) else {
        p.replace_position(start);
        return None;
    };
    Some(vec![
        Op::new_start(Node::Strikethrough, start_token),
        Op::new_value(body),
        Op::new_end(Node::Strikethrough, end_token),
    ])
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, parser::StopCondition, strikethrough::strikethrough},
    };

    #[test]
    fn happy_path() {
        let p = "~~happy~~".into();
        assert_eq!(
            strikethrough(&p),
            Some(vec![
                Op::new_start(Node::Strikethrough, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::Strikethrough, p.slice(2..3)),
            ])
        );
    }

    #[test]
    fn no_closing_token() {
        let p = "~~happy".into();
        assert_eq!(strikethrough(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Tilde, 0..2, Position::default())))
        )
    }

    #[test]
    fn terminator() {
        let p: Parser = "~~ha\n\nppy~~".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(strikethrough(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Tilde, 0..2, Position::default())))
        )
    }
}
