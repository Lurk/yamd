use std::ops::Range;

use crate::lexer::{Lexer, Token, TokenKind};
use crate::op::{Content, Node, Op};

/// Distinguishes unordered (`-`) from ordered (`+`) lists during parsing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListKind {
    /// Unordered list, items prefixed with `-`. Maps to [`Node::UnorderedList`].
    Unordered,
    /// Ordered list, items prefixed with `+`. Maps to [`Node::OrderedList`].
    Ordered,
}

impl ListKind {
    pub fn node(&self) -> Node {
        match self {
            ListKind::Unordered => Node::UnorderedList,
            ListKind::Ordered => Node::OrderedList,
        }
    }
}

impl TryFrom<&Token> for ListKind {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        if value.kind == TokenKind::Minus && value.range.len() == 1 {
            Ok(ListKind::Unordered)
        } else if value.kind == TokenKind::Plus && value.range.len() == 1 {
            Ok(ListKind::Ordered)
        } else {
            Err(())
        }
    }
}

/// Defines when the parser should treat the current position as a logical end-of-input.
///
/// Stop conditions are pushed onto a stack via [`Parser::with_eof`] and checked by
/// [`Parser::at_eof`]. This lets nested parsers (e.g., a paragraph inside a collapsible block)
/// stop at their enclosing delimiter without consuming it.
#[derive(Debug, Clone, Copy)]
pub enum StopCondition {
    /// Double newline — separates block-level elements.
    Terminator,
    /// `%}` at column 0 — ends a collapsible block.
    CollapsibleEnd,
    /// `!!` (two bangs) at column 0 — ends a highlight block.
    HighlightEnd,
    /// A list marker at or below the given nesting level — signals a sibling or parent item.
    ListBoundary { level: usize, kind: ListKind },
}

fn is_list_marker(t: &Token, kind: Option<ListKind>) -> bool {
    match kind {
        Some(ListKind::Unordered) => t.kind == TokenKind::Minus && t.range.len() == 1,
        Some(ListKind::Ordered) => t.kind == TokenKind::Plus && t.range.len() == 1,
        None => (t.kind == TokenKind::Minus || t.kind == TokenKind::Plus) && t.range.len() == 1,
    }
}

fn is_space_1(t: &Token) -> bool {
    t.kind == TokenKind::Space && t.range.len() == 1
}

fn at_list_boundary(p: &Parser, current_level: usize, max_level: usize, kind: ListKind) -> bool {
    for level in 0..=max_level {
        let k = if level == current_level {
            Some(kind)
        } else {
            None
        };
        let mut offset = p.pos;
        let matched = if level == 0 {
            p.tokens.get(offset).is_some_and(|t| t.position.column == 0) && {
                // check: list_marker, space_1
                p.tokens.get(offset).is_some_and(|t| is_list_marker(t, k)) && {
                    offset += 1;
                    p.tokens.get(offset).is_some_and(is_space_1)
                }
            }
        } else {
            p.tokens.get(offset).is_some_and(|t| t.position.column == 0) && {
                // check: space of len==level, list_marker, space_1
                p.tokens
                    .get(offset)
                    .is_some_and(|t| t.kind == TokenKind::Space && t.range.len() == level)
                    && {
                        offset += 1;
                        p.tokens.get(offset).is_some_and(|t| is_list_marker(t, k)) && {
                            offset += 1;
                            p.tokens.get(offset).is_some_and(is_space_1)
                        }
                    }
            }
        };
        if matched {
            return true;
        }
    }
    false
}

impl StopCondition {
    fn matches(&self, token: &Token, parser: &Parser) -> bool {
        match self {
            Self::Terminator => token.kind == TokenKind::Terminator,
            Self::CollapsibleEnd => {
                token.kind == TokenKind::CollapsibleEnd && token.position.column == 0
            }
            Self::HighlightEnd => {
                token.kind == TokenKind::Bang
                    && token.position.column == 0
                    && token.range.len() == 2
            }
            Self::ListBoundary { level, kind } => {
                token.position.column == 0 && at_list_boundary(parser, *level, *level + 1, *kind)
            }
        }
    }
}

/// Token-stream parser that produces a flat [`Op`] sequence.
///
/// Wraps the lexer output with a position cursor, a [`StopCondition`] stack for context-sensitive
/// end-of-input detection, and an output buffer of [`Op`]s.
///
/// Node-specific parsing functions (e.g., `heading`, `paragraph`) receive `&mut Parser`, use
/// [`eat`](Parser::eat)/[`at`](Parser::at) to match tokens, and push results to [`ops`](Parser::ops).
/// On mismatch they restore [`pos`](Parser::pos) and truncate `ops` to backtrack.
pub struct Parser<'a> {
    pub source: &'a str,
    tokens: Vec<Token>,
    pub pos: usize,
    eof_stack: Vec<StopCondition>,
    pub ops: Vec<Op>,
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            source: input,
            tokens: Lexer::new(input).collect(),
            pos: 0,
            eof_stack: Vec::new(),
            ops: Vec::new(),
        }
    }
}

