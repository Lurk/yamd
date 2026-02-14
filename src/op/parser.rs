use std::cell::RefCell;

use crate::eat_seq;
use crate::lexer::{Lexer, Token, TokenKind};
use crate::op::Node;

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
    let start = p.pos();
    for level in 0..=max_level {
        let k = if level == current_level {
            Some(kind)
        } else {
            None
        };
        let matched = if level == 0 {
            p.at(|t: &Token| t.position.column == 0)
                && eat_seq!(p, |t: &Token| is_list_marker(t, k), is_space_1).is_some()
        } else {
            p.at(|t: &Token| t.position.column == 0)
                && eat_seq!(
                    p,
                    |t: &Token| t.kind == TokenKind::Space && t.range.len() == level,
                    |t: &Token| is_list_marker(t, k),
                    is_space_1
                )
                .is_some()
        };
        p.replace_position(start);
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

pub struct EofGuard<'a> {
    parser: &'a Parser,
}

impl Drop for EofGuard<'_> {
    fn drop(&mut self) {
        self.parser.eof_stack.borrow_mut().pop();
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: RefCell<usize>,
    eof_stack: RefCell<Vec<StopCondition>>,
}

impl From<&str> for Parser {
    fn from(input: &str) -> Self {
        Self {
            tokens: Lexer::new(input).collect(),
            pos: RefCell::new(0),
            eof_stack: RefCell::new(Vec::new()),
        }
    }
}

