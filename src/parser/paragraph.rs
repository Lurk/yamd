use crate::{
    lexer::{Token, TokenKind},
    nodes::Paragraph,
};

use super::{anchor, bold, code_span, italic, strikethrough, Parser};

pub(crate) fn paragraph<Callback>(p: &mut Parser<'_>, new_line_check: Callback) -> Option<Paragraph>
where
    Callback: Fn(&Token) -> bool,
{
    let start = p.pos();
    let mut paragraph = Paragraph::default();
    let mut text_start: Option<usize> = None;
    let mut end_modifier = 0;

    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => break,
            TokenKind::Star if t.slice.len() == 2 => {
                if let Some(n) = bold(p) {
                    if let Some(start) = text_start.take() {
                        paragraph.body.push(p.range_to_string(start..pos).into());
                    }

                    paragraph.body.push(n.into());
                }
            }
            TokenKind::Underscore if t.slice.len() == 1 => {
                if let Some(n) = italic(p) {
                    if let Some(start) = text_start.take() {
                        paragraph.body.push(p.range_to_string(start..pos).into());
                    }

                    paragraph.body.push(n.into());
                }
            }
            TokenKind::Tilde if t.slice.len() == 2 => {
                if let Some(n) = strikethrough(p) {
                    if let Some(start) = text_start.take() {
                        paragraph.body.push(p.range_to_string(start..pos).into());
                    }

                    paragraph.body.push(n.into());
                }
            }
            TokenKind::LeftSquareBracket => {
                if let Some(n) = anchor(p) {
                    if let Some(start) = text_start.take() {
                        paragraph.body.push(p.range_to_string(start..pos).into());
                    }

                    paragraph.body.push(n.into());
                }
            }
            TokenKind::Backtick if t.slice.len() == 1 => {
                if let Some(n) = code_span(p) {
                    if let Some(start) = text_start.take() {
                        paragraph.body.push(p.range_to_string(start..pos).into());
                    }

                    paragraph.body.push(n.into());
                }
            }
            _ if pos != start && t.position.column == 0 && new_line_check(t) => {
                end_modifier = 1;
                text_start.take_if(|start| pos - *start < 2);
                break;
            }
            _ => {
                text_start.get_or_insert(pos);
                p.next_token();
            }
        }
    }

    if let Some(start) = text_start.take() {
        paragraph
            .body
            .push(p.range_to_string(start..p.pos() - end_modifier).into());
    }

    if end_modifier != 0 && paragraph.body.is_empty() {
        return None;
    }

    Some(paragraph)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::{Anchor, Bold, CodeSpan, Italic, Paragraph, Strikethrough},
        parser::{paragraph, Parser},
    };

    #[test]
    pub fn terminated() {
        let mut p = Parser::new("**b** _i_ ~~s~~ [a](u) `c` \n\n");
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
            ]))
        )
    }

    #[test]
    pub fn unterminated() {
        let mut p = Parser::new("_i_ ~~s~~ **b**[a](u) `c` ");
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
            paragraph(&mut p, |t| t.kind == TokenKind::ColapsibleEnd),
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
            paragraph(&mut p, |t| t.kind == TokenKind::ColapsibleEnd),
            None
        );
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(
                    TokenKind::ColapsibleEnd,
                    "%}",
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
}
