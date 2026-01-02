use std::fmt::{Display, Formatter};

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

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Terminator => f.write_str("Terminator"),
            TokenKind::Eol => f.write_str("Eol"),
            TokenKind::LeftCurlyBrace => f.write_str("LeftCurlyBrace"),
            TokenKind::RightCurlyBrace => f.write_str("RightCurlyBrace"),
            TokenKind::CollapsibleStart => f.write_str("CollapsibleStart"),
            TokenKind::CollapsibleEnd => f.write_str("CollapsibleEnd"),
            TokenKind::Tilde => f.write_str("Tilde"),
            TokenKind::Star => f.write_str("Star"),
            TokenKind::Space => f.write_str("Space"),
            TokenKind::Minus => f.write_str("Minus"),
            TokenKind::Hash => f.write_str("Hash"),
            TokenKind::GreaterThan => f.write_str("GreaterThan"),
            TokenKind::Bang => f.write_str("Bang"),
            TokenKind::Backtick => f.write_str("Backtick"),
            TokenKind::Plus => f.write_str("Plus"),
            TokenKind::LeftSquareBracket => f.write_str("LeftSquareBracket"),
            TokenKind::RightSquareBracket => f.write_str("RightSquareBracket"),
            TokenKind::LeftParenthesis => f.write_str("LeftParenthesis"),
            TokenKind::RightParenthesis => f.write_str("RightParenthesis"),
            TokenKind::Underscore => f.write_str("Underscore"),
            TokenKind::Pipe => f.write_str("Pipe"),
            TokenKind::Literal => f.write_str("Literal"),
        }
    }
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
pub struct Token<'input> {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The slice of the input string that corresponds to this token.
    pub slice: &'input str,
    /// The position of the token in the input string.
    pub position: Position,
    /// Indicates if the token is escaped.
    pub escaped: bool,
}

impl<'input> Token<'input> {
    /// Creates a new `Token` instance.
    pub fn new(kind: TokenKind, slice: &'input str, position: Position) -> Self {
        Self {
            kind,
            slice,
            position,
            escaped: false,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.slice)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::lexer::{Position, Token, TokenKind};

    #[test]
    fn display() {
        assert_eq!(
            Token::new(TokenKind::Literal, "str", Position::default()).to_string(),
            "str"
        );
    }
}
