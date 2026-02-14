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

pub fn embed(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_left_curly2)?;

    let Some((lhs, separator_token)) = p.advance_until(is_pipe) else {
        p.replace_position(start);
        return None;
    };

    let Some((rhs, end_token)) = p.advance_until(is_right_curly2) else {
        p.replace_position(start);
        return None;
    };

    if !p.at_block_boundary() {
        p.replace_position(start);
        return None;
    }

    Some(vec![
        Op::new_start(Node::Embed, start_token),
        Op::new_value(lhs),
        Op::new_value(separator_token),
        Op::new_value(rhs),
        Op::new_end(Node::Embed, end_token),
    ])
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, embed::embed, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p = "{{happy|path}}".into();
        assert_eq!(
            embed(&p),
            Some(vec![
                Op::new_start(Node::Embed, p.slice(0..1)), // {{
                Op::new_value(p.slice(1..2)),              // happy
                Op::new_value(p.slice(2..3)),              // |
                Op::new_value(p.slice(3..4)),              // path
                Op::new_end(Node::Embed, p.slice(4..5)),   // }}
            ])
        );
    }

    #[test]
    fn terminator() {
        let p: Parser = "{{\n\n|path}}".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(embed(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, 0..2, Position::default()),
            ))
        )
    }

    #[test]
    fn do_not_have_closing_token() {
        let p = "{{happy|path}".into();
        assert_eq!(embed(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, 0..2, Position::default()),
            ))
        );
    }

    #[test]
    fn no_pipe() {
        let p = "{{happy}}".into();
        assert_eq!(embed(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, 0..2, Position::default()),
            ))
        )
    }
}
