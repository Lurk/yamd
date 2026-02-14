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

fn icon(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let start_token = eat_seq!(p, is_one_bang, is_space)?;

    let Some((body, end_token)) = p.advance_until(is_eol) else {
        p.replace_position(start);
        return None;
    };

    Some(vec![
        Op::new_start(Node::Icon, start_token),
        Op::new_value(body),
        Op::new_end(Node::Icon, end_token),
    ])
}

pub fn highlight(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();

    let start_token = eat_seq!(p, is_two_bangs, |t: &Token| is_space(t) || is_eol(t))?;

    let skip_title = start_token.last().is_some_and(eol);

    let mut ops = vec![Op::new_start(Node::Highlight, start_token)];

    if !skip_title && let Some(heading) = modifier(p) {
        ops.extend(heading);
    }

    {
        let _g = p.push_eof(StopCondition::Terminator);
        if let Some(icon_ops) = icon(p) {
            ops.extend(icon_ops);
        }
    }

    if !p.at(|t: &Token| t.position.column == 0) {
        p.replace_position(start);
        return None;
    }

    while !p.at_eof() {
        if let Some(token) = p.eat(is_two_bangs) {
            ops.push(Op::new_end(Node::Highlight, token));
            return Some(ops);
        } else if let Some(token) = p.eat(is_terminator) {
            ops.push(Op::new_value(token));
        }
        let _g1 = p.push_eof(StopCondition::HighlightEnd);
        let _g2 = p.push_eof(StopCondition::Terminator);
        ops.extend(paragraph(p));
    }

    p.replace_position(start);
    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, highlight::highlight, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p = "!! Title\n! Icon\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p),
            Some(vec![
                Op::new_start(Node::Highlight, p.slice(0..2)), // !!
                Op::new_start(Node::Modifier, &[]),            //
                Op::new_value(p.slice(2..3)),                  // Title
                Op::new_end(Node::Modifier, p.slice(3..4)),    // \n
                Op::new_start(Node::Icon, p.slice(4..6)),      // !
                Op::new_value(p.slice(6..7)),                  // Icon
                Op::new_end(Node::Icon, p.slice(7..8)),        // \n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_start(Node::Italic, p.slice(8..9)),    // _
                Op::new_value(p.slice(9..10)),                 // i
                Op::new_end(Node::Italic, p.slice(10..11)),    // _
                Op::new_value(p.slice(11..12)),                // ' '
                Op::new_start(Node::Bold, p.slice(12..13)),    // **
                Op::new_value(p.slice(13..14)),                // b
                Op::new_end(Node::Bold, p.slice(14..15)),      // **
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_value(p.slice(15..16)),                // \n\n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_value(p.slice(16..17)),                // t
                Op::new_start(Node::Strikethrough, p.slice(17..18)), // ~~
                Op::new_value(p.slice(18..19)),                // s
                Op::new_end(Node::Strikethrough, p.slice(19..20)), // ~~
                Op::new_value(p.slice(20..22)),                // t\n
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_end(Node::Highlight, p.slice(22..23)), // !!
            ])
        );
    }

    #[test]
    fn no_title() {
        let p = "!!\n! Icon\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p),
            Some(vec![
                Op::new_start(Node::Highlight, p.slice(0..2)), // !!\n
                Op::new_start(Node::Icon, p.slice(2..4)),      // !
                Op::new_value(p.slice(4..5)),                  // Icon
                Op::new_end(Node::Icon, p.slice(5..6)),        // \n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_start(Node::Italic, p.slice(6..7)),    // _
                Op::new_value(p.slice(7..8)),                  // i
                Op::new_end(Node::Italic, p.slice(8..9)),      // _
                Op::new_value(p.slice(9..10)),                 // ' '
                Op::new_start(Node::Bold, p.slice(10..11)),    // **
                Op::new_value(p.slice(11..12)),                // b
                Op::new_end(Node::Bold, p.slice(12..13)),      // **
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_value(p.slice(13..14)),                // \n\n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_value(p.slice(14..15)),                // t
                Op::new_start(Node::Strikethrough, p.slice(15..16)), // ~~
                Op::new_value(p.slice(16..17)),                // s
                Op::new_end(Node::Strikethrough, p.slice(17..18)), // ~~
                Op::new_value(p.slice(18..20)),                // t\n
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_end(Node::Highlight, p.slice(20..21)), // !!
            ])
        )
    }

    #[test]
    fn no_icon() {
        let p = "!! Title\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p),
            Some(vec![
                Op::new_start(Node::Highlight, p.slice(0..2)), // !!
                Op::new_start(Node::Modifier, &[]),            //
                Op::new_value(p.slice(2..3)),                  // Title
                Op::new_end(Node::Modifier, p.slice(3..4)),    // \n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_start(Node::Italic, p.slice(4..5)),    // _
                Op::new_value(p.slice(5..6)),                  // i
                Op::new_end(Node::Italic, p.slice(6..7)),      // _
                Op::new_value(p.slice(7..8)),                  // ' '
                Op::new_start(Node::Bold, p.slice(8..9)),      // **
                Op::new_value(p.slice(9..10)),                 // b
                Op::new_end(Node::Bold, p.slice(10..11)),      // **
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_value(p.slice(11..12)),                // \n\n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_value(p.slice(12..13)),                // t
                Op::new_start(Node::Strikethrough, p.slice(13..14)), // ~~
                Op::new_value(p.slice(14..15)),                // s
                Op::new_end(Node::Strikethrough, p.slice(15..16)), // ~~
                Op::new_value(p.slice(16..18)),                // t\n
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_end(Node::Highlight, p.slice(18..19)), // !!
            ])
        )
    }

    #[test]
    fn no_closing_token() {
        let p = "!! Title\n_i_ **b**\n\nt~~s~~t!!".into();
        assert_eq!(highlight(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
        );
    }

    #[test]
    fn no_space_between_start_and_title() {
        let p = "!!Title\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(highlight(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
        );
    }

    #[test]
    fn terminator_in_title() {
        let p: Parser = "!! Title\n\n_i_ **b**\n\nt~~s~~t\n!!".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(highlight(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
        );
    }

    #[test]
    fn terminator_in_icon() {
        let p = "!!\n! icon\n\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p),
            Some(vec![
                Op::new_start(Node::Highlight, p.slice(0..2)), // !!\n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_value(p.slice(2..5)),                  // ! icon
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_value(p.slice(5..6)),                  // \n\n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_start(Node::Italic, p.slice(6..7)),    // _
                Op::new_value(p.slice(7..8)),                  // i
                Op::new_end(Node::Italic, p.slice(8..9)),      // _
                Op::new_value(p.slice(9..10)),                 // ' '
                Op::new_start(Node::Bold, p.slice(10..11)),    // **
                Op::new_value(p.slice(11..12)),                // b
                Op::new_end(Node::Bold, p.slice(12..13)),      // **
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_value(p.slice(13..14)),                // \n\n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_value(p.slice(14..15)),                // t
                Op::new_start(Node::Strikethrough, p.slice(15..16)), // ~~
                Op::new_value(p.slice(16..17)),                // s
                Op::new_end(Node::Strikethrough, p.slice(17..18)), // ~~
                Op::new_value(p.slice(18..20)),                // t\n
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_end(Node::Highlight, p.slice(20..21)), // !!
            ])
        );
    }

    #[test]
    fn no_space_before_icon() {
        let p = "!! Title\n!Icon\n!!".into();
        assert_eq!(
            highlight(&p),
            Some(vec![
                Op::new_start(Node::Highlight, p.slice(0..2)), // !!
                Op::new_start(Node::Modifier, &[]),            //
                Op::new_value(p.slice(2..3)),                  // Title
                Op::new_end(Node::Modifier, p.slice(3..4)),    // \n
                Op::new_start(Node::Paragraph, &[]),           //
                Op::new_value(p.slice(4..7)),                  // !Icon\n
                Op::new_end(Node::Paragraph, &[]),             //
                Op::new_end(Node::Highlight, p.slice(7..8)),   // !!
            ])
        );
    }

    #[test]
    fn only_one_token() {
        let p = "!!".into();
        assert_eq!(highlight(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default())))
        );
    }
}
