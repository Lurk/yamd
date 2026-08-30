//! # Lexer module for YAMD
//! This module provides a lexer for the YAMD format. It tokenizes the input string into various
//! tokens such as literals, EOLs, and special characters.

mod token;

use std::{char, collections::VecDeque, iter::Peekable, str::CharIndices};

pub use token::{Token, TokenKind};

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
    literal_start: Option<(usize, bool)>,
    len: usize,
    escaped: u32,
    byte_index: usize,
    at_line_start: bool,
    iter: Peekable<CharIndices<'input>>,
    queue: VecDeque<Token>,
    token: Option<Token>,
}

impl<'input> Lexer<'input> {
    /// Creates a new lexer instance.
    pub fn new(input: &'input str) -> Self {
        Self {
            byte_index: 0,
            at_line_start: true,
            len: input.len(),
            iter: input.char_indices().peekable(),
            literal_start: None,
            escaped: 0,
            queue: VecDeque::with_capacity(2),
            token: None,
        }
    }

    fn emit_literal_if_started(&mut self, end_byte_index: usize) {
        if let Some((start_byte_index, is_line_start)) = self.literal_start.take() {
            if let Some(token) = self.token.replace(Token {
                kind: TokenKind::Literal,
                range: start_byte_index..end_byte_index,
                is_line_start,
                escaped: self.escaped,
            }) {
                self.queue.push_back(token);
            }
            self.escaped = 0;
        }
    }

    fn eol(&mut self, byte_index: usize, is_line_start: bool, len_in_bytes: usize) {
        self.emit_literal_if_started(byte_index);
        self.at_line_start = true;
        let Some(t) = self.token.replace(Token::new(
            TokenKind::Eol,
            byte_index..byte_index + len_in_bytes,
            is_line_start,
        )) else {
            return;
        };
        if t.kind == TokenKind::Eol {
            self.token.replace(Token::new(
                TokenKind::Terminator,
                t.range.start..t.range.start + t.range.len() + len_in_bytes,
                t.is_line_start,
            ));
            return;
        }
        self.queue.push_back(t);
    }

    fn emit(&mut self, token: Token) {
        self.emit_literal_if_started(token.range.start);
        if let Some(l) = self.token.replace(token) {
            self.queue.push_back(l);
        }
    }

    fn next_is(&mut self, char: char) -> bool {
        let Some((_, next_char)) = self.iter.peek() else {
            return false;
        };
        if *next_char == char {
            self.next_char();
            return true;
        }
        false
    }

    fn next_char(&mut self) -> Option<(usize, bool, char)> {
        if let Some((byte_offset, char)) = self.iter.next() {
            self.byte_index = byte_offset;
            let res = Some((self.byte_index, self.at_line_start, char));
            self.at_line_start = false;
            return res;
        }
        None
    }

    fn escape(&mut self, byte_index: usize, is_line_start: bool) {
        self.literal_start
            .get_or_insert((byte_index, is_line_start));
        if self.next_char().is_some() {
            self.escaped += 1;
        }
    }

    fn take_while(
        &mut self,
        c: char,
        kind: TokenKind,
        start_byte_index: usize,
        start_is_line_start: bool,
    ) {
        while self.next_is(c) {}
        self.emit(Token::new(
            kind,
            start_byte_index..self.byte_index + 1,
            start_is_line_start,
        ))
    }

