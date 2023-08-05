#[derive(Clone, Debug, PartialEq)]
pub enum Quantifiers {
    Once(char),
    RepeatTimes(usize, char),
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

    pub fn end(quantifiers_lengths: [usize; SIZE]) -> Self {
        Self {
            hit: true,
            end: true,
            length: quantifiers_lengths.iter().sum(),
            quantifiers_lengths,
        }
    }
}

pub struct Pattern<'token, const SIZE: usize> {
    index: usize,
    last_char: Option<char>,
    sequence: &'token [Quantifiers; SIZE],
    quantifiers_lengths: [usize; SIZE],
}

impl<'sequence, const SIZE: usize> Pattern<'sequence, SIZE> {
    pub fn new(sequence: &'sequence [Quantifiers; SIZE]) -> Self {
        Self {
            index: 0,
            last_char: None,
            sequence,
            quantifiers_lengths: [0; SIZE],
        }
    }

    fn increment_quantifier_length(&mut self, index: usize) {
        if let Some(count) = self.quantifiers_lengths.get_mut(index) {
            *count += 1;
        };
    }

    fn next_index(&mut self, c: &char, index: usize) -> Option<usize> {
        let current_pattern_length = self.get_quantifier_length(index).unwrap_or(&0);
        return match self.sequence.get(index) {
            Some(Quantifiers::Once(p)) if p == c => {
                self.increment_quantifier_length(index);
                Some(index + 1)
            }
            Some(Quantifiers::RepeatTimes(length, p))
                if (p == c && current_pattern_length + 1 < *length) =>
            {
                self.increment_quantifier_length(index);

                Some(index)
            }
            Some(Quantifiers::RepeatTimes(length, p))
                if (p == c && current_pattern_length + 1 == *length) =>
            {
                self.increment_quantifier_length(index);

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
            self.last_char = Some(*c);
            return self.create_state(true);
        } else if self.index == 1 && self.last_char == Some(*c) {
            return self.create_state(true);
        } else if let Some(new_index) = self.next_index(c, 0) {
            self.reset();
            self.increment_quantifier_length(0);
            self.index = new_index;
            self.last_char = Some(*c);
            return self.create_state(true);
        }
        self.reset();
        self.create_state(false)
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.last_char = None;
        self.quantifiers_lengths = [0; SIZE];
    }

    fn create_state(&mut self, hit: bool) -> PatternState<SIZE> {
        if self.index == self.sequence.len() {
            let state = PatternState::end(self.quantifiers_lengths);
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
        assert_eq!(m.check_character(&'*'), PatternState::end([1, 1]));
    }

    #[test]
    fn matcher_not_matched() {
        let mut m = Pattern::new(&[Once('*'), Once('*')]);
        assert_eq!(m.check_character(&'a'), PatternState::new(false));
        assert_eq!(m.check_character(&'b'), PatternState::new(false));
    }

    #[test]
    fn pattern_exact_repeat_happy_path() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' '), Once('-')]);
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&'-'), PatternState::end([2, 1]));
    }

    #[test]
    fn pattern_starts_with_exact_repeat() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' '), Once('-')]);
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&'-'), PatternState::end([2, 1]));
    }

    #[test]
    fn pattern_starts_with_0_exact_repeat() {
        let mut m = Pattern::new(&[RepeatTimes(0, ' '), Once('-')]);
        assert_eq!(m.check_character(&'-'), PatternState::end([0, 1]));
    }

    #[test]
    fn pattern_ends_with_exact_repeat() {
        let mut m = Pattern::new(&[Once('-'), RepeatTimes(2, ' ')]);
        assert_eq!(m.check_character(&'-'), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::end([1, 2]));
        assert_eq!(m.check_character(&' '), PatternState::new(false));
    }

    #[test]
    fn repeat_times_pattern() {
        let mut m = Pattern::new(&[RepeatTimes(2, ' ')]);
        assert_eq!(m.check_character(&' '), PatternState::new(true));
        assert_eq!(m.check_character(&' '), PatternState::end([2]));
        assert_eq!(m.check_character(&' '), PatternState::new(true));
    }

    #[test]
    fn partialy_repeating_pattern() {
        let mut m = Pattern::new(&[Once('a'), RepeatTimes(3, 'b')]);
        assert_eq!(m.check_character(&'a'), PatternState::new(true));
        assert_eq!(m.check_character(&'b'), PatternState::new(true));
        assert_eq!(m.check_character(&'b'), PatternState::new(true));
        assert_eq!(m.check_character(&'a'), PatternState::new(true));
    }

    #[test]
    fn when_reset_repeating_pattern() {
        let mut m = Pattern::new(&[Once('\n'), RepeatTimes(3, '\\')]);
        assert_eq!(m.check_character(&'\n'), PatternState::new(true));
        assert_eq!(m.check_character(&'\\'), PatternState::new(true));
        assert_eq!(m.check_character(&'\\'), PatternState::new(true));
        assert_eq!(m.check_character(&'\n'), PatternState::new(true));
        assert_eq!(m.check_character(&'\\'), PatternState::new(true));
        assert_eq!(m.check_character(&'\\'), PatternState::new(true));
        assert_eq!(m.check_character(&'\\'), PatternState::end([1, 3]));
    }
}