impl Parser<'_> {
    /// Returns `true` when the position cursor has passed the last token.
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Returns the total number of tokens.
    #[inline]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Returns `true` if the token stream is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Returns the token at `index`, or `None` if out of bounds.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    /// Returns the current position and token, or `None` at end-of-stream.
    #[inline]
    pub fn peek(&self) -> Option<(usize, &Token)> {
        Some((self.pos, self.tokens.get(self.pos)?))
    }

    /// Advances the cursor by one and returns the previous position, or `None` at end-of-stream.
    #[inline]
    pub fn advance(&mut self) -> Option<usize> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let pos = self.pos;
        self.pos += 1;
        Some(pos)
    }

    /// Returns a slice of tokens for the given index range.
    #[inline]
    pub fn slice(&self, range: Range<usize>) -> &[Token] {
        &self.tokens[range]
    }

    /// Advances the cursor by one. Does nothing at end-of-stream.
    #[inline]
    pub fn next(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    /// Consumes the current token if `pred` returns `true`. Returns the token index range on match, or `None` (without advancing) on mismatch.
    #[inline]
    pub fn eat(&mut self, pred: impl Fn(&Token) -> bool) -> Option<Range<usize>> {
        let token = self.tokens.get(self.pos)?;
        if pred(token) {
            let start = self.pos;
            self.pos += 1;
            Some(start..self.pos)
        } else {
            None
        }
    }

    /// Returns `true` if the current token satisfies `pred`, without consuming it.
    #[inline]
    pub fn at(&self, pred: impl Fn(&Token) -> bool) -> bool {
        self.peek().is_some_and(|(_, t)| pred(t))
    }

    /// Pushes `cond` onto the stop-condition stack, runs `f`, then pops it.
    /// This scopes a stop condition to a parsing function — nested parsers see the condition
    /// via [`at_eof`](Parser::at_eof) and stop before consuming the delimiter.
    pub fn with_eof<R>(&mut self, cond: StopCondition, f: impl FnOnce(&mut Self) -> R) -> R {
        self.eof_stack.push(cond);
        let result = f(self);
        self.eof_stack.pop();
        result
    }

    /// Like [`with_eof`](Parser::with_eof) but pushes multiple conditions at once.
    pub fn with_eofs<R>(&mut self, conds: &[StopCondition], f: impl FnOnce(&mut Self) -> R) -> R {
        for &c in conds {
            self.eof_stack.push(c);
        }
        let result = f(self);
        for _ in conds {
            self.eof_stack.pop();
        }
        result
    }

    /// Returns `true` if at end-of-stream or the current token matches any condition on the stop-condition stack.
    #[inline]
    pub fn at_eof(&self) -> bool {
        let Some((_, token)) = self.peek() else {
            return true;
        };
        self.eof_stack.iter().any(|cond| cond.matches(token, self))
    }

    /// Changes the token at `pos` to [`Literal`](crate::lexer::TokenKind::Literal).
    /// Used during backtracking to prevent a special character from being re-interpreted
    /// as a delimiter on the next parse attempt.
    pub fn flip_to_literal(&mut self, pos: usize) {
        if let Some(token) = self.tokens.get_mut(pos) {
            token.kind = TokenKind::Literal;
        }
    }

    /// Returns `true` if at a block boundary — either at logical EOF or at a [`Terminator`](StopCondition::Terminator) token.
    #[inline]
    pub fn at_block_boundary(&self) -> bool {
        self.at_eof() || self.at(|t| t.kind == TokenKind::Terminator)
    }

    /// Scans forward from the current position looking for a token that satisfies `matcher`.
    /// Returns `(before_range, match_range)` on success, where `before_range` covers tokens
    /// before the match and `match_range` covers the matched token. Backtracks to the starting
    /// position if no match is found before EOF or a stop condition.
    pub fn advance_until(
        &mut self,
        matcher: impl Fn(&Token) -> bool,
    ) -> Option<(Range<usize>, Range<usize>)> {
        let start = self.pos;
        while let Some(token) = self.tokens.get(self.pos) {
            if self.at_eof() {
                break;
            }
            if matcher(token) {
                let before = start..self.pos;
                let match_start = self.pos;
                self.pos += 1;
                return Some((before, match_start..self.pos));
            }
            self.pos += 1;
        }
        self.pos = start;
        None
    }

    /// Converts a token index range into a [`Content::Span`] using the tokens' byte ranges.
    #[inline]
    pub fn span(&self, range: Range<usize>) -> Content {
        if range.is_empty() {
            Content::Span(0..0)
        } else {
            let byte_start = self.tokens[range.start].range.start;
            let byte_end = self.tokens[range.end - 1].range.end;
            Content::Span(byte_start..byte_end)
        }
    }

    /// Consumes the parser and returns the accumulated operations.
    pub fn into_ops(self) -> Vec<Op> {
        self.ops
    }
}

/// Token predicate that matches an end-of-line token. Intended for use with [`Parser::eat`]
/// and [`Parser::at`].
pub fn eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

