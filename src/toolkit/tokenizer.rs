#[derive(Clone, Debug)]
pub enum Quantifiers {
    Once(char),
    ZeroOrMore(char),
    RepeatTimes(usize, char),
}

struct Pattern<'token, const SIZE: usize> {
    index: usize,
    sequence: &'token [Quantifiers; SIZE],
    length: usize,
    quantifiers_lengths: [usize; SIZE],
}

impl<'sequence, const SIZE: usize> Pattern<'sequence, SIZE> {
    fn new(sequence: &'sequence [Quantifiers; SIZE]) -> Self {
        Self {
            index: 0,
            sequence,
            length: 0,
            quantifiers_lengths: [0; SIZE],
        }
    }

    fn next_index(&mut self, c: &char, index: usize) -> Option<usize> {
        let current_pattern_length = self.get_quantifier_length(index).unwrap_or(&0);
        return match self.sequence.get(index) {
            Some(Quantifiers::Once(p)) if p == c => {
                if let Some(count) = self.quantifiers_lengths.get_mut(index) {
                    *count += 1;
                };
                Some(index + 1)
            }
            Some(Quantifiers::ZeroOrMore(p)) if p == c => {
                if let Some(count) = self.quantifiers_lengths.get_mut(index) {
                    *count += 1;
                };
                Some(index)
            }
            Some(Quantifiers::ZeroOrMore(p)) if p != c => self.next_index(c, index + 1),
            Some(Quantifiers::RepeatTimes(length, p))
                if (p == c && current_pattern_length + 1 < *length) =>
            {
                if let Some(count) = self.quantifiers_lengths.get_mut(index) {
                    *count += 1;
                };

                Some(index)
            }
            Some(Quantifiers::RepeatTimes(length, p))
                if (p == c && current_pattern_length + 1 == *length) =>
            {
                Some(index + 1)
            }
            Some(Quantifiers::RepeatTimes(length, _)) if (*length == 0) => {
                self.next_index(c, index + 1)
            }
            _ => None,
        };
    }
    fn check_character(&mut self, c: &char) -> bool {
        if let Some(new_index) = self.next_index(c, self.index) {
            self.index = new_index;
            self.length += 1;
            return true;
        }
        self.index = 0;
        self.length = 0;
        self.quantifiers_lengths = [0; SIZE];
        false
    }

    fn is_end_of_sequence(&self) -> bool {
        self.index == self.sequence.len()
    }

    fn get_quantifier_length(&self, index: usize) -> Option<&usize> {
        self.quantifiers_lengths.get(index)
    }
}

pub struct Matcher<'input> {
    input: &'input str,
    position: usize,
}

pub struct Match<'input>{
    start_token:&'input str,
    body:&'input str,
    end_token: &'input str
}

impl<'input> Matcher<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }

    pub fn get_node_body_start_position<const START_SEQUENCE_SIZE: usize>(
        &self,
        start_token: &'input [Quantifiers; START_SEQUENCE_SIZE],
    ) -> Option<usize> {
        let add = if self.position == 0 { 0 } else { 1 };
        if start_token.is_empty() {
            return Some(self.position + add);
        } else {
            let mut pattern = Pattern::new(start_token);
            for char in self.input.chars().skip(self.position + add) {
                if !pattern.check_character(&char) {
                    break;
                }
                if pattern.is_end_of_sequence() {
                    return Some(pattern.length + self.position + add);
                }
            }
        }
        None
    }

    pub fn get_node_body<const START_SEQUENCE_SIZE: usize, const END_SEQUENCE_SIZE: usize>(
        &mut self,
        start_sequence: &'input [Quantifiers; START_SEQUENCE_SIZE],
        end_sequence: &'input [Quantifiers; END_SEQUENCE_SIZE],
    ) -> Option<&str> {
        self.get_node_body_with_end_of_input(start_sequence, end_sequence, false)
    }

    pub fn get_node_body_with_end_of_input<
        const START_SEQUENCE_SIZE: usize,
        const END_SEQUENCE_SIZE: usize,
    >(
        &mut self,
        start_sequence: &'input [Quantifiers; START_SEQUENCE_SIZE],
        end_sequence: &'input [Quantifiers; END_SEQUENCE_SIZE],
        match_end_of_input: bool,
    ) -> Option<&str> {
        if let Some(body_start) = self.get_node_body_start_position(start_sequence) {
            let mut end_matcher = Pattern::new(end_sequence);
            for (index, char) in self.input.chars().enumerate().skip(body_start) {
                if end_matcher.check_character(&char) && end_matcher.is_end_of_sequence() {
                    self.position = index;
                    return Some(&self.input[body_start..index - (end_matcher.length - 1)]);
                } else if match_end_of_input && index == self.input.len() - 1 {
                    self.position = index;
                    return Some(&self.input[body_start..]);
                }
            }
        }

        None
    }

    pub fn get_rest(&self) -> &'input str {
        &self.input[self.position + 1..]
    }
}
#[cfg(test)]
mod tests {
    use crate::toolkit::tokenizer::{
        Pattern,
        Quantifiers::{Once, RepeatTimes, ZeroOrMore},
        Matcher,
    };

