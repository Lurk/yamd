use std::{
    fmt::{Display, Formatter},
    ops::Range,
};

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
#[derive(Debug, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_kind_display() {
        assert_eq!(TokenKind::Terminator.to_string(), "Terminator");
        assert_eq!(TokenKind::Eol.to_string(), "Eol");
        assert_eq!(TokenKind::LeftCurlyBrace.to_string(), "LeftCurlyBrace");
        assert_eq!(TokenKind::RightCurlyBrace.to_string(), "RightCurlyBrace");
        assert_eq!(TokenKind::CollapsibleStart.to_string(), "CollapsibleStart");
        assert_eq!(TokenKind::CollapsibleEnd.to_string(), "CollapsibleEnd");
        assert_eq!(TokenKind::Tilde.to_string(), "Tilde");
        assert_eq!(TokenKind::Star.to_string(), "Star");
        assert_eq!(TokenKind::Space.to_string(), "Space");
        assert_eq!(TokenKind::Minus.to_string(), "Minus");
        assert_eq!(TokenKind::Hash.to_string(), "Hash");
        assert_eq!(TokenKind::GreaterThan.to_string(), "GreaterThan");
        assert_eq!(TokenKind::Bang.to_string(), "Bang");
        assert_eq!(TokenKind::Backtick.to_string(), "Backtick");
        assert_eq!(TokenKind::Plus.to_string(), "Plus");
        assert_eq!(
            TokenKind::LeftSquareBracket.to_string(),
            "LeftSquareBracket"
        );
        assert_eq!(
            TokenKind::RightSquareBracket.to_string(),
            "RightSquareBracket"
        );
        assert_eq!(TokenKind::LeftParenthesis.to_string(), "LeftParenthesis");
        assert_eq!(TokenKind::RightParenthesis.to_string(), "RightParenthesis");
        assert_eq!(TokenKind::Underscore.to_string(), "Underscore");
        assert_eq!(TokenKind::Pipe.to_string(), "Pipe");
        assert_eq!(TokenKind::Literal.to_string(), "Literal");
    }

    #[test]
    fn token_new() {
        let token = Token::new(TokenKind::Literal, 0..5, Position::default());
        assert_eq!(token.kind, TokenKind::Literal);
        assert_eq!(token.range, 0..5);
        assert!(!token.escaped);
    }
}
