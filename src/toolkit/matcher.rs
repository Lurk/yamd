use super::pattern::{Pattern, Quantifiers};

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
                let (is_character_in_pattern, is_end_of_sequence) = pattern.check_character(&char);
                if is_character_in_pattern && is_end_of_sequence {
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
                if start_pattern.check_character(&char) == (true, true) {
                    start_pattern.reset();
                    balance += 1;
                } else if balance > 0 && end_pattern.check_character(&char) == (true, true) {
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
    use crate::toolkit::matcher::{
        Match, Matcher,
        Quantifiers::{Once, RepeatTimes},
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
}
