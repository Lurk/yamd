#[derive(Clone, Debug, PartialEq)]
pub enum Quantifiers {
    Once(char),
    ZeroOrMore(char),
    RepeatTimes(usize, char),
}

pub struct Pattern<'token, const SIZE: usize> {
    index: usize,
    sequence: &'token [Quantifiers; SIZE],
    pub length: usize,
    quantifiers_lengths: [usize; SIZE],
}

impl<'sequence, const SIZE: usize> Pattern<'sequence, SIZE> {
    pub fn new(sequence: &'sequence [Quantifiers; SIZE]) -> Self {
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

    pub fn check_character(&mut self, c: &char) -> bool {
        if let Some(new_index) = self.next_index(c, self.index) {
            self.index = new_index;
            self.length += 1;
            return true;
        }
        self.reset();
        false
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.length = 0;
        self.quantifiers_lengths = [0; SIZE];
    }

    pub fn is_end_of_sequence(&self) -> bool {
        self.index == self.sequence.len()
    }

    pub fn get_quantifier_length(&self, index: usize) -> Option<&usize> {
        self.quantifiers_lengths.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::pattern::{Pattern, Quantifiers::*};

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
    fn repeat_times_pattern() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' ')]);
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
