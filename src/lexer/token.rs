use std::ops::Range;

/// The `TokenKind` enum represents the different types of tokens that can be found in the input.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
    CollapsibleStart,
    /// Exactly one `%}` combination
    CollapsibleEnd,
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

/// The `Position` struct represents the position of a token in the input.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Position {
    /// The byte index of the token in the input string.
    pub byte_index: usize,
    /// The column number of the token in the input string.
    pub column: usize,
    /// The line number of the token in the input string.
    pub row: usize,
}

/// The `Token` struct represents a token in the input string.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The range in the input string that corresponds to this token.
    pub range: Range<usize>,
    /// The position of the token in the input string.
    pub position: Position,
    /// Indicates if the token is escaped.
    pub escaped: bool,
}

impl Token {
    /// Creates a new non escaped `Token` instance.
    pub fn new(kind: TokenKind, range: Range<usize>, position: Position) -> Self {
        Self {
            kind,
            range,
            position,
            escaped: false,
        }
    }
}
