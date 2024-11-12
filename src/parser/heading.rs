use crate::{
    lexer::{Token, TokenKind},
    nodes::Heading,
};

use super::{anchor, Parser};

pub(crate) fn heading<Callback>(p: &mut Parser<'_>, new_line_check: Callback) -> Option<Heading>
where
    Callback: Fn(&Token) -> bool,
{
    let start = p.pos();
    let mut text_start: Option<usize> = None;
    let mut end_modifier = 0;

    let mut heading = Heading::new(
        p.next_token()
            .expect("to have token")
            .slice
            .len()
            .try_into()
            .expect("to be < 7"),
        vec![],
    );

    if let Some(t) = p.next_token() {
        if t.kind != TokenKind::Space {
            p.backtrack(start);
            p.flip_to_literal_at(start);
            return None;
        }
    }

    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => break,
            TokenKind::LeftSquareBracket => {
                if let Some(a) = anchor(p) {
                    if let Some(start) = text_start.take() {
                        heading.body.push(p.range_to_string(start..pos).into());
                    }
                    heading.body.push(a.into());
                } else {
                    text_start.get_or_insert(pos);
                    p.next_token();
                }
            }
            _ if t.position.column == 0 && new_line_check(t) => {
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
        heading
            .body
            .push(p.range_to_string(start..p.pos() - end_modifier).into());
    }

    if !heading.body.is_empty() {
        return Some(heading);
    }

    p.backtrack(start);
    p.flip_to_literal_at(start);

    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::{Anchor, Heading},
        parser::{heading, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("## heading [a](u) text");
        assert_eq!(
            heading(&mut p, |_| false),
            Some(Heading::new(
                2,
                vec![
                    String::from("heading ").into(),
                    Anchor::new("a", "u").into(),
                    String::from(" text").into()
                ]
            ))
        );
    }

    #[test]
    fn start_with_anchor() {
        let mut p = Parser::new("## [a](u) heading");
        assert_eq!(
            heading(&mut p, |_| false),
            Some(Heading::new(
                2,
                vec![
                    Anchor::new("a", "u").into(),
                    String::from(" heading").into(),
                ]
            ))
        );
    }

    #[test]
    fn broken_anchor() {
        let mut p = Parser::new("## heading [a](u text");
        assert_eq!(
            heading(&mut p, |_| false),
            Some(Heading::new(
                2,
                vec![String::from("heading [a](u text").into(),]
            ))
        );
    }

    #[test]
    fn with_terminator() {
        let mut p = Parser::new("## heading\n\ntext");
        assert_eq!(
            heading(&mut p, |_| false),
            Some(Heading::new(2, vec![String::from("heading").into()]))
        );
    }

    #[test]
    fn have_no_space_before_text() {
        let mut p = Parser::new("##heading\n\ntext");
        assert_eq!(heading(&mut p, |_| false), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "##", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn new_line_check() {
        let mut p = Parser::new("## heading [a](u) text\n ");
        assert_eq!(
            heading(&mut p, |t| t.kind == TokenKind::Space),
            Some(Heading::new(
                2,
                vec![
                    String::from("heading ").into(),
                    Anchor::new("a", "u").into(),
                    String::from(" text").into()
                ]
            ))
        );
    }

    #[test]
    fn only_one_token() {
        let mut p = Parser::new("##");
        assert_eq!(heading(&mut p, |_| false), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "##", Position::default()),
                0
            ))
        );
    }
}
