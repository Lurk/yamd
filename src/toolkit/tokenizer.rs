#[derive(Clone, Debug, PartialEq)]
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
        self.reset();
        false
    }

    fn reset(&mut self) {
        self.index = 0;
        self.length = 0;
        self.quantifiers_lengths = [0; SIZE];
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

#[derive(Debug, PartialEq)]
pub struct Match<'input> {
    pub start_token: &'input str,
    pub body: &'input str,
    pub end_token: &'input str,
}

impl<'input> Matcher<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }

    fn are_sequences_equal<const FIRST_SEQUENCE_SIZE: usize, const SECOND_SEQUENCE_SIZE: usize>(
        a: &[Quantifiers; FIRST_SEQUENCE_SIZE],
        b: &[Quantifiers; SECOND_SEQUENCE_SIZE],
    ) -> bool {
        if FIRST_SEQUENCE_SIZE == SECOND_SEQUENCE_SIZE {
            return a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b);
        }
        false
    }

    pub fn get_match<const START_SEQUENCE_SIZE: usize, const END_SEQUENCE_SIZE: usize>(
        &mut self,
        start_sequence: &'input [Quantifiers; START_SEQUENCE_SIZE],
        end_sequence: &'input [Quantifiers; END_SEQUENCE_SIZE],
        match_end_of_input: bool,
    ) -> Option<Match<'input>> {
        if !match_end_of_input
            && START_SEQUENCE_SIZE > 0
            && END_SEQUENCE_SIZE == START_SEQUENCE_SIZE
            && !Self::are_sequences_equal(start_sequence, end_sequence)
        {
            return self.get_balanced_match(start_sequence, end_sequence);
        }
        self.get_unbalanced_match(start_sequence, end_sequence, match_end_of_input)
    }

    fn iterate<const SEQUENCE_SIZE: usize>(
        &self,
        sequence: &[Quantifiers; SEQUENCE_SIZE],
        start_position: usize,
        fail_fast: bool,
        match_end_of_input: bool,
    ) -> Option<(usize, usize)> {
        if sequence.is_empty() {
            return Some((start_position, 0));
        } else {
            let mut pattern = Pattern::new(sequence);
            if match_end_of_input && start_position == self.input.len() {
                return Some((start_position, 0));
            }
            for (index, char) in self.input.chars().enumerate().skip(start_position) {
                let is_character_in_pattern = pattern.check_character(&char);
                if is_character_in_pattern && pattern.is_end_of_sequence() {
                    return Some((index + 1, pattern.length));
                } else if match_end_of_input && index == self.input.len() - 1 {
                    return Some((index + 1, 0));
                } else if fail_fast && !is_character_in_pattern {
                    break;
                }
            }
        }
        None
    }

    fn get_unbalanced_match<const START_SEQUENCE_SIZE: usize, const END_SEQUENCE_SIZE: usize>(
        &mut self,
        start_sequence: &'input [Quantifiers; START_SEQUENCE_SIZE],
        end_sequence: &'input [Quantifiers; END_SEQUENCE_SIZE],
        match_end_of_input: bool,
    ) -> Option<Match<'input>> {
        if let Some((start_token_end_index, start_sequence_length)) =
            self.iterate(start_sequence, self.position, true, false)
        {
            if let Some((end_token_end_index, end_sequence_length)) = self.iterate(
                end_sequence,
                start_token_end_index,
                false,
                match_end_of_input,
            ) {
                self.position = end_token_end_index;
                return Some(Match {
                    start_token: &self.input
                        [(start_token_end_index - start_sequence_length)..start_token_end_index],
                    body: &self.input
                        [start_token_end_index..end_token_end_index - end_sequence_length],
                    end_token: &self.input
                        [(end_token_end_index - end_sequence_length)..end_token_end_index],
                });
            }
        }
        None
    }

    fn get_balanced_match<const START_SEQUENCE_SIZE: usize, const END_SEQUENCE_SIZE: usize>(
        &mut self,
        start_sequence: &'input [Quantifiers; START_SEQUENCE_SIZE],
        end_sequence: &'input [Quantifiers; END_SEQUENCE_SIZE],
    ) -> Option<Match<'input>> {
        let mut start_pattern = Pattern::new(start_sequence);
        let mut end_pattern = Pattern::new(end_sequence);
        let mut balance = 0;
        if let Some((start_token_end_index, _)) =
            self.iterate(start_sequence, self.position, true, false)
        {
            for (index, char) in self.input.chars().enumerate().skip(self.position) {
                if start_pattern.check_character(&char) && start_pattern.is_end_of_sequence() {
                    start_pattern.reset();
                    balance += 1;
                } else if balance > 0
                    && end_pattern.check_character(&char)
                    && end_pattern.is_end_of_sequence()
                {
                    balance -= 1;
                    if balance == 0 {
                        let end_token_end_index = index + 1;
                        let end_token_start_index = end_token_end_index - end_pattern.length;
                        self.position = index + 1;
                        return Some(Match {
                            start_token: &self.input[..start_token_end_index],
                            body: &self.input[start_token_end_index..end_token_start_index],
                            end_token: &self.input[end_token_start_index..end_token_end_index],
                        });
                    }
                    end_pattern.reset();
                }
            }
        }
        None
    }

    pub fn get_rest(&self) -> &'input str {
        &self.input[self.position..]
    }
}
#[cfg(test)]
mod tests {
    use crate::toolkit::tokenizer::{
        Match, Matcher, Pattern,
        Quantifiers::{Once, RepeatTimes, ZeroOrMore},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn get_match() {
        let mut matcher = Matcher::new("*italic*~~one more~~ statement");
        assert_eq!(
            matcher.get_match(&[Once('*')], &[Once('*')], false),
            Some(Match {
                start_token: "*",
                body: "italic",
                end_token: "*"
            })
        );
        assert_eq!(
            matcher.get_match(&[RepeatTimes(2, '~')], &[RepeatTimes(2, '~')], false),
            Some(Match {
                start_token: "~~",
                body: "one more",
                end_token: "~~",
            })
        );
    }

    #[test]
    fn get_match_with_end_of_input() {
        let mut matcher = Matcher::new("*italic*~~one more");
        assert_eq!(
            matcher.get_match(&[Once('*')], &[Once('*')], false),
            Some(Match {
                start_token: "*",
                body: "italic",
                end_token: "*"
            })
        );
        assert_eq!(
            matcher.get_match(&[RepeatTimes(2, '~')], &[RepeatTimes(2, '~')], true),
            Some(Match {
                start_token: "~~",
                body: "one more",
                end_token: "",
            })
        );
    }

    #[test]
    fn get_match_with_empty_start_token_and_macth_end() {
        let mut matcher = Matcher::new("t");
        assert_eq!(
            matcher.get_match(&[], &[RepeatTimes(2, '\n')], true),
            Some(Match {
                start_token: "",
                body: "t",
                end_token: ""
            })
        )
    }

    #[test]
    fn get_match_with_empty_body() {
        let mut matcher = Matcher::new("--");
        assert_eq!(
            matcher.get_match(&[RepeatTimes(2, '-')], &[RepeatTimes(2, '\n')], true),
            Some(Match {
                start_token: "--",
                body: "",
                end_token: ""
            })
        )
    }

    #[test]
    fn patterns_with_non_equal_length_can_not_be_balanced() {
        let mut matcher = Matcher::new("(()t");
        assert_eq!(
            matcher.get_match(&[Once('(')], &[Once(')'), Once('t')], false),
            Some(Match {
                start_token: "(",
                body: "(",
                end_token: ")t"
            })
        )
    }

    #[test]
    fn get_balanced_match() {
        let mut matcher = Matcher::new("{{}}");
        assert_eq!(
            matcher.get_match(&[Once('{')], &[Once('}')], false),
            Some(Match {
                start_token: "{",
                body: "{}",
                end_token: "}"
            })
        );
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
