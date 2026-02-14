use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, italic::italic, parser::Parser, strikethrough::strikethrough},
};

fn is_star2(t: &Token) -> bool {
    t.kind == TokenKind::Star && t.range.len() == 2
}

pub fn bold(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = p.eat(is_star2)?;
    let mut ops = vec![Op::new_start(Node::Bold, start_token)];

    let mut text_start: Option<usize> = None;
    let mut end_token = None;

    while let Some((pos, _)) = p.peek() {
        if p.at_eof() {
            p.replace_position(start);
            return None;
        } else if let Some(tok) = p.eat(is_star2) {
            if let Some(s) = text_start {
                ops.push(Op::new_value(p.slice(s..pos)));
                text_start = None;
            }
            end_token = Some(tok);
            break;
        } else if let Some(nested_ops) = strikethrough(p) {
            if let Some(s) = text_start {
                ops.push(Op::new_value(p.slice(s..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = italic(p) {
            if let Some(s) = text_start {
                ops.push(Op::new_value(p.slice(s..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else {
            text_start.get_or_insert(pos);
            p.next();
        }
    }

    if let Some(end_tok) = end_token {
        if let Some(s) = text_start {
            ops.push(Op::new_value(p.slice(s..p.pos())));
        }
        ops.push(Op::new_end(Node::Bold, end_tok));
        return Some(ops);
    }

    p.replace_position(start);
    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, bold::bold, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p: Parser = "**~~happy~~ _path_**".into();
        assert_eq!(
            bold(&p),
            Some(vec![
                Op::new_start(Node::Bold, p.slice(0..1)),
                Op::new_start(Node::Strikethrough, p.slice(1..2)),
                Op::new_value(p.slice(2..3)),
                Op::new_end(Node::Strikethrough, p.slice(3..4)),
                Op::new_value(p.slice(4..5)),
                Op::new_start(Node::Italic, p.slice(5..6)),
                Op::new_value(p.slice(6..7)),
                Op::new_end(Node::Italic, p.slice(7..8)),
                Op::new_end(Node::Bold, p.slice(8..9)),
            ])
        );
    }

    #[test]
    fn terminator() {
        let p: Parser = "**~~happy~~ _path_\n\n**".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(bold(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        )
    }

    #[test]
    fn end_of_input() {
        let p: Parser = "**~~happy~~ _path_".into();
        assert_eq!(bold(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        )
    }

    #[test]
    fn end_of_input_in_nested() {
        let p: Parser = "**~~happy _path_".into();
        assert_eq!(bold(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        );
    }

    #[test]
    fn text_before_strikethrough() {
        let p: Parser = "**text ~~happy~~ _path_**".into();
        assert_eq!(
            bold(&p),
            Some(vec![
                Op::new_start(Node::Bold, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_start(Node::Strikethrough, p.slice(2..3)),
                Op::new_value(p.slice(3..4)),
                Op::new_end(Node::Strikethrough, p.slice(4..5)),
                Op::new_value(p.slice(5..6)),
                Op::new_start(Node::Italic, p.slice(6..7)),
                Op::new_value(p.slice(7..8)),
                Op::new_end(Node::Italic, p.slice(8..9)),
                Op::new_end(Node::Bold, p.slice(9..10)),
            ])
        );
    }

    #[test]
    fn unclosed_italic() {
        let p: Parser = "**_path**".into();
        assert_eq!(
            bold(&p),
            Some(vec![
                Op::new_start(Node::Bold, p.slice(0..1)),
                Op::new_value(p.slice(1..3)),
                Op::new_end(Node::Bold, p.slice(3..4)),
            ])
        );
    }
}