impl Parser {
    pub fn is_eof(&self) -> bool {
        *self.pos.borrow() >= self.tokens.len()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn pos(&self) -> usize {
        *self.pos.borrow()
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    pub fn peek(&self) -> Option<(usize, &Token)> {
        let pos = self.pos();
        Some((pos, self.tokens.get(pos)?))
    }

    pub fn advance(&self) -> Option<(usize, &Token)> {
        let pos = self.pos();
        let token = self.tokens.get(pos)?;
        self.replace_position(pos + 1)?;
        Some((pos, token))
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> &[Token] {
        &self.tokens[range]
    }

    pub fn next(&self) {
        let pos = self.pos();
        self.replace_position(pos + 1);
    }

    pub fn replace_position(&self, new_pos: usize) -> Option<usize> {
        if new_pos > self.len() {
            None
        } else {
            Some(self.pos.replace(new_pos))
        }
    }

    pub fn eat(&self, pred: impl Fn(&Token) -> bool) -> Option<&[Token]> {
        let pos = self.pos();
        let (_, token) = self.peek()?;
        if pred(token) {
            self.next();
            Some(self.slice(pos..self.pos()))
        } else {
            None
        }
    }

    pub fn at(&self, pred: impl Fn(&Token) -> bool) -> bool {
        self.peek().is_some_and(|(_, t)| pred(t))
    }

    pub fn push_eof(&self, cond: StopCondition) -> EofGuard<'_> {
        self.eof_stack.borrow_mut().push(cond);
        EofGuard { parser: self }
    }

    pub fn at_eof(&self) -> bool {
        let Some((_, token)) = self.peek() else {
            return true;
        };
        self.eof_stack
            .borrow()
            .iter()
            .any(|cond| cond.matches(token, self))
    }

    /// Like `at_eof()`, but also treats a Terminator token as a block boundary.
    /// Use this in block-level parsers (code, embed) that need to accept
    /// terminators as valid end-of-block when called without a Terminator
    /// stop condition on the stack.
    pub fn at_block_boundary(&self) -> bool {
        self.at_eof() || self.at(|t| t.kind == TokenKind::Terminator)
    }

    pub fn advance_until(&self, matcher: impl Fn(&Token) -> bool) -> Option<(&[Token], &[Token])> {
        let start = self.pos();
        while let Some((pos, token)) = self.peek() {
            if self.at_eof() {
                break;
            }
            if matcher(token) {
                let before = self.slice(start..pos);
                self.next();
                let matched = self.slice(pos..self.pos());
                return Some((before, matched));
            }
            self.next();
        }
        self.replace_position(start);
        None
    }
}

pub fn eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

#[macro_export]
macro_rules! eat_seq {
    ($p:expr, $($pred:expr),+ $(,)?) => {{
        let start = $p.pos();
        if $( $p.eat($pred).is_some() )&&+ {
            Some($p.slice(start..$p.pos()))
        } else {
            $p.replace_position(start);
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
        let p = Parser::from("hello world");
        let result = p.eat(|t| t.kind == TokenKind::Literal);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
        assert_eq!(p.pos(), 1);
    }

    #[test]
    fn eat_no_match_does_not_advance() {
        let p = Parser::from("hello world");
        let result = p.eat(|t| t.kind == TokenKind::Star);
        assert!(result.is_none());
        assert_eq!(p.pos(), 0);
    }

    #[test]
    fn eat_at_eof_returns_none() {
        let p = Parser::from("");
        let result = p.eat(|t| t.kind == TokenKind::Literal);
        assert!(result.is_none());
    }

    #[test]
    fn at_matches_without_consuming() {
        let p = Parser::from("hello");
        assert!(p.at(|t| t.kind == TokenKind::Literal));
        assert_eq!(p.pos(), 0);
    }

    #[test]
    fn at_no_match() {
        let p = Parser::from("hello");
        assert!(!p.at(|t| t.kind == TokenKind::Star));
        assert_eq!(p.pos(), 0);
    }

    #[test]
    fn at_eof_returns_false() {
        let p = Parser::from("");
        assert!(!p.at(|t| t.kind == TokenKind::Literal));
    }

    #[test]
    fn eat_seq_matches_sequence() {
        let p = Parser::from("# hello");
        let result = eat_seq!(p, |t: &Token| t.kind == TokenKind::Hash, |t: &Token| t.kind
            == TokenKind::Space);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 2);
        assert_eq!(p.pos(), 2);
    }

    #[test]
    fn eat_seq_backtracks_on_partial_match() {
        let p = Parser::from("# hello");
        let result = eat_seq!(p, |t: &Token| t.kind == TokenKind::Hash, |t: &Token| t.kind
            == TokenKind::Star);
        assert!(result.is_none());
        assert_eq!(p.pos(), 0);
    }

    #[test]
    fn eat_seq_single_predicate() {
        let p = Parser::from("hello");
        let result = eat_seq!(p, |t: &Token| t.kind == TokenKind::Literal);
        assert!(result.is_some());
        assert_eq!(p.pos(), 1);
    }

    #[test]
    fn advance_until_finds_match() {
        let p = Parser::from("hello\nworld");
        let result = p.advance_until(|t: &Token| t.kind == TokenKind::Eol);
        assert!(result.is_some());
        let (before, matched) = result.unwrap();
        assert_eq!(before.len(), 1);
        assert_eq!(matched.len(), 1);
    }

    #[test]
    fn advance_until_eof_hit() {
        let p = Parser::from("hello\n\nworld");
        let _g = p.push_eof(StopCondition::Terminator);
        let result = p.advance_until(|t: &Token| t.kind == TokenKind::Star);
        assert!(result.is_none());
        assert_eq!(p.pos(), 0);
    }

    #[test]
    fn advance_until_no_match_at_eof() {
        let p = Parser::from("hello");
        let result = p.advance_until(|t: &Token| t.kind == TokenKind::Star);
        assert!(result.is_none());
        assert_eq!(p.pos(), 0);
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
        let p = Parser::from("hello\n\nworld");
        let _g = p.push_eof(StopCondition::Terminator);
        assert!(!p.at_eof());
        p.next();
        assert!(p.at_eof());
    }

    #[test]
    fn eof_guard_pops_on_drop() {
        let p = Parser::from("hello\n\nworld");
        {
            let _g = p.push_eof(StopCondition::Terminator);
            p.next();
            assert!(p.at_eof());
        }
        assert!(!p.at_eof());
    }

    #[test]
    fn nested_guards_pop_in_order() {
        let p = Parser::from("hello\n\nworld");
        let _outer = p.push_eof(StopCondition::Terminator);
        p.next();
        assert!(p.at_eof());
        {
            let _inner = p.push_eof(StopCondition::HighlightEnd);
            assert!(p.at_eof());
        }
        assert!(p.at_eof());
    }
}
