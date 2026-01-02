use std::{cell::RefCell, fmt::Display};

use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: RefCell<usize>,
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            tokens: Lexer::new(input).collect(),
            pos: RefCell::new(0),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn is_eof(&self) -> bool {
        let pos = self.pos.borrow();
        println!("Checking EOF at position {} of {}", *pos, self.tokens.len());
        *pos >= self.tokens.len()
    }

    pub fn is<M>(&self, matcher: M) -> bool
    where
        M: Fn(&Token<'a>) -> bool,
    {
        let pos = self.pos();
        if let Some(token) = self.tokens.get(pos) {
            matcher(token)
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn pos(&self) -> usize {
        *self.pos.borrow()
    }

    pub fn get(&'a self, index: usize) -> Option<&'a Token<'a>> {
        self.tokens.get(index)
    }

    pub fn peek(&'a self) -> Option<(usize, &'a Token<'a>)> {
        let pos = self.pos();

        Some((pos, self.tokens.get(pos)?))
    }

    pub fn advance(&'a self) -> Option<(usize, &'a Token<'a>)> {
        let pos = self.pos();
        let token = self.tokens.get(pos)?;
        self.replace_position(pos + 1)?;
        Some((pos, token))
    }

    pub fn chain(&'a self, query: &Query, invert: bool) -> Option<&'a [Token<'a>]> {
        let start = self.pos();

        if (invert && self.check(query)) || (!invert && !self.check(query)) {
            self.replace_position(start);
            return None;
        }
        Some(self.slice(start..self.pos()))
    }

    fn check(&self, q: &Query) -> bool {
        // println!("Query {}\nposition {}", q, self.pos());
        match q {
            Query::Is(cond) => match self.peek() {
                Some((pos, t)) => {
                    println!("- position {}", pos);
                    cond.is(t)
                }
                None => false,
            },
            Query::Or(queries) => {
                for sub_query in queries.iter() {
                    if self.check(sub_query) {
                        return true;
                    }
                }
                false
            }
            Query::And(items) => {
                for sub_query in items.iter() {
                    if !self.check(sub_query) {
                        return false;
                    }
                }
                true
            }
            Query::Join(items) => {
                let original_pos = self.pos();
                for sub_query in items.iter() {
                    if self.chain(sub_query, false).is_none() {
                        self.replace_position(original_pos);
                        return false;
                    }
                    self.next();
                }
                true
            }
            Query::Eof => self.is_eof(),
        }
    }

    pub fn advance_until(
        &'a self,
        matcher: &Query,
        eof: &Query,
    ) -> Option<(&'a [Token<'a>], &'a [Token<'a>])> {
        let start = self.pos();
        while self.chain(eof, false).is_none()
            && let Some((pos, _)) = self.peek()
        {
            if self.check(matcher) {
                return Some((self.slice(start..pos), self.slice(pos..self.pos())));
            } else {
                self.next();
            }
        }

        self.replace_position(start);
        None
    }

    pub fn slice(&'a self, range: std::ops::Range<usize>) -> &'a [Token<'a>] {
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
}

pub fn terminator(token: &Token) -> bool {
    token.kind == TokenKind::Terminator
}

pub fn first_column(token: &Token) -> bool {
    token.position.column == 0
}

pub fn eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

pub fn one_space(t: &Token) -> bool {
    t.kind == TokenKind::Space && t.slice.len() == 1
}

#[derive(Debug, Clone)]
pub enum Query {
    Is(Condition),
    Or(Vec<Query>),
    And(Vec<Query>),
    Join(Vec<Query>),
    Eof,
}

#[macro_export]
macro_rules! join {
    ( $( $x:expr ),* ) => {
        {
            $crate::op::parser::Query::Join(vec![$( $x ),*])
        }
    };
}

#[macro_export]
macro_rules! and {
    ( $( $x:expr ),* ) => {
        {
            $crate::op::parser::Query::And(vec![$( $x ),*])
        }
    };
}

#[macro_export]
macro_rules! or {
    ( $( $x:expr ),* ) => {
        {
            $crate::op::parser::Query::Or(vec![$( $x ),*])
        }
    };
}

#[macro_export]
macro_rules! is {
    ( $(t=$kind:expr,)? $(c=$column:expr,)? $( el=$exact_len:expr,)? $( maxl=$max_len:expr,)?) => {
        $crate::op::parser::Query::Is(
            $crate::op::parser::Condition::new()
                $( .kind($kind) )?
                $( .column($column) )?
                $( .exact_len($exact_len) )?
                $( .max_len($max_len) )?
        )
    };
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Query::Is(cond) => write!(f, "Is({})", cond),
            Query::Or(queries) => {
                let parts: Vec<String> = queries.iter().map(|q| format!("{}", q)).collect();
                write!(f, "Or({})", parts.join(", "))
            }
            Query::And(queries) => {
                let parts: Vec<String> = queries.iter().map(|q| format!("{}", q)).collect();
                write!(f, "And({})", parts.join(", "))
            }
            Query::Join(queries) => {
                let parts: Vec<String> = queries.iter().map(|q| format!("{}", q)).collect();
                write!(f, "Join({})", parts.join(", "))
            }
            Query::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Condition {
    kind: Option<TokenKind>,
    column: Option<usize>,
    exact_len: Option<usize>,
    min_len: Option<usize>,
    max_len: Option<usize>,
}

impl Condition {
    pub const fn new() -> Self {
        Self {
            kind: None,
            column: None,
            exact_len: None,
            min_len: None,
            max_len: None,
        }
    }

    pub const fn kind(mut self, kind: TokenKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub const fn column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub const fn exact_len(mut self, len: usize) -> Self {
        self.exact_len = Some(len);
        self
    }

    pub fn min_len(mut self, len: usize) -> Self {
        self.min_len = Some(len);
        self
    }

    pub fn max_len(mut self, len: usize) -> Self {
        self.max_len = Some(len);
        self
    }

    pub fn is(&self, t: &Token) -> bool {
        println!("˹Is({})\n˻{:?}", self, t);
        if let Some(kind) = &self.kind
            && &t.kind != kind
        {
            return false;
        }
        if let Some(column) = &self.column
            && &t.position.column != column
        {
            return false;
        }
        if let Some(exact_len) = &self.exact_len
            && t.slice.len() != *exact_len
        {
            return false;
        }
        if let Some(min_len) = &self.min_len
            && t.slice.len() < *min_len
        {
            return false;
        }
        if let Some(max_len) = &self.max_len
            && t.slice.len() > *max_len
        {
            return false;
        }
        true
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = vec![];
        if let Some(kind) = &self.kind {
            parts.push(format!("kind: {}", kind));
        }
        if let Some(column) = &self.column {
            parts.push(format!("column: {}", column));
        }
        if let Some(exact_len) = &self.exact_len {
            parts.push(format!("exact_len: {}", exact_len));
        }
        if let Some(min_len) = &self.min_len {
            parts.push(format!("min_len: {}", min_len));
        }
        if let Some(max_len) = &self.max_len {
            parts.push(format!("max_len: {}", max_len));
        }
        write!(f, "{}", parts.join(", "))
    }
}

#[cfg(test)]
mod tests {}
