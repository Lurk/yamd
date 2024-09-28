use crate::{lexer::TokenKind, nodes::Code};

use super::Parser;

pub(crate) fn code(p: &mut Parser<'_>) -> Option<Code> {
    let start_pos = p.pos();
    let lang = p
        .advance_until_terminated(|t| t.kind == TokenKind::Eol)
        .map(|(_, end)| end)?;

    let Some((start, end)) = p.advance_until(
        |t| t.kind == TokenKind::Backtick && t.position.column == 0 && t.slice.len() == 3,
        true,
    ) else {
        p.move_to(start_pos);
        p.flip_to_literal_at(start_pos);
        return None;
    };

    p.next_token();
    Some(Code::new(
        p.range_to_string(start_pos + 1..lang),
        p.range_to_string(start..end - 1),
    ))
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::Code,
        parser::Parser,
    };

    use super::code;
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let mut p = Parser::new("```rust\nprintln!(\"hello\");\n```");
        assert_eq!(
            code(&mut p),
            Some(Code::new("rust", "println!(\"hello\");"))
        );
    }

    #[test]
    fn eol_before_lang() {
        let mut p = Parser::new("```\nprintln!(\"hello\");\n```");
        assert_eq!(code(&mut p), Some(Code::new("", "println!(\"hello\");")));
    }

    #[test]
    fn terminator_before_lang() {
        let mut p = Parser::new("```\n\nprintln!(\"hello\");\n```");
        assert_eq!(code(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "```", Position::default()),
                0
            ))
        )
    }

    #[test]
    fn do_not_have_closing_token() {
        let mut p = Parser::new("```\nprintln!(\"hello\");\n``");
        assert_eq!(code(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "```", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn terminator_in_the_middle_and_do_not_have_closing_token() {
        let mut p = Parser::new("```\nprintln!(\"hello\");\n\n\n``");
        assert_eq!(code(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "```", Position::default()),
                0
            ))
        );
    }
}
