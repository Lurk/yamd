use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub trait Branch<Node: std::fmt::Debug> {
    fn new() -> Self;
    fn push<N: Into<N>>(&mut self, node: N);
    fn from_vec(nodes: Vec<Node>) -> Self;
    fn get_maybe_nodes() -> Vec<MaybeNode<Node>>;
    fn get_fallback() -> Box<dyn Fn(&str) -> Node>;

    fn parse_branch(chunk: &str) -> Self
    where
        Self: Sized + Deserializer,
    {
        let mut result = Self::new();
        let mut chunk_position = 0;
        let mut text_start = 0;
        let fallback = Self::get_fallback();
        while chunk_position < chunk.len() {
            chunk_position += 1;
            for parser in Self::get_maybe_nodes() {
                if let Some((node, pos)) = parser(chunk, chunk_position - 1) {
                    if chunk_position - 1 != text_start {
                        result.push(fallback(&chunk[text_start..chunk_position - 1]));
                    }
                    chunk_position = pos;
                    text_start = pos;
                    result.push(node);
                }
            }
        }
        if text_start != chunk_position {
            result.push(fallback(&chunk[text_start..chunk_position]));
        }

        result
    }
}

pub type MaybeNode<Node> = Box<dyn Fn(&str, usize) -> Option<(Node, usize)>>;

pub trait Node {
    fn len(&self) -> usize;
    fn maybe_node<Node>(input: &str, start_position: usize) -> Option<(Node, usize)>
    where
        Self: Sized + Deserializer + Into<Node>,
    {
        if let Some((token, pos)) = Self::deserialize(input, start_position) {
            return Some((token.into(), pos));
        }
        None
    }
}

pub trait Deserializer {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)>
    where
        Self: Sized;
}

struct Matcher<'a> {
    index: usize,
    token: &'a Vec<char>,
}

impl<'a> Matcher<'a> {
    fn new(token: &'a Vec<char>) -> Self {
        Self { index: 0, token }
    }

    fn is_match(&mut self, c: &char) -> bool {
        if self.token.get(self.index) == Some(c) {
            self.index += 1;
            return true;
        }
        self.index = 0;
        false
    }

    fn is_done(&self) -> bool {
        self.index == self.token.len()
    }
}

pub struct Tokenizer<'a> {
    input: &'a str,
    chars: Peekable<Enumerate<Chars<'a>>>,
    hard_stop_token: Vec<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str, start_position: usize) -> Self {
        let mut chars = input.chars().enumerate().peekable();
        if start_position != 0 {
            chars.nth(start_position - 1);
        }
        Tokenizer {
            chars,
            input,
            hard_stop_token: vec!['\n', '\n'],
        }
    }

    pub fn get_next_position(&mut self) -> usize {
        match self.chars.peek() {
            Some((index, _)) => *index,
            None => self.input.len(),
        }
    }

    pub fn get_token_body(&mut self, start_token: Vec<char>, end_token: Vec<char>) -> Option<&str> {
        let mut start_matcher = Matcher::new(&start_token);
        let mut body_start: Option<usize> = None;

        for (index, char) in self.chars.by_ref() {
            if !start_matcher.is_match(&char) {
                break;
            }
            if start_matcher.is_done() {
                body_start = Some(index + 1);
                break;
            }
        }

        if let Some(body_start) = body_start {
            let mut end_matcher = Matcher::new(&end_token);
            let mut hard_stop_matcher = Matcher::new(&self.hard_stop_token);
            for (index, char) in self.chars.by_ref() {
                if end_matcher.is_match(&char) && end_matcher.is_done() {
                    return Some(&self.input[body_start..index - (end_token.len() - 1)]);
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
    use crate::sd::deserializer::{Matcher, Tokenizer};

    #[test]
    fn parse_part() {
        let mut c = Tokenizer::new("test of *italic**one more* statement", 8);
        assert_eq!(c.get_token_body(vec!['*'], vec!['*']), Some("italic"));
        assert_eq!(c.get_next_position(), 16);
        assert_eq!(c.get_token_body(vec!['*'], vec!['*']), Some("one more"));
        assert_eq!(c.get_next_position(), 26);
    }

    #[test]
    fn matcher() {
        let token = &vec!['*', '*'];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), true);
    }

    #[test]
    fn matcher_not_matched() {
        let token = &vec!['*', '*'];
        let mut m = Matcher::new(token);

        assert_eq!(m.is_match(&'a'), false);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'b'), false);
        assert_eq!(m.is_done(), false);
    }
}
