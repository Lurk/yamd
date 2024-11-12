use std::ops::Range;

use crate::{
    lexer::TokenKind,
    nodes::{Collapsible, YamdNodes},
};

use super::{yamd, Parser};

pub(crate) fn collapsible(p: &mut Parser) -> Option<Collapsible> {
    let start = p.pos();
    p.next_token();
    let mut title: Option<Range<usize>> = None;
    let mut nodes: Option<Vec<YamdNodes>> = None;

    while let Some((t, _)) = p.peek() {
        match t.kind {
            TokenKind::Space if title.is_none() => {
                if let Some((start, end)) = p.advance_until(|t| t.position.column == 0, true) {
                    title.replace(start + 1..end - 1);
                } else {
                    break;
                }
            }
            TokenKind::CollapsibleEnd if nodes.is_some() => {
                p.next_token();
                return Some(Collapsible::new(
                    p.range_to_string(title.expect("title to be initialized")),
                    nodes.expect("nodes to be initialized"),
                ));
            }
            _ if title.is_some() && nodes.is_none() => {
                nodes.replace(yamd(p, |t| t.kind == TokenKind::CollapsibleEnd).body);
            }
            _ => {
                break;
            }
        }
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
        nodes::{Collapsible, Heading, Image, Paragraph},
        parser::{collapsible, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n%}");
        assert_eq!(
            collapsible(&mut p),
            Some(Collapsible::new(
                "Title",
                vec![
                    Heading::new(1, vec![String::from("Heading").into()]).into(),
                    Paragraph::new(vec![String::from("text").into()]).into(),
                    Collapsible::new("nested", vec![Image::new("a", "u").into()]).into()
                ]
            ))
        );
    }

    #[test]
    fn no_title() {
        let mut p = Parser::new("{%\ntext%}");
        assert_eq!(collapsible(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "{%", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn parse_empty() {
        let mut p = Parser::new("{% Title\n\n%}");
        assert_eq!(collapsible(&mut p), Some(Collapsible::new("Title", vec![])));
    }

    #[test]
    fn no_end_token() {
        let mut p = Parser::new("{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n");
        assert_eq!(collapsible(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "{%", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn just_heading() {
        let mut p = Parser::new("{% Title\n# Heading\n%}");
        assert_eq!(
            collapsible(&mut p),
            Some(Collapsible::new(
                "Title",
                vec![Heading::new(1, vec![String::from("Heading").into()]).into(),]
            ))
        );
    }

    #[test]
    fn only_two_tokens() {
        let mut p = Parser::new("{% ");
        assert_eq!(collapsible(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "{%", Position::default()),
                0
            ))
        );
    }
}
