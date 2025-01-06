/// # Lexer for YAMD.
///
/// ## Usage:
///
/// ```rust
/// use yamd::lexer::Lexer;
/// let lexer = Lexer::new("string");
/// for slice in lexer.map(|t|t.slice){
///     print!("{}", slice);
/// }
/// ```
mod token;

use std::{char, collections::VecDeque, iter::Peekable, str::CharIndices};

pub use token::{Position, Token, TokenKind};

pub struct Lexer<'input> {
    literal_start: Option<Position>,
    escaped: bool,
    position: Position,
    input: &'input str,
    iter: Peekable<CharIndices<'input>>,
    queue: VecDeque<Token<'input>>,
    token: Option<Token<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            position: Position::default(),
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
                slice: &self.input[start_position.byte_index..end_byte_index],
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
                t.slice.len() + len_in_bytes,
            ));
            return;
        }
        self.queue.push_back(t);
    }

    fn emit(&mut self, token: Token<'input>) {
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

    fn to_token(&self, kind: TokenKind, position: Position, len_in_bytes: usize) -> Token<'input> {
        Token::new(
            kind,
            &self.input[position.byte_index..position.byte_index + len_in_bytes],
            position,
        )
    }

    fn take_while(&mut self, c: char, kind: TokenKind, start: Position) {
        while self.next_is(c) {}
        self.emit(Token::new(
            kind,
            &self.input[start.byte_index..self.position.byte_index + 1],
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
            ' ' => self.take_while(' ', TokenKind::Space, position),
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
                self.position.byte_index = self.input.len();
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
    type Item = Token<'input>;

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
                "[",
                Position::default()
            )],
        );
    }

    #[test]
    fn double_bang_with_left_square_bracket_afterwards() {
        assert_eq!(
            Lexer::new("!![").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Bang, "!!", Position::default()),
                Token::new(
                    TokenKind::LeftSquareBracket,
                    "[",
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
                Token::new(TokenKind::Bang, "!!", Position::default()),
                Token::new(
                    TokenKind::Literal,
                    "g",
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
                    slice: "!",
                    position: Position {
                        byte_index: 1,
                        column: 0,
                        row: 0
                    },
                    escaped: true
                },
                Token::new(
                    TokenKind::LeftSquareBracket,
                    "[",
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
                Token::new(TokenKind::Bang, "!", Position::default()),
                Token::new(
                    TokenKind::LeftSquareBracket,
                    "[",
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
            vec![Token::new(TokenKind::Bang, "!!!", Position::default())]
        );
    }

    #[test]
    fn hastag() {
        assert_eq!(
            Lexer::new("[#####").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::LeftSquareBracket, "[", Position::default()),
                Token::new(
                    TokenKind::Hash,
                    "#####",
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
                Token::new(TokenKind::Hash, "###", Position::default()),
                Token {
                    kind: TokenKind::Literal,
                    slice: " ",
                    position: Position {
                        byte_index: 4,
                        column: 3,
                        row: 0,
                    },
                    escaped: true
                },
                Token::new(
                    TokenKind::Space,
                    "  ",
                    Position {
                        byte_index: 5,
                        column: 4,
                        row: 0
                    }
                )
            ]
        );
    }

    #[test]
    fn eol() {
        assert_eq!(
            Lexer::new("\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Eol, "\n", Position::default())]
        );
    }

    #[test]
    fn double_eol() {
        assert_eq!(
            Lexer::new("\n\n").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::Terminator,
                "\n\n",
                Position::default()
            )]
        );
    }

    #[test]
    fn triple_eol() {
        assert_eq!(
            Lexer::new("\n\n\n").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Terminator, "\n\n", Position::default()),
                Token::new(
                    TokenKind::Eol,
                    "\n",
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
            vec![Token::new(TokenKind::Eol, "\r\n", Position::default())]
        );
    }

    #[test]
    fn windows_double_eol() {
        assert_eq!(
            Lexer::new("\r\n\r\n").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::Terminator,
                "\r\n\r\n",
                Position::default()
            )]
        );
    }

    #[test]
    fn blob_that_ends_with_emoji() {
        assert_eq!(
            Lexer::new("hello blob😉").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, "hello", Position::default()),
                Token::new(
                    TokenKind::Space,
                    " ",
                    Position {
                        byte_index: 5,
                        column: 5,
                        row: 0
                    }
                ),
                Token::new(
                    TokenKind::Literal,
                    "blob😉",
                    Position {
                        byte_index: 6,
                        column: 6,
                        row: 0
                    }
                )
            ]
        )
    }

    #[test]
    fn correct_position_utf8() {
        assert_eq!(
            Lexer::new("blob😉 ").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, "blob😉", Position::default()),
                Token::new(
                    TokenKind::Space,
                    " ",
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
                "{{",
                Position::default()
            )]
        )
    }

    #[test]
    fn colapsible_start() {
        assert_eq!(
            Lexer::new("{%").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::CollapsibleStart,
                "{%",
                Position::default()
            )]
        )
    }

    #[test]
    fn colapsible_end() {
        assert_eq!(
            Lexer::new("{%").collect::<Vec<_>>(),
            vec![Token::new(
                TokenKind::CollapsibleStart,
                "{%",
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
                "]",
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
                "(",
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
                ")",
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
                slice: "\\",
                position: Position {
                    byte_index: 1,
                    column: 0,
                    row: 0
                },
                escaped: true
            }]
        )
    }

    #[test]
    fn escape_after_literal() {
        assert_eq!(
            Lexer::new("literal\\[[").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, "literal", Position::default()),
                Token {
                    kind: TokenKind::Literal,
                    slice: "[",
                    position: Position {
                        byte_index: 8,
                        column: 7,
                        row: 0,
                    },
                    escaped: true
                },
                Token::new(
                    TokenKind::LeftSquareBracket,
                    "[",
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
            vec![Token::new(TokenKind::Star, "**", Position::default()),]
        )
    }

    #[test]
    fn tirple_backtick() {
        assert_eq!(
            Lexer::new("````").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Backtick, "````", Position::default()),]
        );
    }

    #[test]
    fn underscore() {
        assert_eq!(
            Lexer::new("_").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Underscore, "_", Position::default())]
        )
    }

    #[test]
    fn plus() {
        assert_eq!(
            Lexer::new("+").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Plus, "+", Position::default())]
        )
    }

    #[test]
    fn minus() {
        assert_eq!(
            Lexer::new("-").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Minus, "-", Position::default())]
        )
    }

    #[test]
    fn multiple_minus() {
        assert_eq!(
            Lexer::new("----").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Minus, "----", Position::default())]
        )
    }

    #[test]
    fn greater_than() {
        assert_eq!(
            Lexer::new(">>> >>\n>").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::GreaterThan, ">>>", Position::default()),
                Token::new(
                    TokenKind::Space,
                    " ",
                    Position {
                        byte_index: 3,
                        column: 3,
                        row: 0,
                    }
                ),
                Token::new(
                    TokenKind::GreaterThan,
                    ">>",
                    Position {
                        byte_index: 4,
                        column: 4,
                        row: 0,
                    }
                ),
                Token::new(
                    TokenKind::Eol,
                    "\n",
                    Position {
                        byte_index: 6,
                        column: 6,
                        row: 0,
                    }
                ),
                Token::new(
                    TokenKind::GreaterThan,
                    ">",
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
            vec![Token::new(TokenKind::Backtick, "``", Position::default(),),]
        )
    }

    #[test]
    fn strikethrough() {
        assert_eq!(
            Lexer::new("~~").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Tilde, "~~", Position::default())]
        )
    }

    #[test]
    fn strikethrough_after_blob() {
        assert_eq!(
            Lexer::new("abc~~").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, "abc", Position::default()),
                Token::new(
                    TokenKind::Tilde,
                    "~~",
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
                "}}",
                Position::default()
            )]
        )
    }

    #[test]
    fn eol_after_literal() {
        assert_eq!(
            Lexer::new("r\n").collect::<Vec<_>>(),
            vec![
                Token::new(TokenKind::Literal, "r", Position::default()),
                Token::new(
                    TokenKind::Eol,
                    "\n",
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
