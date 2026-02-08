use crate::{
    lexer::{Token, TokenKind},
    nodes::Paragraph,
};

use super::{anchor, bold, code_span, emphasis, italic, strikethrough, BranchBuilder, Parser};

pub(crate) fn paragraph<Callback>(p: &mut Parser<'_>, new_line_check: Callback) -> Option<Paragraph>
where
    Callback: Fn(&Token) -> bool,
{
    let start = p.pos();
    let mut builder = BranchBuilder::new();
    let mut end_modifier = 0;

    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => break,
            TokenKind::Star if t.range.len() == 2 => builder.push(bold(p), p, pos),
            TokenKind::Star if t.range.len() == 1 => builder.push(emphasis(p), p, pos),
            TokenKind::Underscore if t.range.len() == 1 => builder.push(italic(p), p, pos),
            TokenKind::Tilde if t.range.len() == 2 => builder.push(strikethrough(p), p, pos),
            TokenKind::LeftSquareBracket => builder.push(anchor(p), p, pos),
            TokenKind::Backtick if t.range.len() == 1 => builder.push(code_span(p), p, pos),
            _ if pos != start && t.position.column == 0 && new_line_check(t) => {
                end_modifier = 1;
                builder.clear_text_if_shorter_than(pos, 2);
                break;
            }
            _ => {
                builder.start_text(pos);
                p.next_token();
            }
        }
    }

    builder.consume_text(p, p.pos() - end_modifier);

    builder.build()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::{Anchor, Bold, CodeSpan, Emphasis, Italic, Paragraph, Strikethrough},
        parser::{paragraph, Parser},
    };

    #[test]
    pub fn terminated() {
        let mut p = Parser::new("**b** _i_ ~~s~~ [a](u) `c` *e* \n\n");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![
                Bold::new(vec![String::from("b").into()]).into(),
                String::from(" ").into(),
                Italic::new("i").into(),
                String::from(" ").into(),
                Strikethrough::new("s").into(),
                String::from(" ").into(),
                Anchor::new("a", "u").into(),
                String::from(" ").into(),
                CodeSpan::new("c").into(),
                String::from(" ").into(),
                Emphasis::new("e").into(),
                String::from(" ").into()
            ]))
        )
    }

    #[test]
    pub fn unterminated() {
        let mut p = Parser::new("_i_ ~~s~~ **b**[a](u) `c` *e* ");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![
                Italic::new("i").into(),
                String::from(" ").into(),
                Strikethrough::new("s").into(),
                String::from(" ").into(),
                Bold::new(vec![String::from("b").into()]).into(),
                Anchor::new("a", "u").into(),
                String::from(" ").into(),
                CodeSpan::new("c").into(),
                String::from(" ").into(),
                Emphasis::new("e").into(),
                String::from(" ").into()
            ]))
        )
    }

    #[test]
    pub fn fallback() {
        let mut p = Parser::new("_i_ ~~s~~ **b[a](u) `c` ");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![
                Italic::new("i").into(),
                String::from(" ").into(),
                Strikethrough::new("s").into(),
                String::from(" **b").into(),
                Anchor::new("a", "u").into(),
                String::from(" ").into(),
                CodeSpan::new("c").into(),
                String::from(" ").into(),
            ]))
        )
    }

    #[test]
    pub fn stop_cb() {
        let mut p = Parser::new("_i_ ~~s~~ **b[a](u) \n%} `c` ");
        assert_eq!(
            paragraph(&mut p, |t| t.kind == TokenKind::CollapsibleEnd),
            Some(Paragraph::new(vec![
                Italic::new("i").into(),
                String::from(" ").into(),
                Strikethrough::new("s").into(),
                String::from(" **b").into(),
                Anchor::new("a", "u").into(),
                String::from(" ").into()
            ]))
        )
    }

    #[test]
    pub fn stop_cb_empty() {
        let mut p = Parser::new("\n%} `c` ");
        assert_eq!(
            paragraph(&mut p, |t| t.kind == TokenKind::CollapsibleEnd),
            None
        );
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(
                    TokenKind::CollapsibleEnd,
                    1..3,
                    Position {
                        byte_index: 1,
                        column: 0,
                        row: 1,
                    }
                ),
                1
            ))
        );
    }

    #[test]
    fn eol_at_start() {
        let mut p = Parser::new("\nt");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![String::from("\nt").into()]))
        )
    }

    #[test]
    fn eol_after_node() {
        let mut p = Parser::new("~~s~~\nt");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![
                Strikethrough::new("s").into(),
                String::from("\nt").into()
            ]))
        )
    }

    #[test]
    fn not_closed_code_span() {
        let mut p = Parser::new("`");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![String::from("`").into()]))
        )
    }

    #[test]
    fn not_anchor() {
        let mut p = Parser::new("[]");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![String::from("[]").into()]))
        )
    }

    #[test]
    fn not_italic() {
        let mut p = Parser::new("_");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![String::from("_").into()]))
        )
    }

    #[test]
    fn not_strikethrough() {
        let mut p = Parser::new("~~");
        assert_eq!(
            paragraph(&mut p, |_| false),
            Some(Paragraph::new(vec![String::from("~~").into()]))
        )
    }
}
