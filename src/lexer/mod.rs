//! # Lexer module for YAMD
//! This module provides a lexer for the YAMD format. It tokenizes the input string into various
//! tokens such as literals, EOLs, and special characters.

mod token;

use std::{char, collections::VecDeque, iter::Peekable, str::CharIndices};

pub use token::{Position, Token, TokenKind};

/// # Lexer for YAMD.
///
/// ## Usage:
///
/// ```rust
/// use yamd::lexer::Lexer;
/// let input = "hello world";
/// let lexer = Lexer::new(input);
/// for range in lexer.map(|t|t.range){
///     print!("{}", &input[range]);
/// }
/// ```
pub struct Lexer<'input> {
    literal_start: Option<Position>,
    len: usize,
    escaped: bool,
    position: Position,
    iter: Peekable<CharIndices<'input>>,
    queue: VecDeque<Token>,
    token: Option<Token>,
}

impl<'input> Lexer<'input> {
    /// Creates a new lexer instance.
    pub fn new(input: &'input str) -> Self {
        Self {
            position: Position::default(),
            len: input.len(),
            iter: input.char_indices().peekable(),
            literal_start: None,
            escaped: false,
            queue: VecDeque::with_capacity(2),
            token: None,
        }
    }

    fn emit_literal_if_started(&mut self, end_byte_index: usize) {
        if let Some(start_position) = self.literal_start.take() {
            if let Some(token) = self.token.replace(Token {
                kind: TokenKind::Literal,
                range: start_position.byte_index..end_byte_index,
                position: start_position,
                escaped: self.escaped,
            }) {
                self.queue.push_back(token);
            }
            self.escaped = false;
        }
    }

    fn eol(&mut self, position: Position, len_in_bytes: usize) {
        self.emit_literal_if_started(position.byte_index);
        self.position.row += 1;
        self.position.column = 0;
        let Some(t) = self
            .token
            .replace(self.to_token(TokenKind::Eol, position, len_in_bytes))
        else {
            return;
        };
        if t.kind == TokenKind::Eol {
            self.token.replace(self.to_token(
                TokenKind::Terminator,
                t.position,
                t.range.len() + len_in_bytes,
            ));
            return;
        }
        self.queue.push_back(t);
    }

    fn emit(&mut self, token: Token) {
        self.emit_literal_if_started(token.position.byte_index);
        if let Some(l) = self.token.replace(token) {
            self.queue.push_back(l);
        }
    }

    fn next_is(&mut self, char: char) -> bool {
        let Some((_, next_char)) = self.iter.peek() else {
            return false;
        };
        if *next_char == char {
            self.next_char(false);
            return true;
        }
        false
    }

    fn next_char(&mut self, escaped: bool) -> Option<(Position, char)> {
        if let Some((byte_offset, char)) = self.iter.next() {
            self.position.byte_index = byte_offset;
            let res = Some((self.position.clone(), char));
            if char != '\\' || escaped {
                self.position.column += 1;
            }
            return res;
        }
        None
    }

    fn to_token(&self, kind: TokenKind, position: Position, len_in_bytes: usize) -> Token {
        Token::new(
            kind,
            position.byte_index..position.byte_index + len_in_bytes,
            position,
        )
    }

    fn take_while(&mut self, c: char, kind: TokenKind, start: Position) {
        while self.next_is(c) {}
        self.emit(Token::new(
            kind,
            start.byte_index..self.position.byte_index + 1,
            start,
        ))
    }

