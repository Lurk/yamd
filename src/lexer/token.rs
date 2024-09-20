use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// Two consecutive [TokenKind::Eol]
    Terminator,
    /// Exactly one `\n` or `\r\n` combination
    Eol,
    /// One or more `{`
    LeftCurlyBrace,
    /// One or more `}`
    RightCurlyBrace,
    /// Exactly one `{%` combination
    ColapsibleStart,
    /// Exactly one `%}` combination
    ColapsibleEnd,
    /// One or more `~`
    Tilde,
    /// One or more `*`
    Star,
    /// One or more ` `
    Space,
    /// One or more `-`
    Minus,
    /// One or more `#`
    Hash,
    /// One or more `>`
    GreaterThan,
    /// One or more `!`
    Bang,
    /// One or more `` ` ``
    Backtick,
    /// One or more `+`
    Plus,
    /// Exactly one `[`
    LeftSquareBracket,
    /// Exactly one `]`
    RightSquareBracket,
    /// Exactly one `(`
    LeftParenthesis,
    /// Exactly one `)`
    RightParenthesis,
    /// Exactly one `_`
    Underscore,
    /// Exactly one `|`
    Pipe,
    /// One or more chars that does not fall to one of the rules from above.
    Literal,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Position {
    pub byte_index: usize,
    pub column: usize,
    pub row: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token<'input> {
    pub kind: TokenKind,
    pub slice: &'input str,
    pub position: Position,
}

impl<'input> Token<'input> {
    pub fn new(kind: TokenKind, slice: &'input str, position: Position) -> Self {
        Self {
            kind,
            slice,
            position,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.slice)
    }
}