/// Attempts to eat a sequence of tokens matching the given predicates in order.
/// Returns `Some(start..end)` covering all matched tokens on success, or `None`
/// with the parser position restored on partial or full mismatch.
///
/// ```
/// # use yamd::op::Parser;
/// # use yamd::eat_seq;
/// # use yamd::lexer::{Token, TokenKind};
/// let mut p = Parser::from("# hello");
/// let range = eat_seq!(p, |t: &Token| t.kind == TokenKind::Hash, |t: &Token| t.kind == TokenKind::Space);
/// assert!(range.is_some());
/// ```
#[macro_export]
macro_rules! eat_seq {
    ($p:expr, $($pred:expr),+ $(,)?) => {{
        let start = $p.pos;
        if $( $p.eat($pred).is_some() )&&+ {
            Some(start..$p.pos)
        } else {
            $p.pos = start;
            None
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::TokenKind;

    #[test]
    fn eat_matches_and_consumes() {
        let mut p = Parser::from("hello world");
        let result = p.eat(|t| t.kind == TokenKind::Literal);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0..1);
        assert_eq!(p.pos, 1);
    }

    #[test]
    fn eat_no_match_does_not_advance() {
        let mut p = Parser::from("hello world");
        let result = p.eat(|t| t.kind == TokenKind::Star);
        assert!(result.is_none());
        assert_eq!(p.pos, 0);
    }

    #[test]
    fn eat_at_eof_returns_none() {
        let mut p = Parser::from("");
        let result = p.eat(|t| t.kind == TokenKind::Literal);
        assert!(result.is_none());
    }

    #[test]
    fn at_matches_without_consuming() {
        let p = Parser::from("hello");
        assert!(p.at(|t| t.kind == TokenKind::Literal));
        assert_eq!(p.pos, 0);
    }

    #[test]
    fn at_no_match() {
        let p = Parser::from("hello");
        assert!(!p.at(|t| t.kind == TokenKind::Star));
        assert_eq!(p.pos, 0);
    }

    #[test]
    fn at_eof_returns_false() {
        let p = Parser::from("");
        assert!(!p.at(|t| t.kind == TokenKind::Literal));
    }

    #[test]
    fn eat_seq_matches_sequence() {
        let mut p = Parser::from("# hello");
        let result = eat_seq!(p, |t: &Token| t.kind == TokenKind::Hash, |t: &Token| t.kind
            == TokenKind::Space);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0..2);
        assert_eq!(p.pos, 2);
    }

    #[test]
    fn eat_seq_backtracks_on_partial_match() {
        let mut p = Parser::from("# hello");
        let result = eat_seq!(p, |t: &Token| t.kind == TokenKind::Hash, |t: &Token| t.kind
            == TokenKind::Star);
        assert!(result.is_none());
        assert_eq!(p.pos, 0);
    }

    #[test]
    fn eat_seq_single_predicate() {
        let mut p = Parser::from("hello");
        let result = eat_seq!(p, |t: &Token| t.kind == TokenKind::Literal);
        assert!(result.is_some());
        assert_eq!(p.pos, 1);
    }

    #[test]
    fn advance_until_finds_match() {
        let mut p = Parser::from("hello\nworld");
        let result = p.advance_until(|t: &Token| t.kind == TokenKind::Eol);
        assert!(result.is_some());
        let (before, matched) = result.unwrap();
        assert_eq!(before, 0..1);
        assert_eq!(matched, 1..2);
    }

    #[test]
    fn advance_until_eof_hit() {
        let mut p = Parser::from("hello\n\nworld");
        p.with_eof(StopCondition::Terminator, |p| {
            let result = p.advance_until(|t: &Token| t.kind == TokenKind::Star);
            assert!(result.is_none());
            assert_eq!(p.pos, 0);
        });
    }

    #[test]
    fn advance_until_no_match_at_eof() {
        let mut p = Parser::from("hello");
        let result = p.advance_until(|t: &Token| t.kind == TokenKind::Star);
        assert!(result.is_none());
        assert_eq!(p.pos, 0);
    }

    #[test]
    fn at_eof_empty_stack_not_at_end() {
        let p = Parser::from("hello");
        assert!(!p.at_eof());
    }

    #[test]
    fn at_eof_empty_stack_at_end() {
        let p = Parser::from("");
        assert!(p.at_eof());
    }

    #[test]
    fn at_eof_terminator_on_stack() {
        let mut p = Parser::from("hello\n\nworld");
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!p.at_eof());
            p.next();
            assert!(p.at_eof());
        });
    }

    #[test]
    fn eof_guard_pops_on_drop() {
        let mut p = Parser::from("hello\n\nworld");
        p.with_eof(StopCondition::Terminator, |p| {
            p.next();
            assert!(p.at_eof());
        });
        assert!(!p.at_eof());
    }

    #[test]
    fn nested_guards_pop_in_order() {
        let mut p = Parser::from("hello\n\nworld");
        p.with_eof(StopCondition::Terminator, |p| {
            p.next();
            assert!(p.at_eof());
            p.with_eof(StopCondition::HighlightEnd, |p| {
                assert!(p.at_eof());
            });
            assert!(p.at_eof());
        });
    }
}