    fn parse(&mut self, position: Position, char: char) {
        match char {
            '\n' => self.eol(position, 1),
            '\r' if self.next_is('\n') => self.eol(position, 2),
            '{' if self.next_is('%') => {
                self.emit(self.to_token(TokenKind::CollapsibleStart, position, 2))
            }
            '%' if self.next_is('}') => {
                self.emit(self.to_token(TokenKind::CollapsibleEnd, position, 2))
            }
            '\\' => {
                self.emit_literal_if_started(position.byte_index);
                if let Some((pos, _)) = self.next_char(true) {
                    self.escaped = true;
                    self.literal_start.get_or_insert(pos);
                }
            }
            '~' => self.take_while('~', TokenKind::Tilde, position),
            '*' => self.take_while('*', TokenKind::Star, position),
            '}' => self.take_while('}', TokenKind::RightCurlyBrace, position),
            '{' => self.take_while('{', TokenKind::LeftCurlyBrace, position),
            ' ' if self.literal_start.is_none() => self.take_while(' ', TokenKind::Space, position),
            '-' => self.take_while('-', TokenKind::Minus, position),
            '#' => self.take_while('#', TokenKind::Hash, position),
            '>' => self.take_while('>', TokenKind::GreaterThan, position),
            '!' => self.take_while('!', TokenKind::Bang, position),
            '`' => self.take_while('`', TokenKind::Backtick, position),
            '+' => self.take_while('+', TokenKind::Plus, position),
            '[' => self.emit(self.to_token(TokenKind::LeftSquareBracket, position, 1)),
            ']' => self.emit(self.to_token(TokenKind::RightSquareBracket, position, 1)),
            '(' => self.emit(self.to_token(TokenKind::LeftParenthesis, position, 1)),
            ')' => self.emit(self.to_token(TokenKind::RightParenthesis, position, 1)),
            '_' => self.emit(self.to_token(TokenKind::Underscore, position, 1)),
            '|' => self.emit(self.to_token(TokenKind::Pipe, position, 1)),
            _ => {
                self.literal_start.get_or_insert(position);
            }
        }
    }

    fn advance(&mut self) {
        while self.queue.is_empty() {
            if let Some((position, char)) = self.next_char(false) {
                self.parse(position, char);
            } else {
                self.position.byte_index = self.len;
                self.emit_literal_if_started(self.position.byte_index);
                if let Some(token) = self.token.take() {
                    self.queue.push_back(token)
                }
                return;
            }
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance();
        self.queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::lexer::{Lexer, Position, Token, TokenKind};

    #[test]
    fn left_square_bracket() {
        assert_eq!(
            Lexer::new("[").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::LeftSquareBracket,
                0..1,
                Position::default()
            )],
        );
    }