    fn parse(&mut self, byte_index: usize, is_line_start: bool, char: char) {
        match char {
            '\n' => self.eol(byte_index, is_line_start, 1),
            '\r' if self.next_is('\n') => self.eol(byte_index, is_line_start, 2),
            '{' if self.next_is('%') => self.emit(Token::new(
                TokenKind::CollapsibleStart,
                byte_index..byte_index + 2,
                is_line_start,
            )),
            '%' if self.next_is('}') => self.emit(Token::new(
                TokenKind::CollapsibleEnd,
                byte_index..byte_index + 2,
                is_line_start,
            )),
            '\\' => self.escape(byte_index, is_line_start),
            '~' => self.take_while('~', TokenKind::Tilde, byte_index, is_line_start),
            '*' => self.take_while('*', TokenKind::Star, byte_index, is_line_start),
            '}' => self.take_while('}', TokenKind::RightCurlyBrace, byte_index, is_line_start),
            '{' => self.take_while('{', TokenKind::LeftCurlyBrace, byte_index, is_line_start),
            ' ' if self.literal_start.is_none() => {
                self.take_while(' ', TokenKind::Space, byte_index, is_line_start)
            }
            '-' => self.take_while('-', TokenKind::Minus, byte_index, is_line_start),
            '#' => self.take_while('#', TokenKind::Hash, byte_index, is_line_start),
            '>' => self.take_while('>', TokenKind::GreaterThan, byte_index, is_line_start),
            '!' => self.take_while('!', TokenKind::Bang, byte_index, is_line_start),
            '`' => self.take_while('`', TokenKind::Backtick, byte_index, is_line_start),
            '+' => self.take_while('+', TokenKind::Plus, byte_index, is_line_start),
            '[' => self.emit(Token::new(
                TokenKind::LeftSquareBracket,
                byte_index..byte_index + 1,
                is_line_start,
            )),
            ']' => self.emit(Token::new(
                TokenKind::RightSquareBracket,
                byte_index..byte_index + 1,
                is_line_start,
            )),
            '(' => self.emit(Token::new(
                TokenKind::LeftParenthesis,
                byte_index..byte_index + 1,
                is_line_start,
            )),
            ')' => self.emit(Token::new(
                TokenKind::RightParenthesis,
                byte_index..byte_index + 1,
                is_line_start,
            )),
            '_' => self.emit(Token::new(
                TokenKind::Underscore,
                byte_index..byte_index + 1,
                is_line_start,
            )),
            '|' => self.emit(Token::new(
                TokenKind::Pipe,
                byte_index..byte_index + 1,
                is_line_start,
            )),
            _ => {
                self.literal_start
                    .get_or_insert((byte_index, is_line_start));
            }
        }
    }

