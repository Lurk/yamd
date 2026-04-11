use std::ops::Range;

use crate::lexer::{Lexer, Token, TokenKind};
use crate::op::{Content, Node, Op};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListKind {
    Unordered,
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

#[derive(Debug, Clone, Copy)]
pub enum StopCondition {
    Terminator,
    CollapsibleEnd,
    HighlightEnd,
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
    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    pub fn peek(&self) -> Option<(usize, &Token)> {
        Some((self.pos, self.tokens.get(self.pos)?))
    }

    pub fn advance(&mut self) -> Option<usize> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let pos = self.pos;
        self.pos += 1;
        Some(pos)
    }

    pub fn slice(&self, range: Range<usize>) -> &[Token] {
        &self.tokens[range]
    }

    pub fn next(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

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

    pub fn at(&self, pred: impl Fn(&Token) -> bool) -> bool {
        self.peek().is_some_and(|(_, t)| pred(t))
    }

    pub fn with_eof<R>(&mut self, cond: StopCondition, f: impl FnOnce(&mut Self) -> R) -> R {
        self.eof_stack.push(cond);
        let result = f(self);
        self.eof_stack.pop();
        result
    }

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

    pub fn at_eof(&self) -> bool {
        let Some((_, token)) = self.peek() else {
            return true;
        };
        self.eof_stack.iter().any(|cond| cond.matches(token, self))
    }

    pub fn flip_to_literal(&mut self, pos: usize) {
        if let Some(token) = self.tokens.get_mut(pos) {
            token.kind = TokenKind::Literal;
        }
    }

    pub fn at_block_boundary(&self) -> bool {
        self.at_eof() || self.at(|t| t.kind == TokenKind::Terminator)
    }

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

    pub fn span(&self, range: Range<usize>) -> Content {
        if range.is_empty() {
            Content::Span(0..0)
        } else {
            let byte_start = self.tokens[range.start].range.start;
            let byte_end = self.tokens[range.end - 1].range.end;
            Content::Span(byte_start..byte_end)
        }
    }

    pub fn into_ops(self) -> Vec<Op> {
        self.ops
    }
}

pub fn eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

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
