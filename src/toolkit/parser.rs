pub type Parser<N> = Box<dyn Fn(&str, usize) -> Option<(N, usize)>>;
pub type Consumer<N> = Box<dyn Fn(&str, usize) -> (N, usize)>;

pub fn parse_to_parser<N, P>() -> Parser<N>
where
    P: Parse + Sized + Into<N>,
{
    Box::new(move |input, current_position| {
        P::parse(input, current_position).map(|(n, consumed)| (n.into(), consumed))
    })
}

pub fn parse_to_consumer<N, P>() -> Consumer<N>
where
    P: Parse + Sized + Into<N>,
{
    Box::new(move |input, current_position| {
        let (n, consumed) = P::parse(input, current_position).expect("consumer shoud never fail");
        (n.into(), consumed)
    })
}

pub trait Parse {
    fn parse(input: &str, current_position: usize) -> Option<(Self, usize)>
    where
        Self: Sized;
}

pub trait Branch<N> {
    fn get_parsers(&self) -> Vec<Parser<N>>;
    fn get_consumer(&self) -> Option<Consumer<N>>;
    fn push_node(&mut self, node: N);
    fn consume(&mut self, from: &mut Option<usize>, input: &str, delimeter: &str)
    where
        Self: Sized,
    {
        if let (Some(from), Some(consumer)) = (&from, self.get_consumer()) {
            let mut position = *from;
            while position < input.len() {
                if !delimeter.is_empty() && input[position..].starts_with(delimeter) {
                    position += delimeter.len();
                }

                let (node, consumed) = consumer(&input[position..], 0);
                self.push_node(node);
                position += consumed;
            }
        }
        *from = None;
    }

    fn parse_branch(mut self, input: &str, delimeter: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let parsers = self.get_parsers();
        let mut position = 0;
        let mut should_consume: Option<usize> = None;

        while position < input.len() {
            let start = position;
            let starts_with_delimeter =
                delimeter.is_empty() || input[position..].starts_with(delimeter);

            if position == 0 || starts_with_delimeter {
                if position != 0 && starts_with_delimeter {
                    position += delimeter.len();
                }

                if let Some((node, parsed)) = parsers.iter().find_map(|p| p(input, position)) {
                    self.consume(&mut should_consume, &input[..start], delimeter);
                    position += parsed;
                    self.push_node(node);
                }
            }

            if start == position {
                let _ = self.get_consumer()?;
                should_consume = should_consume.or(Some(position));
                position += &input[position..]
                    .chars()
                    .next()
                    .expect("always to have next character")
                    .len_utf8();
            }
        }

        self.consume(&mut should_consume, input, delimeter);

        Some(self)
    }
}