    fn advance(&mut self) {
        while self.queue.is_empty() {
            if let Some((byte_index, is_line_start, char)) = self.next_char() {
                self.parse(byte_index, is_line_start, char);
            } else {
                self.byte_index = self.len;
                self.emit_literal_if_started(self.byte_index);
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

    use crate::lexer::{Lexer, Token, TokenKind};

    #[test]
    fn left_square_bracket() {
        assert_eq!(
            Lexer::new("[").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::LeftSquareBracket, 0..1, true)],
        );
    }

    #[test]
    fn double_bang_with_left_square_bracket_afterwards() {
        assert_eq!(
            Lexer::new("!![").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Bang, 0..2, true),
                Token::new(TokenKind::LeftSquareBracket, 2..3, false)
            ]
        )
    }

    #[test]
    fn double_bang_with_blob_afterwards() {
        assert_eq!(
            Lexer::new("!!g").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Bang, 0..2, true),
                Token::new(TokenKind::Literal, 2..3, false)
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
                    range: 0..2,
                    is_line_start: true,
                    escaped: 1
                },
                Token::new(TokenKind::LeftSquareBracket, 2..3, false)
            ]
        )
    }

    #[test]
    fn bang_open_brace() {
        assert_eq!(
            Lexer::new("![").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Bang, 0..1, true),
                Token::new(TokenKind::LeftSquareBracket, 1..2, false)
            ]
        )
    }

    #[test]
    fn triple_bang() {
        assert_eq!(
            Lexer::new("!!!").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Bang, 0..3, true)]
        );
    }

    #[test]
    fn hastag() {
        assert_eq!(
            Lexer::new("[#####").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::LeftSquareBracket, 0..1, true),
                Token::new(TokenKind::Hash, 1..6, false)
            ]
        );
    }

    #[test]
    fn space() {
        assert_eq!(
            Lexer::new("###\\   ").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Hash, 0..3, true),
                Token {
                    kind: TokenKind::Literal,
                    range: 3..7,
                    is_line_start: false,
                    escaped: 1
                },
            ]
        );
    }

    #[test]
    fn escaped_space_compression() {
        assert_eq!(
            Lexer::new("\\ \\  ").collect::<Vec<_>>(),
            vec![Token {
                kind: TokenKind::Literal,
                range: 0..5,
                is_line_start: true,
                escaped: 2
            }]
        );
    }

    #[test]
    fn eol() {
        assert_eq!(
            Lexer::new("\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Eol, 0..1, true)]
        );
    }

    #[test]
    fn double_eol() {
        assert_eq!(
            Lexer::new("\n\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Terminator, 0..2, true)]
        );
    }

    #[test]
    fn triple_eol() {
        assert_eq!(
            Lexer::new("\n\n\n").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Terminator, 0..2, true),
                Token::new(TokenKind::Eol, 2..3, true)
            ]
        );
    }

    #[test]
    fn windows_eol() {
        assert_eq!(
            Lexer::new("\r\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Eol, 0..2, true)]
        );
    }

    #[test]
    fn windows_double_eol() {
        assert_eq!(
            Lexer::new("\r\n\r\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Terminator, 0..4, true)]
        );
    }

    #[test]
    fn blob_that_ends_with_emoji() {
        assert_eq!(
            Lexer::new("hello blob😉").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Literal, 0..14, true),]
        )
    }

    #[test]
    fn correct_position_utf8() {
        assert_eq!(
            Lexer::new("blob😉-").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..8, true),
                Token::new(TokenKind::Minus, 8..9, false)
            ]
        )
    }

    #[test]
    fn double_open_brace() {
        assert_eq!(
            Lexer::new("{{").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::LeftCurlyBrace, 0..2, true)]
        )
    }

    #[test]
    fn collapsible_start() {
        assert_eq!(
            Lexer::new("{%").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::CollapsibleStart, 0..2, true)]
        )
    }

    #[test]
    fn collapsible_end() {
        assert_eq!(
            Lexer::new("{%").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::CollapsibleStart, 0..2, true)]
        )
    }

    #[test]
    fn right_square_bracket() {
        assert_eq!(
            Lexer::new("]").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::RightSquareBracket, 0..1, true)]
        )
    }

    #[test]
    fn open_parenthesis() {
        assert_eq!(
            Lexer::new("(").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::LeftParenthesis, 0..1, true)]
        )
    }

    #[test]
    fn closing_parenthesis() {
        assert_eq!(
            Lexer::new(")").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::RightParenthesis, 0..1, true)]
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
                range: 0..2,
                is_line_start: true,
                escaped: 1
            },]
        )
    }

    #[test]
    fn escape_after_literal() {
        assert_eq!(
            Lexer::new("literal\\[[").collect::<Vec<_>>(),
            vec![
                Token {
                    kind: TokenKind::Literal,
                    range: 0..9,
                    is_line_start: true,
                    escaped: 1
                },
                Token::new(TokenKind::LeftSquareBracket, 9..10, false,),
            ]
        )
    }

    #[test]
    fn double_star() {
        assert_eq!(
            Lexer::new("**").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Star, 0..2, true),]
        )
    }

    #[test]
    fn quadruple_backtick() {
        assert_eq!(
            Lexer::new("````").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Backtick, 0..4, true),]
        );
    }

    #[test]
    fn underscore() {
        assert_eq!(
            Lexer::new("_").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Underscore, 0..1, true)]
        )
    }

    #[test]
    fn plus() {
        assert_eq!(
            Lexer::new("+").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Plus, 0..1, true)]
        )
    }

    #[test]
    fn minus() {
        assert_eq!(
            Lexer::new("-").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Minus, 0..1, true)]
        )
    }

    #[test]
    fn multiple_minus() {
        assert_eq!(
            Lexer::new("----").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Minus, 0..4, true)]
        )
    }

    #[test]
    fn greater_than() {
        assert_eq!(
            Lexer::new(">>> >>\n>").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::GreaterThan, 0..3, true),
                Token::new(TokenKind::Space, 3..4, false),
                Token::new(TokenKind::GreaterThan, 4..6, false),
                Token::new(TokenKind::Eol, 6..7, false),
                Token::new(TokenKind::GreaterThan, 7..8, true)
            ]
        )
    }

    #[test]
    fn backtick() {
        assert_eq!(
            Lexer::new("``").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Backtick, 0..2, true,),]
        )
    }

    #[test]
    fn strikethrough() {
        assert_eq!(
            Lexer::new("~~").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Tilde, 0..2, true)]
        )
    }

    #[test]
    fn strikethrough_after_blob() {
        assert_eq!(
            Lexer::new("abc~~").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..3, true),
                Token::new(TokenKind::Tilde, 3..5, false)
            ]
        )
    }

    #[test]
    fn embed_end() {
        assert_eq!(
            Lexer::new("}}").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::RightCurlyBrace, 0..2, true)]
        )
    }

    #[test]
    fn eol_after_literal() {
        assert_eq!(
            Lexer::new("r\n").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..1, true),
                Token::new(TokenKind::Eol, 1..2, false)
            ]
        )
    }

    #[test]
    fn dangling_backslash_at_eof() {
        assert_eq!(
            Lexer::new("\\").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Literal, 0..1, true)]
        );
    }
}
