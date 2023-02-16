use std::{iter::Enumerate, str::Chars};

#[derive(Clone)]
pub enum Pattern {
    Exact(char),
    Repeat(char),
    ExactRepeat(usize, char),
}

struct Matcher<'token> {
    index: usize,
    token: &'token Vec<Pattern>,
    length: usize,
    exact_repeat_length: usize,
}

impl<'token> Matcher<'token> {
    fn new(token: &'token Vec<Pattern>) -> Self {
        Self {
            index: 0,
            token,
            length: 0,
            exact_repeat_length: 0,
        }
    }

    fn new_index(&mut self, c: &char, index: usize) -> Option<usize> {
        return match self.token.get(index) {
            Some(Pattern::Exact(p)) if p == c => {
                self.exact_repeat_length = 0;
                Some(index + 1)
            }
            Some(Pattern::Repeat(p)) if p == c => {
                self.exact_repeat_length = 0;
                Some(index)
            }
            Some(Pattern::Repeat(p)) if p != c => {
                self.exact_repeat_length = 0;
                self.new_index(c, index + 1)
            }
            Some(Pattern::ExactRepeat(length, p))
                if (p == c && self.exact_repeat_length < *length) =>
            {
                self.exact_repeat_length += 1;
                Some(index)
            }
            Some(Pattern::ExactRepeat(length, p))
                if (p != c && self.exact_repeat_length == *length) =>
            {
                self.exact_repeat_length = 0;
                self.new_index(c, index + 1)
            }
            _ => {
                self.exact_repeat_length = 0;
                None
            }
        };
    }
    fn is_match(&mut self, c: &char) -> bool {
        if let Some(new_index) = self.new_index(c, self.index) {
            self.index = new_index;
            self.length += 1;
            return true;
        }
        self.index = 0;
        self.length = 0;
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
    position: usize,
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
            position: 0,
        }
    }

    pub fn get_body_start(&mut self, start_token: Vec<Pattern>) -> Option<usize> {
        if start_token.is_empty() {
            let add = if self.position == 0 { 0 } else { 1 };
            Some(self.position + add)
        } else {
            let mut start_matcher = Matcher::new(&start_token);
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
        }
    }

    pub fn get_token_body(
        &mut self,
        start_token: Vec<Pattern>,
        end_token: Vec<Pattern>,
    ) -> Option<&str> {
        if let Some(body_start) = self.get_body_start(start_token) {
            let mut end_matcher = Matcher::new(&end_token);
            for (index, char) in self.chars.by_ref() {
                self.position = index;
                if end_matcher.is_match(&char) && end_matcher.is_done() {
                    return Some(&self.input[body_start..index - (end_matcher.length - 1)]);
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
    use crate::sd::tokenizer::{
        Matcher,
        Pattern::{Exact, ExactRepeat, Repeat},
        Tokenizer,
    };

    #[test]
    fn parse_part() {
        let mut c = Tokenizer::new("*italic**one more* statement");
        assert_eq!(
            c.get_token_body(vec![Exact('*')], vec![Exact('*')]),
            Some("italic")
        );
        assert_eq!(
            c.get_token_body(vec![Exact('*')], vec![Exact('*')]),
            Some("one more")
        );
    }

    #[test]
    fn matcher() {
        let token = &vec![Exact('*'), Exact('*')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), true);
    }

    #[test]
    fn matcher_not_matched() {
        let token = &vec![Exact('*'), Exact('*')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'a'), false);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'b'), false);
        assert_eq!(m.is_done(), false);
    }

    #[test]
    fn pattern_repeat() {
        let token = &vec![Repeat(' '), Exact('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_done(), true);
        assert_eq!(m.length, 3);
        assert_eq!(m.is_match(&'-'), false);
        assert_eq!(m.length, 0);
        assert_eq!(m.is_done(), false);
    }

    #[test]
    fn pattern_repeat_zero() {
        let token = &vec![Repeat(' '), Exact('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_done(), true);
        assert_eq!(m.is_match(&'-'), false);
    }

    #[test]
    fn pattern_exact_repeat_happy_path() {
        let token = &vec![ExactRepeat(2, ' '), Exact('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_done(), true);
    }

    #[test]
    fn pattern_starts_with_exact_repeat() {
        let token = &vec![ExactRepeat(2, ' '), Exact('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), false)
    }

    #[test]
    fn pattern_ends_with_exact_repeat() {
        let token = &vec![Exact('-'), ExactRepeat(2, ' ')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), false);
    }

    #[test]
    fn new_index() {
        let token = &vec![Repeat(' '), Exact('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.new_index(&' ', 0), Some(0));
        assert_eq!(m.new_index(&'-', 1), Some(2));
        assert_eq!(m.new_index(&'d', 0), None);
        assert_eq!(m.new_index(&'d', 1), None);
    }
    #[test]
    fn pattern_repeat_is_not_matched() {
        let token = &vec![Repeat(' '), Exact('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&'a'), false);
        assert_eq!(m.is_done(), false);
    }
}
