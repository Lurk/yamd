#[derive(Clone)]
pub enum Pattern {
    Once(char),
    ZerroOrMore(char),
    RepeatTimes(usize, char),
}

struct Matcher<'token> {
    index: usize,
    token: &'token Vec<Pattern>,
    length: usize,
    pattern_lengths: Vec<usize>,
}

impl<'token> Matcher<'token> {
    fn new(token: &'token Vec<Pattern>) -> Self {
        Self {
            index: 0,
            token,
            length: 0,
            pattern_lengths: vec![0; token.len()],
        }
    }

    fn new_index(&mut self, c: &char, index: usize) -> Option<usize> {
        let current_pattern_length = self.get_pattern_lengh(index).unwrap_or(&0);
        return match self.token.get(index) {
            Some(Pattern::Once(p)) if p == c => {
                if let Some(count) = self.pattern_lengths.get_mut(index) {
                    *count += 1;
                };
                Some(index + 1)
            }
            Some(Pattern::ZerroOrMore(p)) if p == c => {
                if let Some(count) = self.pattern_lengths.get_mut(index) {
                    *count += 1;
                };
                Some(index)
            }
            Some(Pattern::ZerroOrMore(p)) if p != c => self.new_index(c, index + 1),
            Some(Pattern::RepeatTimes(length, p))
                if (p == c && current_pattern_length < length) =>
            {
                if let Some(count) = self.pattern_lengths.get_mut(index) {
                    *count += 1;
                };

                Some(index)
            }
            Some(Pattern::RepeatTimes(length, p))
                if (p != c && current_pattern_length == length) =>
            {
                self.new_index(c, index + 1)
            }
            _ => None,
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
        self.pattern_lengths = vec![0; self.token.len()];
        false
    }

    fn is_done(&self) -> bool {
        self.index == self.token.len()
    }

    fn get_pattern_lengh(&self, index: usize) -> Option<&usize> {
        self.pattern_lengths.get(index)
    }
}

pub struct Tokenizer<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> Tokenizer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }

    pub fn get_body_start_position(&self, start_token: Vec<Pattern>) -> Option<usize> {
        let add = if self.position == 0 { 0 } else { 1 };
        if start_token.is_empty() {
            return Some(self.position + add);
        } else {
            let mut start_matcher = Matcher::new(&start_token);
            for char in self.input.chars().skip(self.position + add) {
                if !start_matcher.is_match(&char) {
                    break;
                }
                if start_matcher.is_done() {
                    return Some(start_matcher.length + self.position + add);
                }
            }
        }
        None
    }

    pub fn get_token_body(
        &mut self,
        start_token: Vec<Pattern>,
        end_token: Vec<Pattern>,
    ) -> Option<&str> {
        self.get_token_body_with_options(start_token, end_token, false)
    }

    pub fn get_token_body_with_options(
        &mut self,
        start_token: Vec<Pattern>,
        end_token: Vec<Pattern>,
        match_end_of_input: bool,
    ) -> Option<&str> {
        if let Some(body_start) = self.get_body_start_position(start_token) {
            let mut end_matcher = Matcher::new(&end_token);
            for (index, char) in self.input.chars().enumerate().skip(body_start) {
                self.position = index;
                if end_matcher.is_match(&char) && end_matcher.is_done() {
                    return Some(&self.input[body_start..index - (end_matcher.length - 1)]);
                } else if match_end_of_input && index == self.input.len() - 1 {
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
        Pattern::{Once, RepeatTimes, ZerroOrMore},
        Tokenizer,
    };

    #[test]
    fn parse_part() {
        let mut c = Tokenizer::new("*italic**one more* statement");
        assert_eq!(
            c.get_token_body(vec![Once('*')], vec![Once('*')]),
            Some("italic")
        );
        assert_eq!(
            c.get_token_body(vec![Once('*')], vec![Once('*')]),
            Some("one more")
        );
    }

    #[test]
    fn matcher() {
        let token = &vec![Once('*'), Once('*')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'*'), true);
        assert_eq!(m.is_done(), true);
    }

    #[test]
    fn matcher_not_matched() {
        let token = &vec![Once('*'), Once('*')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'a'), false);
        assert_eq!(m.is_done(), false);
        assert_eq!(m.is_match(&'b'), false);
        assert_eq!(m.is_done(), false);
    }

    #[test]
    fn pattern_repeat() {
        let token = &vec![ZerroOrMore(' '), Once('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_done(), true);
        assert_eq!(m.length, 3);
        assert_eq!(m.get_pattern_lengh(0), Some(&2));
        assert_eq!(m.get_pattern_lengh(1), Some(&1));
        assert_eq!(m.is_match(&'-'), false);
        assert_eq!(m.length, 0);
        assert_eq!(m.is_done(), false);
    }

    #[test]
    fn pattern_repeat_zero() {
        let token = &vec![ZerroOrMore(' '), Once('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_done(), true);
        assert_eq!(m.get_pattern_lengh(0), Some(&0));
        assert_eq!(m.get_pattern_lengh(1), Some(&1));
        assert_eq!(m.is_match(&'-'), false);
    }

    #[test]
    fn pattern_exact_repeat_happy_path() {
        let token = &vec![RepeatTimes(2, ' '), Once('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_done(), true);
    }

    #[test]
    fn pattern_starts_with_exact_repeat() {
        let token = &vec![RepeatTimes(2, ' '), Once('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), false)
    }

    #[test]
    fn pattern_ends_with_exact_repeat() {
        let token = &vec![Once('-'), RepeatTimes(2, ' ')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&'-'), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), false);
    }

    #[test]
    fn new_index() {
        let token = &vec![ZerroOrMore(' '), Once('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.new_index(&' ', 0), Some(0));
        assert_eq!(m.new_index(&'-', 1), Some(2));
        assert_eq!(m.new_index(&'d', 0), None);
        assert_eq!(m.new_index(&'d', 1), None);
    }
    #[test]
    fn pattern_repeat_is_not_matched() {
        let token = &vec![ZerroOrMore(' '), Once('-')];
        let mut m = Matcher::new(token);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&' '), true);
        assert_eq!(m.is_match(&'a'), false);
        assert_eq!(m.is_done(), false);
    }
}