    #[test]
    fn parse_part() {
        let mut c = Matcher::new("*italic**one more* statement");
        assert_eq!(c.get_node_body(&[Once('*')], &[Once('*')]), Some("italic"));
        assert_eq!(
            c.get_node_body(&[Once('*')], &[Once('*')]),
            Some("one more")
        );
        assert_eq!(c.get_rest(), " statement");
    }

    #[test]
    fn matcher() {
        let mut m = Pattern::new(&[Once('*'), Once('*')]);
        assert_eq!(m.check_character(&'*'), true);
        assert_eq!(m.is_end_of_sequence(), false);
        assert_eq!(m.check_character(&'*'), true);
        assert_eq!(m.is_end_of_sequence(), true);
    }

    #[test]
    fn matcher_not_matched() {
        let mut m = Pattern::new(&[Once('*'), Once('*')]);
        assert_eq!(m.check_character(&'a'), false);
        assert_eq!(m.is_end_of_sequence(), false);
        assert_eq!(m.check_character(&'b'), false);
        assert_eq!(m.is_end_of_sequence(), false);
    }

    #[test]
    fn pattern_repeat() {
        let mut m = Pattern::new(&[ZeroOrMore(' '), Once('-')]);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&'-'), true);
        assert_eq!(m.is_end_of_sequence(), true);
        assert_eq!(m.length, 3);
        assert_eq!(m.get_quantifier_length(0), Some(&2));
        assert_eq!(m.get_quantifier_length(1), Some(&1));
        assert_eq!(m.check_character(&'-'), false);
        assert_eq!(m.length, 0);
        assert_eq!(m.is_end_of_sequence(), false);
    }

    #[test]
    fn pattern_repeat_zero() {
        let mut m = Pattern::new(&[ZeroOrMore(' '), Once('-')]);
        assert_eq!(m.check_character(&'-'), true);
        assert_eq!(m.is_end_of_sequence(), true);
        assert_eq!(m.get_quantifier_length(0), Some(&0));
        assert_eq!(m.get_quantifier_length(1), Some(&1));
        assert_eq!(m.check_character(&'-'), false);
    }

    #[test]
    fn pattern_exact_repeat_happy_path() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' '), Once('-')]);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&'-'), true);
        assert_eq!(m.is_end_of_sequence(), true);
    }

    #[test]
    fn pattern_starts_with_exact_repeat() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' '), Once('-')]);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&' '), false)
    }

    #[test]
    fn pattern_starts_with_0_exact_repeat() {
        let mut m = Pattern::new(&[RepeatTimes(0, ' '), Once('-')]);
        assert_eq!(m.check_character(&'-'), true);
        assert_eq!(m.is_end_of_sequence(), true);
    }

    #[test]
    fn pattern_ends_with_exact_repeat() {
        let mut m = Pattern::new(&[Once('-'), RepeatTimes(2, ' ')]);
        assert_eq!(m.check_character(&'-'), true);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.is_end_of_sequence(), true);
        assert_eq!(m.check_character(&' '), false);
    }

    #[test]
    fn new_index() {
        let mut m = Pattern::new(&[ZeroOrMore(' '), Once('-')]);
        assert_eq!(m.next_index(&' ', 0), Some(0));
        assert_eq!(m.next_index(&'-', 1), Some(2));
        assert_eq!(m.next_index(&'d', 0), None);
        assert_eq!(m.next_index(&'d', 1), None);
    }
    #[test]
    fn pattern_repeat_is_not_matched() {
        let mut m = Pattern::new(&[ZeroOrMore(' '), Once('-')]);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&' '), true);
        assert_eq!(m.check_character(&'a'), false);
        assert_eq!(m.is_end_of_sequence(), false);
    }
}
