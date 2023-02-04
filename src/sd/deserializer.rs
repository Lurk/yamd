use std::{iter::Enumerate, str::Chars};

pub trait Branch<Nodes>
where
    Nodes: Node,
{
    fn new() -> Self;
    fn push<CanBeNode: Into<Nodes>>(&mut self, node: CanBeNode);
    fn from_vec(nodes: Vec<Nodes>) -> Self;
    fn get_maybe_nodes() -> Vec<MaybeNode<Nodes>>;
    fn get_fallback_node() -> Box<dyn Fn(&str) -> Nodes>;

    fn parse_branch(chunk: &str) -> Self
    where
        Self: Sized + Deserializer + Node,
    {
        let mut branch = Self::new();
        let mut chunk_position = 0;
        let mut text_start = 0;
        let fallback_node = Self::get_fallback_node();
        let maybe_nodes = Self::get_maybe_nodes();
        while chunk_position < chunk.len() {
            let slice = &chunk[chunk_position..];
            chunk_position += 1;
            for parser in &maybe_nodes {
                if let Some(node) = parser(slice) {
                    if text_start != chunk_position - 1 {
                        branch.push(fallback_node(&chunk[text_start..chunk_position - 1]));
                    }
                    branch.push(node);
                    chunk_position = branch.len() - branch.get_token_length();
                    text_start = chunk_position;
                }
            }
        }
        if text_start < chunk.len() {
            branch.push(fallback_node(&chunk[text_start..]));
        }

        branch
    }
}

pub type MaybeNode<BranchNodes> = Box<dyn Fn(&str) -> Option<BranchNodes>>;

pub trait Node {
    fn len(&self) -> usize;
    fn get_token_length(&self) -> usize;
    fn maybe_node<BranchNodes>(input: &str) -> Option<BranchNodes>
    where
        Self: Sized + Deserializer + Into<BranchNodes>,
    {
        if let Some(token) = Self::deserialize(input) {
            return Some(token.into());
        }
        None
    }
}

pub trait Deserializer {
    fn deserialize(input: &str) -> Option<Self>
    where
        Self: Sized;
}

struct Matcher<'token> {
    index: usize,
    token: &'token Vec<char>,
}

impl<'token> Matcher<'token> {
    fn new(token: &'token Vec<char>) -> Self {
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

pub struct Tokenizer<'input> {
    input: &'input str,
    chars: Enumerate<Chars<'input>>,
    hard_stop_token: Vec<char>,
}

impl<'input> Tokenizer<'input> {
    pub fn new(input: &'input str) -> Self {
        let chars = input.chars().enumerate();
        Tokenizer {
            chars,
            input,
            hard_stop_token: vec!['\n', '\n'],
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
        let mut c = Tokenizer::new("*italic**one more* statement");
        assert_eq!(c.get_token_body(vec!['*'], vec!['*']), Some("italic"));
        assert_eq!(c.get_token_body(vec!['*'], vec!['*']), Some("one more"));
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
