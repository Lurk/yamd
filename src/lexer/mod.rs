//! # Lexer module for YAMD
//! This module provides a lexer for the YAMD format. It tokenizes the input string into various
//! tokens such as literals, EOLs, and special characters.

mod token;

pub use token::{Token, TokenKind};

const fn build_reserved_table() -> [bool; 256] {
    let mut table = [false; 256];
    let mut i = 0;
    let reserved: &[u8] = b"\n\r{%}~*-#>!`+[]()_|\\";
    while i < reserved.len() {
        table[reserved[i] as usize] = true;
        i += 1;
    }
    table
}

const RESERVED: [bool; 256] = build_reserved_table();

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
    input: &'input [u8],
    escaped: u32,
    pos: usize,
    at_line_start: bool,
}

impl<'input> Lexer<'input> {
    /// Creates a new lexer instance.
    pub fn new(input: &'input str) -> Self {
        Self {
            pos: 0,
            at_line_start: true,
            input: input.as_bytes(),
            escaped: 0,
        }
    }

    /// Length in bytes of the Eol sequence starting at `at`, if there is one.
    fn eol_len_at(&self, at: usize) -> Option<usize> {
        match *self.input.get(at)? {
            b'\n' => Some(1),
            b'\r' if self.input.get(at + 1) == Some(&b'\n') => Some(2),
            _ => None,
        }
    }

    fn eol(&mut self, byte_index: usize, is_line_start: bool, len_in_bytes: usize) -> Token {
        self.at_line_start = true;
        let mut end = byte_index + len_in_bytes;
        let mut kind = TokenKind::Eol;
        if let Some(next_len) = self.eol_len_at(end) {
            end += next_len;
            kind = TokenKind::Terminator;
        }
        self.pos = end;
        Token::new(kind, byte_index..end, is_line_start)
    }

    fn next_is(&mut self, byte: u8) -> bool {
        if self.input.get(self.pos) == Some(&byte) {
            self.next_byte();
            return true;
        }
        false
    }

    fn next_byte(&mut self) -> Option<(usize, u8)> {
        let byte_index = self.pos;
        let byte = *self.input.get(byte_index)?;
        self.pos = byte_index + 1;
        self.at_line_start = false;
        Some((byte_index, byte))
    }

    fn consume_literal_run(&mut self) {
        while self.pos < self.input.len() && !RESERVED[self.input[self.pos] as usize] {
            self.pos += 1;
        }
    }

    fn consume_escaped_byte(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
            self.escaped += 1;
        }
    }

    fn consume_literal(&mut self, start_byte_index: usize, is_line_start: bool) -> Token {
        loop {
            self.consume_literal_run();
            if self.input.get(self.pos) != Some(&b'\\') {
                break;
            }
            self.pos += 1;
            self.consume_escaped_byte();
        }
        let token = Token {
            kind: TokenKind::Literal,
            range: start_byte_index..self.pos,
            is_line_start,
            escaped: self.escaped,
        };
        self.escaped = 0;
        token
    }

    fn escape(&mut self, byte_index: usize, is_line_start: bool) -> Token {
        self.consume_escaped_byte();
        self.consume_literal(byte_index, is_line_start)
    }

    fn take_while(
        &mut self,
        byte: u8,
        kind: TokenKind,
        start_byte_index: usize,
        start_is_line_start: bool,
    ) -> Token {
        while self.next_is(byte) {}
        Token::new(kind, start_byte_index..self.pos, start_is_line_start)
    }

    fn parse(&mut self, byte_index: usize, is_line_start: bool, byte: u8) -> Token {
        match byte {
            b'\n' => self.eol(byte_index, is_line_start, 1),
            b'\r' if self.next_is(b'\n') => self.eol(byte_index, is_line_start, 2),
            b'{' if self.next_is(b'%') => Token::new(
                TokenKind::CollapsibleStart,
                byte_index..byte_index + 2,
                is_line_start,
            ),
            b'%' if self.next_is(b'}') => Token::new(
                TokenKind::CollapsibleEnd,
                byte_index..byte_index + 2,
                is_line_start,
            ),
            b'\\' => self.escape(byte_index, is_line_start),
            b'~' => self.take_while(b'~', TokenKind::Tilde, byte_index, is_line_start),
            b'*' => self.take_while(b'*', TokenKind::Star, byte_index, is_line_start),
            b'}' => self.take_while(b'}', TokenKind::RightCurlyBrace, byte_index, is_line_start),
            b'{' => self.take_while(b'{', TokenKind::LeftCurlyBrace, byte_index, is_line_start),
            b' ' => self.take_while(b' ', TokenKind::Space, byte_index, is_line_start),
            b'-' => self.take_while(b'-', TokenKind::Minus, byte_index, is_line_start),
            b'#' => self.take_while(b'#', TokenKind::Hash, byte_index, is_line_start),
            b'>' => self.take_while(b'>', TokenKind::GreaterThan, byte_index, is_line_start),
            b'!' => self.take_while(b'!', TokenKind::Bang, byte_index, is_line_start),
            b'`' => self.take_while(b'`', TokenKind::Backtick, byte_index, is_line_start),
            b'+' => self.take_while(b'+', TokenKind::Plus, byte_index, is_line_start),
            b'[' => Token::new(
                TokenKind::LeftSquareBracket,
                byte_index..byte_index + 1,
                is_line_start,
            ),
            b']' => Token::new(
                TokenKind::RightSquareBracket,
                byte_index..byte_index + 1,
                is_line_start,
            ),
            b'(' => Token::new(
                TokenKind::LeftParenthesis,
                byte_index..byte_index + 1,
                is_line_start,
            ),
            b')' => Token::new(
                TokenKind::RightParenthesis,
                byte_index..byte_index + 1,
                is_line_start,
            ),
            b'_' => Token::new(
                TokenKind::Underscore,
                byte_index..byte_index + 1,
                is_line_start,
            ),
            b'|' => Token::new(TokenKind::Pipe, byte_index..byte_index + 1, is_line_start),
            _ => self.consume_literal(byte_index, is_line_start),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let is_line_start = self.at_line_start;
        let (byte_index, byte) = self.next_byte()?;
        Some(self.parse(byte_index, is_line_start, byte))
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
    fn windows_eol_after_literal() {
        assert_eq!(
            Lexer::new("r\r\n").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, 0..1, true),
                Token::new(TokenKind::Eol, 1..3, false)
            ]
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

    #[test]
    fn reserved_table_covers_every_byte_parse_splits_a_literal_on() {
        for byte in 0u8..=127 {
            let input = format!("a{}a", byte as char);
            let tokens: Vec<_> = Lexer::new(&input).collect();
            let literal_stayed_whole =
                matches!(tokens.as_slice(), [t] if t.kind == TokenKind::Literal);
            if !literal_stayed_whole {
                assert!(
                    super::RESERVED[byte as usize],
                    "byte {byte:#x} ({:?}) splits a literal it appears inside of but is missing from RESERVED",
                    byte as char
                );
            }
        }
    }

    #[test]
    fn reserved_table_excludes_every_non_ascii_byte() {
        for byte in 128u8..=255 {
            assert!(
                !super::RESERVED[byte as usize],
                "byte {byte:#x} is non-ASCII but marked reserved"
            );
        }
    }
}
