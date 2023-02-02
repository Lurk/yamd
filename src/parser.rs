use std::{iter::Enumerate, str::Chars};

pub trait Branch<Tags> {
    fn new() -> Self;
    fn push<Node: Into<Tags>>(&mut self, node: Node);
    fn from_vec(nodes: Vec<Tags>) -> Self;
}

type ParserToTags<Tags> = Box<dyn Fn(&str, usize) -> Option<(Tags, usize)>>;

pub trait Parser<Tags> {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)>
    where
        Self: Sized;

    fn parse_to_tag<T>(input: &str, start_position: usize) -> Option<(T, usize)>
    where
        Self: Sized + Into<T>,
    {
        if let Some((node, pos)) = Self::parse(input, start_position) {
            return Some((node.into(), pos));
        }
        None
    }

    fn parse_node<Node: Into<Tags>>(
        chunk: &str,
        parsers: &[ParserToTags<Tags>; 2],
        fallback: Box<dyn Fn(&str) -> Node>,
    ) -> Self
    where
        Self: Sized + Branch<Tags>,
    {
        let mut result = Self::new();
        let mut chunk_position = 0;
        let mut text_start = 0;
        while chunk_position < chunk.len() {
            chunk_position += 1;

            for parser in parsers {
                if let Some((node, pos)) = parser(chunk, chunk_position - 1) {
                    if chunk_position - 1 != text_start {
                        result.push(fallback(&chunk[text_start..chunk_position - 1]));
                        text_start = pos;
                    }
                    chunk_position = pos;
                    result.push(node);
                }
            }
        }
        if text_start != chunk_position {
            result.push(fallback(&chunk[text_start..chunk_position]));
        }

        result
    }

    fn get_iterator(input: &str, start_position: usize) -> Enumerate<Chars> {
        let mut chars = input.chars().enumerate();
        if start_position != 0 {
            chars.nth(start_position - 1);
        }
        chars
    }
}

pub trait ParserPart {
    fn parse_part(&mut self, start: Vec<char>, end: Vec<char>) -> Option<usize>;
}

struct Matcher {
    index: usize,
    needle: Vec<char>,
}

impl Matcher {
    fn new(needle: Vec<char>) -> Self {
        Self { index: 0, needle }
    }

    fn is_match(&mut self, c: &char) -> bool {
        if self.needle.get(self.index) == Some(c) {
            self.index += 1;
            return true;
        }
        self.index = 0;
        false
    }

    fn is_done(&self) -> bool {
        self.index == self.needle.len()
    }
}

impl<'a> ParserPart for Enumerate<Chars<'a>> {
    fn parse_part(&mut self, start: Vec<char>, end: Vec<char>) -> Option<usize> {
        let mut start_matcher = Matcher::new(start);
        let mut start_matched = false;

        for (_, char) in self.by_ref() {
            if !start_matcher.is_match(&char) {
                break;
            }
            if start_matcher.is_done() {
                start_matched = true;
            }
        }

        if start_matched {
            let mut end_matcher = Matcher::new(end);
            let mut hard_stop_matcher = Matcher::new(vec!['\n', '\n']);
            for (index, char) in self.by_ref() {
                if end_matcher.is_match(&char) && end_matcher.is_done() {
                    return Some(index);
                } else if hard_stop_matcher.is_match(&char) && hard_stop_matcher.is_done() {
                    return None;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Matcher;

    use super::ParserPart;

    #[test]
    fn parse_part() {
        let mut c = "test of *italic**one more* statement".chars().enumerate();
        c.nth(7);
        assert_eq!(c.parse_part(vec!['*'], vec!['*']), Some(15));
        assert_eq!(c.parse_part(vec!['*'], vec!['*']), Some(25));
    }

    #[test]
    fn matcher() {
        let mut m = Matcher::new(vec!['*', '*']);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), true);
    }

    #[test]
    fn matcher_not_matched() {
        let mut m = Matcher::new(vec!['*', '*']);
        assert_eq!(m.is_match(&'a'), false);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'b'), false);
        assert_eq!(m.is_done(), false);
    }
}
