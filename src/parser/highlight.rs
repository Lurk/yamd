use std::ops::Range;

use crate::{
    lexer::TokenKind,
    nodes::{Highlight, Paragraph},
};

use super::{paragraph, Parser};

#[derive(PartialEq)]
enum State {
    TitleCommit,
    Icon,
    IconCommit,
    Body,
}

pub(crate) fn highlight(p: &mut Parser) -> Option<Highlight> {
    let start = p.pos();

    p.next_token();

    let mut title: Option<Range<usize>> = None;
    let mut icon: Option<Range<usize>> = None;
    let mut nodes: Vec<Paragraph> = vec![];

    let mut state = State::TitleCommit;

    while let Some((t, _)) = p.peek() {
        match t.kind {
            TokenKind::Terminator if state != State::Body => break,
            TokenKind::Terminator => {
                p.next_token();
            }
            TokenKind::Space if state == State::TitleCommit && title.is_none() => {
                if let Some((start, end)) = p.advance_until(|t| t.kind == TokenKind::Eol) {
                    state = State::Icon;
                    title.replace(start + 1..end);
                } else {
                    break;
                }
            }
            TokenKind::Plus if t.slice.len() == 1 && state == State::Icon && icon.is_none() => {
                state = State::IconCommit;
                p.next_token();
            }
            TokenKind::Space if state == State::IconCommit && icon.is_none() => {
                if let Some((start, end)) = p.advance_until(|t| t.kind == TokenKind::Eol) {
                    state = State::Body;
                    icon.replace(start + 1..end);
                } else {
                    break;
                }
            }
            TokenKind::Eol if state == State::TitleCommit => {
                state = State::Icon;
                p.next_token();
            }
            TokenKind::Plus if t.slice.len() == 2 => {
                p.next_token();
                return Some(Highlight::new(
                    title.map(|r| p.range_to_string(r)),
                    icon.map(|r| p.range_to_string(r)),
                    nodes,
                ));
            }
            _ if state == State::Body || state == State::Icon => {
                state = State::Body;
                if let Some(n) = paragraph(p, |t| t.kind == TokenKind::Plus && t.slice.len() == 2) {
                    nodes.push(n);
                }
            }
            _ => break,
        }
    }

    p.move_to(start);
    p.flip_to_literal_at(start);

    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::{Bold, Highlight, Italic, Paragraph, Strikethrough},
        parser::{highlight, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("++ Title\n+ Icon\n_i_ **b**\n\nt~~s~~t\n++");
        assert_eq!(
            highlight(&mut p),
            Some(Highlight::new(
                Some("Title"),
                Some("Icon"),
                vec![
                    Paragraph::new(vec![
                        Italic::new("i").into(),
                        String::from(" ").into(),
                        Bold::new(vec![String::from("b").into()]).into()
                    ]),
                    Paragraph::new(vec![
                        String::from("t").into(),
                        Strikethrough::new("s").into(),
                        String::from("t").into()
                    ])
                ]
            ))
        )
    }

    #[test]
    fn no_title() {
        let mut p = Parser::new("++\n+ Icon\n_i_ **b**\n\nt~~s~~t\n++");
        assert_eq!(
            highlight(&mut p),
            Some(Highlight::new(
                None::<String>,
                Some("Icon"),
                vec![
                    Paragraph::new(vec![
                        Italic::new("i").into(),
                        String::from(" ").into(),
                        Bold::new(vec![String::from("b").into()]).into()
                    ]),
                    Paragraph::new(vec![
                        String::from("t").into(),
                        Strikethrough::new("s").into(),
                        String::from("t").into()
                    ])
                ]
            ))
        )
    }

    #[test]
    fn no_icon() {
        let mut p = Parser::new("++ Title\n_i_ **b**\n\nt~~s~~t\n++");
        assert_eq!(
            highlight(&mut p),
            Some(Highlight::new(
                Some("Title"),
                None::<String>,
                vec![
                    Paragraph::new(vec![
                        Italic::new("i").into(),
                        String::from(" ").into(),
                        Bold::new(vec![String::from("b").into()]).into()
                    ]),
                    Paragraph::new(vec![
                        String::from("t").into(),
                        Strikethrough::new("s").into(),
                        String::from("t").into()
                    ])
                ]
            ))
        )
    }

    #[test]
    fn no_closing_token() {
        let mut p = Parser::new("++ Title\n_i_ **b**\n\nt~~s~~t++");
        assert_eq!(highlight(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "++", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn no_space_between_start_and_title() {
        let mut p = Parser::new("++Title\n_i_ **b**\n\nt~~s~~t\n++");
        assert_eq!(highlight(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "++", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn terminator_in_title() {
        let mut p = Parser::new("++ Title\n\n_i_ **b**\n\nt~~s~~t\n++");
        assert_eq!(highlight(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "++", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn terminator_in_icon() {
        let mut p = Parser::new("++\n+ icon\n\n_i_ **b**\n\nt~~s~~t\n++");
        assert_eq!(highlight(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "++", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn no_space_before_icon() {
        let mut p = Parser::new("++ Title\n+Icon\n++");
        assert_eq!(highlight(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "++", Position::default()),
                0
            ))
        );
    }
}