    #[test]
    fn double_bang_with_left_square_bracket_afterwards() {
        assert_eq!(
            Lexer::new("!![").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Bang, 0..2, Position::default()),
                Token::new(
                    TokenKind::LeftSquareBracket,
                    2..3,
                    Position {
                        byte_index: 2,
                        column: 2,
                        row: 0,
                    }
                )
            ]
        )
    }

    #[test]
    fn double_bang_with_blob_afterwards() {
        assert_eq!(
            Lexer::new("!!g").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Bang, 0..2, Position::default()),
                Token::new(
                    TokenKind::Literal,
                    2..3,
                    Position {
                        byte_index: 2,
                        column: 2,
                        row: 0,
                    }
                )
            ]
        )
    }

    #[test]
    fn escaped_bang() {
        assert_eq!(
            Lexer::new("\\![").collect::<Vec<_>>(),
            vec![
                Token {
                    kind: TokenKind::Literal,
                    range: 1..2,
                    position: Position {
                        byte_index: 1,
                        column: 0,
                        row: 0
                    },
                    escaped: true
                },
                Token::new(
                    TokenKind::LeftSquareBracket,
                    2..3,
                    Position {
                        byte_index: 2,
                        column: 1,
                        row: 0
                    }
                )
            ]
        )
    }

    #[test]
    fn bang_open_brace() {
        assert_eq!(
            Lexer::new("![").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Bang, 0..1, Position::default()),
                Token::new(
                    TokenKind::LeftSquareBracket,
                    1..2,
                    Position {
                        byte_index: 1,
                        column: 1,
                        row: 0
                    }
                )
            ]
        )
    }

    #[test]
    fn triple_bang() {
        assert_eq!(
            Lexer::new("!!!").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Bang, 0..3, Position::default())]
        );
    }

    #[test]
    fn hastag() {
        assert_eq!(
            Lexer::new("[#####").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default()),
                Token::new(
                    TokenKind::Hash,
                    1..6,
                    Position {
                        byte_index: 1,
                        column: 1,
                        row: 0
                    }
                )
            ]
        );
    }

    #[test]
    fn space() {
        assert_eq!(
            Lexer::new("###\\   ").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Hash, 0..3, Position::default()),
                Token {
                    kind: TokenKind::Literal,
                    range: 4..7,
                    position: Position {
                        byte_index: 4,
                        column: 3,
                        row: 0,
                    },
                    escaped: true
                },
            ]
        );
    }

    #[test]
    fn escaped_space_compression() {
        assert_eq!(
            Lexer::new("\\ \\  ").collect::<Vec<_>>(),
            vec![
                Token {
                    kind: TokenKind::Literal,
                    range: 1..2,
                    position: Position {
                        byte_index: 1,
                        column: 0,
                        row: 0
                    },
                    escaped: true
                },
                Token {
                    kind: TokenKind::Literal,
                    range: 3..5,
                    position: Position {
                        byte_index: 3,
                        column: 1,
                        row: 0
                    },
                    escaped: true
                },
            ]
        );
    }

    #[test]
    fn eol() {
        assert_eq!(
            Lexer::new("\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Eol, 0..1, Position::default())]
        );
    }

    #[test]
    fn double_eol() {
        assert_eq!(
            Lexer::new("\n\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Terminator, 0..2, Position::default())]
        );
    }

    #[test]
    fn triple_eol() {
        assert_eq!(
            Lexer::new("\n\n\n").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Terminator, 0..2, Position::default()),
                Token::new(
                    TokenKind::Eol,
                    2..3,
                    Position {
                        byte_index: 2,
                        column: 0,
                        row: 2
                    }
                )
            ]
        );
    }

    #[test]
    fn windows_eol() {
        assert_eq!(
            Lexer::new("\r\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Eol, 0..2, Position::default())]
        );
    }

    #[test]
    fn windows_double_eol() {
        assert_eq!(
            Lexer::new("\r\n\r\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Terminator, 0..4, Position::default())]
        );
    }

    #[test]
    fn blob_that_ends_with_emoji() {
        assert_eq!(
            Lexer::new("hello blob😉").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Literal, 0..14, Position::default()),]
        )
    }

    #[test]
    fn correct_position_utf8() {
        assert_eq!(
            Lexer::new("blob😉-").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..8, Position::default()),
                Token::new(
                    TokenKind::Minus,
                    8..9,
                    Position {
                        byte_index: 8,
                        column: 5,
                        row: 0
                    }
                )
            ]
        )
    }

    #[test]
    fn double_open_brace() {
        assert_eq!(
            Lexer::new("{{").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::LeftCurlyBrace,
                0..2,
                Position::default()
            )]
        )
    }

    #[test]
    fn collapsible_start() {
        assert_eq!(
            Lexer::new("{%").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::CollapsibleStart,
                0..2,
                Position::default()
            )]
        )
    }

    #[test]
    fn collapsible_end() {
        assert_eq!(
            Lexer::new("{%").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::CollapsibleStart,
                0..2,
                Position::default()
            )]
        )
    }

    #[test]
    fn right_square_bracket() {
        assert_eq!(
            Lexer::new("]").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::RightSquareBracket,
                0..1,
                Position::default()
            )]
        )
    }

    #[test]
    fn open_parenthesis() {
        assert_eq!(
            Lexer::new("(").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::LeftParenthesis,
                0..1,
                Position::default()
            )]
        )
    }

    #[test]
    fn closing_parenthesis() {
        assert_eq!(
            Lexer::new(")").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::RightParenthesis,
                0..1,
                Position::default()
            )]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(Lexer::new("").collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn double_escape() {
        assert_eq!(
            Lexer::new("\\\\").collect::<Vec<_>>(),
            vec![Token {
                kind: TokenKind::Literal,
                range: 1..2,
                position: Position {
                    byte_index: 1,
                    column: 0,
                    row: 0
                },
                escaped: true
            },]
        )
    }

    #[test]
    fn escape_after_literal() {
        assert_eq!(
            Lexer::new("literal\\[[").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..7, Position::default()),
                Token {
                    kind: TokenKind::Literal,
                    range: 8..9,
                    position: Position {
                        byte_index: 8,
                        column: 7,
                        row: 0,
                    },
                    escaped: true
                },
                Token::new(
                    TokenKind::LeftSquareBracket,
                    9..10,
                    Position {
                        byte_index: 9,
                        column: 8,
                        row: 0,
                    },
                ),
            ]
        )
    }

    #[test]
    fn double_star() {
        assert_eq!(
            Lexer::new("**").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Star, 0..2, Position::default()),]
        )
    }

    #[test]
    fn quadruple_backtick() {
        assert_eq!(
            Lexer::new("````").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Backtick, 0..4, Position::default()),]
        );
    }

    #[test]
    fn underscore() {
        assert_eq!(
            Lexer::new("_").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Underscore, 0..1, Position::default())]
        )
    }

    #[test]
    fn plus() {
        assert_eq!(
            Lexer::new("+").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Plus, 0..1, Position::default())]
        )
    }

    #[test]
    fn minus() {
        assert_eq!(
            Lexer::new("-").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Minus, 0..1, Position::default())]
        )
    }

    #[test]
    fn multiple_minus() {
        assert_eq!(
            Lexer::new("----").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Minus, 0..4, Position::default())]
        )
    }

    #[test]
    fn greater_than() {
        assert_eq!(
            Lexer::new(">>> >>\n>").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::GreaterThan, 0..3, Position::default()),
                Token::new(
                    TokenKind::Space,
                    3..4,
                    Position {
                        byte_index: 3,
                        column: 3,
                        row: 0,
                    }
                ),
                Token::new(
                    TokenKind::GreaterThan,
                    4..6,
                    Position {
                        byte_index: 4,
                        column: 4,
                        row: 0,
                    }
                ),
                Token::new(
                    TokenKind::Eol,
                    6..7,
                    Position {
                        byte_index: 6,
                        column: 6,
                        row: 0,
                    }
                ),
                Token::new(
                    TokenKind::GreaterThan,
                    7..8,
                    Position {
                        byte_index: 7,
                        column: 0,
                        row: 1
                    }
                )
            ]
        )
    }

    #[test]
    fn backtick() {
        assert_eq!(
            Lexer::new("``").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Backtick, 0..2, Position::default(),),]
        )
    }

    #[test]
    fn strikethrough() {
        assert_eq!(
            Lexer::new("~~").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Tilde, 0..2, Position::default())]
        )
    }

    #[test]
    fn strikethrough_after_blob() {
        assert_eq!(
            Lexer::new("abc~~").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..3, Position::default()),
                Token::new(
                    TokenKind::Tilde,
                    3..5,
                    Position {
                        byte_index: 3,
                        column: 3,
                        row: 0
                    }
                )
            ]
        )
    }

    #[test]
    fn embed_end() {
        assert_eq!(
            Lexer::new("}}").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::RightCurlyBrace,
                0..2,
                Position::default()
            )]
        )
    }

    #[test]
    fn eol_after_literal() {
        assert_eq!(
            Lexer::new("r\n").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..1, Position::default()),
                Token::new(
                    TokenKind::Eol,
                    1..2,
                    Position {
                        byte_index: 1,
                        column: 1,
                        row: 0
                    }
                )
            ]
        )
    }
}
