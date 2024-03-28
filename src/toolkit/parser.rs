use std::usize;

use super::context::Context;

pub type Parser<N> = Box<dyn Fn(&str, usize, Option<&Context>) -> Option<(N, usize)>>;
pub type Consumer<N> = Box<dyn Fn(&str, usize, Option<&Context>) -> (N, usize)>;

pub fn parse_to_parser<N, P>() -> Parser<N>
where
    P: Parse + Sized + Into<N>,
{
    Box::new(move |input, current_position, ctx| {
        P::parse(input, current_position, ctx).map(|(n, consumed)| (n.into(), consumed))
    })
}

pub fn parse_to_consumer<N, P>() -> Consumer<N>
where
    P: Parse + Sized + Into<N>,
{
    Box::new(move |input, current_position, ctx| {
        let (n, consumed) =
            P::parse(input, current_position, ctx).expect("consumer shoud never fail");
        (n.into(), consumed)
    })
}

pub trait Parse {
    fn parse(input: &str, current_position: usize, ctx: Option<&Context>) -> Option<(Self, usize)>
    where
        Self: Sized;
}

pub trait Branch<N> {
    fn get_parsers(&self) -> Vec<Parser<N>>;
    fn get_consumer(&self) -> Option<Consumer<N>>;
    fn push_node(&mut self, node: N);
    fn consume(&mut self, input: &str, delimeter: &str, ctx: Option<&Context>)
    where
        Self: Sized,
    {
        if let Some(consumer) = self.get_consumer() {
            let mut total_consumed = 0;
            while total_consumed < input.len() {
                if !delimeter.is_empty() && input[total_consumed..].starts_with(delimeter) {
                    total_consumed += delimeter.len();
                }

                if total_consumed == input.len() {
                    break;
                }
                let (node, consumed) = consumer(&input[total_consumed..], 0, ctx);
                self.push_node(node);
                total_consumed += consumed;
            }
        }
    }
    fn parse_branch(mut self, input: &str, delimeter: &str, ctx: Option<Context>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut current_position = 0;
        let mut should_consume: Option<usize> = None;
        while current_position < input.len() {
            if current_position != 0
                && !delimeter.is_empty()
                && input[current_position..].starts_with(delimeter)
            {
                current_position += delimeter.len();
            }
            let start = current_position;
            for parser in self.get_parsers() {
                if let Some((node, parsed)) = parser(input, current_position, ctx.as_ref()) {
                    if let Some(consume_from) = should_consume {
                        self.consume(&input[consume_from..start], delimeter, ctx.as_ref());
                        should_consume = None;
                    }
                    current_position += parsed;
                    self.push_node(node);
                    break;
                }
            }
            if start == current_position {
                if self.get_consumer().is_none() {
                    return None;
                }
                if should_consume.is_none() {
                    should_consume = Some(current_position);
                }
                current_position += &input[current_position..].chars().next().unwrap().len_utf8();
            }
        }
        if let Some(consume_from) = should_consume {
            self.consume(
                &input[consume_from..current_position],
                delimeter,
                ctx.as_ref(),
            );
        }

        Some(self)
    }
}
