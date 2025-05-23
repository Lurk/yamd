mod anchor;
mod bold;
mod branch_builder;
mod code;
mod code_span;
mod collapsible;
mod embed;
mod emphasis;
mod heading;
mod highlight;
mod image;
mod italic;
mod list;
mod metadata;
mod paragraph;
mod strikethrough;
mod yamd;

use std::{
    fmt::Debug,
    iter::Peekable,
    ops::{Bound, Range, RangeBounds, RangeTo},
};

pub(crate) use anchor::anchor;
pub(crate) use bold::bold;
pub(crate) use branch_builder::BranchBuilder;
pub(crate) use code::code;
pub(crate) use code_span::code_span;
pub(crate) use collapsible::collapsible;
pub(crate) use embed::embed;
pub(crate) use emphasis::emphasis;
pub(crate) use heading::heading;
pub(crate) use highlight::highlight;
pub(crate) use image::images;
pub(crate) use italic::italic;
pub(crate) use list::list;
pub(crate) use metadata::metadata;
pub(crate) use paragraph::paragraph;
pub(crate) use strikethrough::strikethrough;
pub(crate) use yamd::yamd;

use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser<'input> {
    lexer: Peekable<Lexer<'input>>,
    stack: Vec<Token<'input>>,
    stack_pos: usize,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            stack: vec![],
            stack_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<&Token<'input>> {
        if self.stack.len() > self.stack_pos {
            let res = self.stack.get(self.stack_pos);
            self.stack_pos += 1;
            return res;
        };

        self.stack.push(self.lexer.next()?);
        self.stack_pos = self.stack.len();
        self.stack.get(self.stack_pos - 1)
    }

    pub fn peek(&mut self) -> Option<(&Token<'input>, usize)> {
        if self.stack.len() > self.stack_pos {
            return self.stack.get(self.stack_pos).map(|t| (t, self.stack_pos));
        }

        self.lexer.peek().map(|t| (t, self.stack_pos))
    }

    pub fn range_to_string<R: RangeBounds<usize> + Debug>(&self, range: R) -> String {
        self.stack[try_range(range, ..self.stack.len()).expect("range to fit")]
            .iter()
            .map(|t| t.slice)
            .collect()
    }

    pub fn backtrack(&mut self, index: usize) {
        if index < self.stack.len() {
            self.stack_pos = index;
        }
    }

    pub fn flip_to_literal_at(&mut self, index: usize) -> bool {
        if let Some(t) = self.stack.get_mut(index) {
            t.kind = TokenKind::Literal;
            return true;
        }
        false
    }

    pub fn advance_until<Callback>(
        &mut self,
        f: Callback,
        skip_terminator: bool,
    ) -> Option<(usize, usize)>
    where
        Callback: Fn(&Token) -> bool,
    {
        let start = self.pos();
        self.next_token();

        while let Some((t, pos)) = self.peek() {
            if t.kind == TokenKind::Terminator && !skip_terminator {
                break;
            };

            if f(t) {
                return Some((start, pos));
            }
            self.next_token();
        }

        None
    }

    pub fn advance_or_backtrack<Callback>(&mut self, f: Callback) -> Option<(usize, usize)>
    where
        Callback: Fn(&Token) -> bool,
    {
        let start = self.pos();

        let Some(result) = self.advance_until(f, false) else {
            self.backtrack(start);
            self.flip_to_literal_at(start);
            return None;
        };
        self.next_token();
        Some(result)
    }

    pub fn pos(&self) -> usize {
        self.stack_pos
    }
}

/// Converts any range: Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive, to
/// Range
/// TODO: remove when slice::try_range will land
fn try_range<R>(range: R, bounds: RangeTo<usize>) -> Option<Range<usize>>
where
    R: RangeBounds<usize>,
{
    let len = bounds.end;

    let start = match range.start_bound() {
        Bound::Included(&start) => start,
        Bound::Excluded(start) => start.checked_add(1)?,
        Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        Bound::Included(end) => end.checked_add(1)?,
        Bound::Excluded(&end) => end,
        Bound::Unbounded => len,
    };

    if start > end || end > len {
        None
    } else {
        Some(Range { start, end })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Bound;

    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        parser::{try_range, Parser},
    };

    #[test]
    fn _try_range() {
        assert_eq!(try_range(.., ..2), Some(0..2));
        assert_eq!(try_range(0.., ..2), Some(0..2));
        assert_eq!(try_range(0..1, ..2), Some(0..1));
        assert_eq!(try_range(..1, ..2), Some(0..1));
        assert_eq!(try_range(0..=1, ..2), Some(0..2));
        assert_eq!(try_range(0..3, ..2), None);
        assert_eq!(
            try_range((Bound::Excluded(1), Bound::Excluded(2)), ..3),
            Some(2..2)
        );
    }

    #[test]
    fn first_next_token() {
        let mut p = Parser::new("test");
        assert_eq!(
            p.next_token(),
            Some(&Token::new(TokenKind::Literal, "test", Position::default()))
        );
    }

    #[test]
    fn advance_until() {
        let mut p = Parser::new("!test");

        assert_eq!(p.advance_or_backtrack(|t| t.kind == TokenKind::Space), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "!", Position::default()), 0))
        )
    }

    #[test]
    fn backtrack() {
        let mut p = Parser::new("!");
        p.next_token();
        p.backtrack(0);
        assert_eq!(
            p.next_token(),
            Some(&Token::new(TokenKind::Bang, "!", Position::default()))
        );
    }
}
