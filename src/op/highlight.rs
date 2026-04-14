use crate::{
    eat_seq,
    lexer::{Token, TokenKind},
    op::{
        Node, Op, Parser,
        modifier::modifier,
        paragraph::paragraph,
        parser::{StopCondition, eol},
    },
};

fn is_two_bangs(t: &Token) -> bool {
    t.kind == TokenKind::Bang && t.position.column == 0 && t.range.len() == 2
}

fn is_one_bang(t: &Token) -> bool {
    t.kind == TokenKind::Bang && t.position.column == 0 && t.range.len() == 1
}

fn is_space(t: &Token) -> bool {
    t.kind == TokenKind::Space && t.range.len() == 1
}

fn is_eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

fn is_terminator(t: &Token) -> bool {
    t.kind == TokenKind::Terminator
}

fn icon(p: &mut Parser) -> bool {
    let start = p.pos;
    let snap = p.ops.len();
    let Some(start_range) = eat_seq!(p, is_one_bang, is_space) else {
        return false;
    };

    let Some((body_range, end_range)) = p.advance_until(is_eol) else {
        p.pos = start;
        p.ops.truncate(snap);
        return false;
    };

    let start_content = p.span(start_range);
    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops.push(Op::new_start(Node::Icon, start_content));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Icon, end_content));
    true
}

