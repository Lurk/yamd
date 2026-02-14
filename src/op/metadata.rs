use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_three_dashes(t: &Token) -> bool {
    t.kind == TokenKind::Minus && t.position.column == 0 && t.range.len() == 3
}

pub fn metadata(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_three_dashes)?;

    let Some((body, end_token)) = p.advance_until(is_three_dashes) else {
        p.replace_position(start);
        return None;
    };

    Some(vec![
        Op::new_start(Node::Metadata, start_token),
        Op::new_value(body),
        Op::new_end(Node::Metadata, end_token),
    ])
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, metadata::metadata},
    };

    #[test]
    fn happy_path() {
        let p = "---\ncontent\n---".into();
        assert_eq!(
            metadata(&p),
            Some(vec![
                Op::new_start(Node::Metadata, p.slice(0..1)),
                Op::new_value(p.slice(1..4)),
                Op::new_end(Node::Metadata, p.slice(4..5)),
            ])
        );
    }

    #[test]
    fn no_closing_delimiter() {
        let p = "---\ncontent".into();
        assert_eq!(metadata(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Minus, 0..3, Position::default())))
        );
    }

    #[test]
    fn no_content() {
        let p = "---\n---".into();
        assert_eq!(
            metadata(&p),
            Some(vec![
                Op::new_start(Node::Metadata, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::Metadata, p.slice(2..3)),
            ]),
        );
    }

    #[test]
    fn only_opening_token() {
        let p = "---".into();
        assert_eq!(metadata(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Minus, 0..3, Position::default())))
        );
    }
}
