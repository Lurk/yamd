#[derive(Clone, Debug, PartialEq)]
pub enum Quantifiers {
    Once(char),
    ZeroOrMore(char),
    RepeatTimes(usize, char),
}

pub struct Pattern<'token, const SIZE: usize> {
    index: usize,
    sequence: &'token [Quantifiers; SIZE],
    length: usize,
    quantifiers_lengths: [usize; SIZE],
}

#[derive(Debug, PartialEq)]
pub struct PatternState<const SIZE: usize> {
    pub hit: bool,
    pub end: bool,
    pub length: usize,
    quantifiers_lengths: [usize; SIZE],
}

impl<const SIZE: usize> PatternState<SIZE> {
    pub fn new(hit: bool) -> Self {
        Self {
            hit,
            end: false,
            length: 0,
            quantifiers_lengths: [0; SIZE],
        }
    }

    pub fn end(length: usize, quantifiers_lengths: [usize; SIZE]) -> Self {
        Self {
            hit: true,
            end: true,
            length,
            quantifiers_lengths,
        }
    }
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
                if let Some(count) = self.quantifiers_lengths.get_mut(index) {
                    *count += 1;
                };

                Some(index + 1)
            }
            Some(Quantifiers::RepeatTimes(length, _)) if (*length == 0) => {
                self.next_index(c, index + 1)
            }
            _ => None,
        };
    }

    pub fn check_character(&mut self, c: &char) -> PatternState<SIZE> {
        if let Some(new_index) = self.next_index(c, self.index) {
            self.index = new_index;
            self.length += 1;
            return self.create_state(true);
        }
        self.reset();
        self.create_state(false)
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.length = 0;
        self.quantifiers_lengths = [0; SIZE];
    }

    fn create_state(&mut self, hit: bool) -> PatternState<SIZE> {
        if self.index == self.sequence.len() {
            let state = PatternState::end(self.length, self.quantifiers_lengths);
            self.reset();
            return state;
        }
        PatternState::new(hit)
    }

    fn get_quantifier_length(&self, index: usize) -> Option<&usize> {
        self.quantifiers_lengths.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::pattern::{Pattern, PatternState, Quantifiers::*};

    #[test]
    fn matcher() {
        let mut m = Pattern::new(&[Once('*'), Once('*')]);
        assert_eq!(m.check_character(&'*'), PatternState::new(true));
        assert_eq!(m.check_character(&'*'), PatternState::end(2, [1, 1]));
    }

    #[test]
    fn matcher_not_matched() {
        let mut m = Pattern::new(&[Once('*'), Once('*')]);
        assert_eq!(m.check_character(&'a'), PatternState::new(false));
        assert_eq!(m.check_character(&'b'), PatternState::new(false));
    }

    #[test]
    fn pattern_repeat() {
        let mut m = Pattern::new(&[ZeroOrMore(' '), Once('-')]);
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&'-'), PatternState::end(3, [2, 1]));
        assert_eq!(m.check_character(&'-'), PatternState::end(1, [0, 1]));
    }

    #[test]
    fn pattern_repeat_zero() {
        let mut m = Pattern::new(&[ZeroOrMore(' '), Once('-')]);
        assert_eq!(m.check_character(&'-'), PatternState::end(1, [0, 1]));
        assert_eq!(m.check_character(&'-'), PatternState::end(1, [0, 1]));
    }

    #[test]
    fn pattern_exact_repeat_happy_path() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' '), Once('-')]);
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&'-'), PatternState::end(3, [2, 1]));
    }

    #[test]
    fn pattern_starts_with_exact_repeat() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' '), Once('-')]);
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(false));
    }

    #[test]
    fn pattern_starts_with_0_exact_repeat() {
        let mut m = Pattern::new(&[RepeatTimes(0, ' '), Once('-')]);
        assert_eq!(m.check_character(&'-'), PatternState::end(1, [0, 1]));
    }

    #[test]
    fn pattern_ends_with_exact_repeat() {
        let mut m = Pattern::new(&[Once('-'), RepeatTimes(2, ' ')]);
        assert_eq!(m.check_character(&'-'), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::end(3, [1, 2]));
        assert_eq!(m.check_character(&' '), PatternState::new(false));
    }

    #[test]
    fn repeat_times_pattern() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' ')]);
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::end(2, [2]));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
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
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&'a'), PatternState::new(false));
    }
}