pub fn highlight(p: &mut Parser) -> bool {
    let start = p.pos;
    let snap = p.ops.len();

    let Some(start_range) = eat_seq!(p, is_two_bangs, |t: &Token| is_space(t) || is_eol(t)) else {
        return false;
    };

    let skip_title = p.get(start_range.end - 1).is_some_and(eol);

    let start_content = p.span(start_range);
    p.ops.push(Op::new_start(Node::Highlight, start_content));

    if !skip_title {
        modifier(p);
    }

    p.with_eof(StopCondition::Terminator, |p| {
        icon(p);
    });

    if !p.at(|t: &Token| t.position.column == 0) {
        p.pos = start;
        p.ops.truncate(snap);
        return false;
    }

    while !p.at_eof() {
        let before = p.pos;

        if let Some(close_range) = p.eat(is_two_bangs) {
            let end_range = if let Some(eol_range) = p.eat(is_eol) {
                close_range.start..eol_range.end
            } else {
                close_range
            };
            let end_content = p.span(end_range);
            p.ops.push(Op::new_end(Node::Highlight, end_content));
            return true;
        } else if let Some(term_range) = p.eat(is_terminator) {
            let term_content = p.span(term_range);
            p.ops.push(Op::new_value(term_content));
        }
        p.with_eofs(
            &[StopCondition::HighlightEnd, StopCondition::Terminator],
            |p| {
                paragraph(p);
            },
        );

        debug_assert!(
            p.pos > before,
            "highlight loop made no progress at token {before}"
        );
    }

    p.pos = start;
    p.ops.truncate(snap);
    false
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Content, Node, Op, Parser, highlight::highlight, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "!! Title\n! Icon\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert!(highlight(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Highlight, p.span(0..2)),
                Op::new_start(Node::Modifier, Content::Span(0..0)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Modifier, p.span(3..4)),
                Op::new_start(Node::Icon, p.span(4..6)),
                Op::new_value(p.span(6..7)),
                Op::new_end(Node::Icon, p.span(7..8)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::Italic, p.span(8..9)),
                Op::new_value(p.span(9..10)),
                Op::new_end(Node::Italic, p.span(10..11)),
                Op::new_value(p.span(11..12)),
                Op::new_start(Node::Bold, p.span(12..13)),
                Op::new_value(p.span(13..14)),
                Op::new_end(Node::Bold, p.span(14..15)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(15..16)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(16..17)),
                Op::new_start(Node::Strikethrough, p.span(17..18)),
                Op::new_value(p.span(18..19)),
                Op::new_end(Node::Strikethrough, p.span(19..20)),
                Op::new_value(p.span(20..22)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Highlight, p.span(22..23)),
            ]
        );
    }

    #[test]
    fn no_title() {
        let mut p: Parser = "!!\n! Icon\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert!(highlight(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Highlight, p.span(0..2)),
                Op::new_start(Node::Icon, p.span(2..4)),
                Op::new_value(p.span(4..5)),
                Op::new_end(Node::Icon, p.span(5..6)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::Italic, p.span(6..7)),
                Op::new_value(p.span(7..8)),
                Op::new_end(Node::Italic, p.span(8..9)),
                Op::new_value(p.span(9..10)),
                Op::new_start(Node::Bold, p.span(10..11)),
                Op::new_value(p.span(11..12)),
                Op::new_end(Node::Bold, p.span(12..13)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(13..14)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(14..15)),
                Op::new_start(Node::Strikethrough, p.span(15..16)),
                Op::new_value(p.span(16..17)),
                Op::new_end(Node::Strikethrough, p.span(17..18)),
                Op::new_value(p.span(18..20)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Highlight, p.span(20..21)),
            ]
        )
    }

    #[test]
    fn no_icon() {
        let mut p: Parser = "!! Title\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert!(highlight(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Highlight, p.span(0..2)),
                Op::new_start(Node::Modifier, Content::Span(0..0)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Modifier, p.span(3..4)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::Italic, p.span(4..5)),
                Op::new_value(p.span(5..6)),
                Op::new_end(Node::Italic, p.span(6..7)),
                Op::new_value(p.span(7..8)),
                Op::new_start(Node::Bold, p.span(8..9)),
                Op::new_value(p.span(9..10)),
                Op::new_end(Node::Bold, p.span(10..11)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(11..12)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(12..13)),
                Op::new_start(Node::Strikethrough, p.span(13..14)),
                Op::new_value(p.span(14..15)),
                Op::new_end(Node::Strikethrough, p.span(15..16)),
                Op::new_value(p.span(16..18)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Highlight, p.span(18..19)),
            ]
        )
    }

    #[test]
    fn no_closing_token() {
        let mut p: Parser = "!! Title\n_i_ **b**\n\nt~~s~~t!!".into();
        assert!(!highlight(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
        );
    }

    #[test]
    fn no_space_between_start_and_title() {
        let mut p: Parser = "!!Title\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert!(!highlight(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
        );
    }

    #[test]
    fn terminator_in_title() {
        let mut p: Parser = "!! Title\n\n_i_ **b**\n\nt~~s~~t\n!!".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!highlight(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
            );
        });
    }

    #[test]
    fn terminator_in_icon() {
        let mut p: Parser = "!!\n! icon\n\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert!(highlight(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Highlight, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..5)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(5..6)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::Italic, p.span(6..7)),
                Op::new_value(p.span(7..8)),
                Op::new_end(Node::Italic, p.span(8..9)),
                Op::new_value(p.span(9..10)),
                Op::new_start(Node::Bold, p.span(10..11)),
                Op::new_value(p.span(11..12)),
                Op::new_end(Node::Bold, p.span(12..13)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(13..14)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(14..15)),
                Op::new_start(Node::Strikethrough, p.span(15..16)),
                Op::new_value(p.span(16..17)),
                Op::new_end(Node::Strikethrough, p.span(17..18)),
                Op::new_value(p.span(18..20)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Highlight, p.span(20..21)),
            ]
        );
    }

    #[test]
    fn no_space_before_icon() {
        let mut p: Parser = "!! Title\n!Icon\n!!".into();
        assert!(highlight(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Highlight, p.span(0..2)),
                Op::new_start(Node::Modifier, Content::Span(0..0)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Modifier, p.span(3..4)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(4..7)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Highlight, p.span(7..8)),
            ]
        );
    }

    #[test]
    fn only_one_token() {
        let mut p: Parser = "!!".into();
        assert!(!highlight(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
        );
    }
}
