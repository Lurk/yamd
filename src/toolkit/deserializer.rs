use std::fmt::Display;

use super::{context::Context, node::Node};

pub trait Branch<BranchNodes>
where
    BranchNodes: Node + Display,
{
    fn push<CanBeNode: Into<BranchNodes>>(&mut self, node: CanBeNode);
    fn get_maybe_nodes() -> Vec<MaybeNode<BranchNodes>>;
    fn get_fallback_node() -> Option<DefinitelyNode<BranchNodes>>;
    fn get_outer_token_length(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn parse_branch(input: &str, delimeter: &str, mut branch: Self) -> Option<Self>
    where
        Self: Sized + Deserializer + Node,
    {
        let mut current_position = 0;
        let mut next_position = 0;
        let mut fallback_position = 0;
        let maybe_nodes = Self::get_maybe_nodes();
        while current_position < input.len() {
            let slice = &input[current_position..];
            next_position += slice.chars().next().unwrap().len_utf8();
            if delimeter.is_empty() || slice.starts_with(delimeter) || current_position == 0 {
                let slice = if current_position == 0 {
                    slice
                } else {
                    &slice[delimeter.len()..]
                };
                for parser in &maybe_nodes {
                    if let Some(node) = parser(slice, branch.context()) {
                        while fallback_position != current_position {
                            fallback_position = branch
                                .fallback(&input[fallback_position..current_position], delimeter);
                        }
                        branch.push(node);
                        next_position = branch.len() - branch.get_outer_token_length();
                        fallback_position = next_position;
                        break;
                    }
                }
            }
            current_position = next_position;
        }
        while fallback_position < input.len() {
            if Self::get_fallback_node().is_none() {
                return None;
            } else {
                fallback_position = branch.fallback(&input[fallback_position..], delimeter);
            }
        }
        Some(branch)
    }

    fn fallback(&mut self, slice: &str, delimeter: &str) -> usize
    where
        Self: Node,
    {
        let slice = if self.is_empty() {
            slice
        } else {
            &slice[delimeter.len()..]
        };
        if !slice.is_empty() {
            self.push(
                Self::get_fallback_node()
                    .map(|f| f(slice))
                    .expect("Fallback node should always be available"),
            );
        }
        self.len() - self.get_outer_token_length()
    }
}

pub type MaybeNode<BranchNodes> = Box<dyn Fn(&str, Option<Context>) -> Option<BranchNodes>>;
pub type DefinitelyNode<BranchNodes> = Box<dyn Fn(&str) -> BranchNodes>;

pub trait FallbackNode {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
    where
        Self: Into<BranchNodes>;
}

pub trait Deserializer {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self>
    where
        Self: Sized;

    fn deserialize(input: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::deserialize_with_context(input, None)
    }
}
