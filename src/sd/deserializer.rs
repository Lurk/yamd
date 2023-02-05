use std::{iter::Enumerate, str::Chars};

pub trait Branch<BranchNodes>
where
    BranchNodes: Node,
{
    fn new() -> Self;
    fn push<CanBeNode: Into<BranchNodes>>(&mut self, node: CanBeNode);
    fn from_vec(nodes: Vec<BranchNodes>) -> Self;
    fn get_maybe_nodes() -> Vec<MaybeNode<BranchNodes>>;
    fn get_fallback_node() -> FallbackNode<BranchNodes>;
    fn get_outer_token_length(&self) -> usize;

    fn parse_branch(input: &str) -> Self
    where
        Self: Sized + Deserializer + Node,
    {
        let mut branch = Self::new();
        let mut current_position = 0;
        let mut fallback_position = 0;
        let fallback_node = Self::get_fallback_node();
        let maybe_nodes = Self::get_maybe_nodes();
        while current_position < input.len() {
            let slice = &input[current_position..];
            current_position += 1;
            for parser in &maybe_nodes {
                if let Some(node) = parser(slice) {
                    if fallback_position != current_position - 1 {
                        branch.push(fallback_node(
                            &input[fallback_position..current_position - 1],
                        ));
                    }
                    branch.push(node);
                    current_position = branch.len() - branch.get_outer_token_length();
                    fallback_position = current_position;
                }
            }
        }
        if fallback_position < input.len() {
            branch.push(fallback_node(&input[fallback_position..]));
        }

        branch
    }
}

pub type MaybeNode<BranchNodes> = Box<dyn Fn(&str) -> Option<BranchNodes>>;
pub type FallbackNode<BranchNodes> = Box<dyn Fn(&str) -> BranchNodes>;

pub trait Node {
    fn len(&self) -> usize;
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
    match_end_of_input: bool,
}

impl<'input> Tokenizer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self::new_with_match_end_of_input(input, false)
    }

    pub fn new_with_match_end_of_input(input: &'input str, match_end_of_input: bool) -> Self {
        let chars = input.chars().enumerate();
        Tokenizer {
            chars,
            input,
            match_end_of_input,
        }
    }

    pub fn get_token_body(&mut self, start_token: Vec<char>, end_token: Vec<char>) -> Option<&str> {
        let mut start_matcher = Matcher::new(&start_token);
        let body_start: Option<usize> = if start_token.is_empty() {
            Some(0)
        } else {
            let mut body_start = None;
            for (index, char) in self.chars.by_ref() {
                if !start_matcher.is_match(&char) {
                    break;
                }
                if start_matcher.is_done() {
                    body_start = Some(index + 1);
                    break;
                }
            }
            body_start
        };

        if let Some(body_start) = body_start {
            let mut end_matcher = Matcher::new(&end_token);
            for (index, char) in self.chars.by_ref() {
                if end_matcher.is_match(&char) && end_matcher.is_done() {
                    return Some(&self.input[body_start..index - (end_token.len() - 1)]);
                } else if self.match_end_of_input && index == self.input.len() - 1 {
                    return Some(&self.input[body_start..]);
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
