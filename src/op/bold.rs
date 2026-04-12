use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, italic::italic, parser::Parser, strikethrough::strikethrough},
};

fn is_star2(t: &Token) -> bool {
    t.kind == TokenKind::Star && t.range.len() == 2
}

pub fn bold(p: &mut Parser) -> bool {
    let start = p.pos;
    let snap = p.ops.len();
    let Some(start_range) = p.eat(is_star2) else {
        return false;
    };
    let start_content = p.span(start_range);
    p.ops.push(Op::new_start(Node::Bold, start_content));

    let mut text_start: Option<usize> = None;
    let mut found_end = false;

    while let Some((pos, _)) = p.peek() {
        if p.at_eof() {
            break;
        } else if let Some(end_range) = p.eat(is_star2) {
            if let Some(s) = text_start.take() {
                let content = p.span(s..pos);
                p.ops.push(Op::new_value(content));
            }
            let end_content = p.span(end_range);
            p.ops.push(Op::new_end(Node::Bold, end_content));
            found_end = true;
            break;
        } else {
            let before_pos = p.pos;
            let before_snap = p.ops.len();
            let matched = strikethrough(p) || italic(p);
            if matched {
                if let Some(s) = text_start.take() {
                    let content = p.span(s..before_pos);
                    p.ops.insert(before_snap, Op::new_value(content));
                }
            } else {
                text_start.get_or_insert(pos);
                p.next();
            }
        }
    }

    if found_end {
        return true;
    }

    p.pos = start;
    p.ops.truncate(snap);
    false
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, bold::bold, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "**~~happy~~ _path_**".into();
        assert!(bold(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Bold, p.span(0..1)),
                Op::new_start(Node::Strikethrough, p.span(1..2)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Strikethrough, p.span(3..4)),
                Op::new_value(p.span(4..5)),
                Op::new_start(Node::Italic, p.span(5..6)),
                Op::new_value(p.span(6..7)),
                Op::new_end(Node::Italic, p.span(7..8)),
                Op::new_end(Node::Bold, p.span(8..9)),
            ]
        );
    }

    #[test]
    fn terminator() {
        let mut p: Parser = "**~~happy~~ _path_\n\n**".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!bold(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
            );
        });
    }

    #[test]
    fn end_of_input() {
        let mut p: Parser = "**~~happy~~ _path_".into();
        assert!(!bold(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        )
    }

    #[test]
    fn end_of_input_in_nested() {
        let mut p: Parser = "**~~happy _path_".into();
        assert!(!bold(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        );
    }

    #[test]
    fn text_before_strikethrough() {
        let mut p: Parser = "**text ~~happy~~ _path_**".into();
        assert!(bold(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Bold, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_start(Node::Strikethrough, p.span(2..3)),
                Op::new_value(p.span(3..4)),
                Op::new_end(Node::Strikethrough, p.span(4..5)),
                Op::new_value(p.span(5..6)),
                Op::new_start(Node::Italic, p.span(6..7)),
                Op::new_value(p.span(7..8)),
                Op::new_end(Node::Italic, p.span(8..9)),
                Op::new_end(Node::Bold, p.span(9..10)),
            ]
        );
    }

    #[test]
    fn unclosed_italic() {
        let mut p: Parser = "**_path**".into();
        assert!(bold(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Bold, p.span(0..1)),
                Op::new_value(p.span(1..3)),
                Op::new_end(Node::Bold, p.span(3..4)),
            ]
        );
    }
}
