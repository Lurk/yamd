use super::{context::Context, node::Node};

pub trait Branch<'text, BranchNodes>
where
    BranchNodes: Node<'text>,
{
    fn push<CanBeNode: Into<BranchNodes>>(&mut self, node: CanBeNode);
    fn get_maybe_nodes() -> Vec<MaybeNode<'text, BranchNodes>>;
    fn get_fallback_node() -> Option<DefinitelyNode<BranchNodes>>;
    fn get_outer_token_length(&self) -> usize;

    fn parse_branch(input: &'text str, mut branch: Self) -> Option<Self>
    where
        Self: Sized + Deserializer<'text> + Node<'text>,
    {
        let mut current_position = 0;
        let mut fallback_position = 0;
        let fallback_node = Self::get_fallback_node();
        let maybe_nodes = Self::get_maybe_nodes();
        while current_position < input.len() {
            let slice = &input[current_position..];
            current_position += slice.chars().next().unwrap().len_utf8();
            if maybe_nodes.is_empty() {
                match fallback_node.as_ref() {
                    Some(fallback_node) => {
                        branch.push(fallback_node(slice));
                        current_position = branch.len() - branch.get_outer_token_length();
                        fallback_position = current_position;
                    }
                    None => return None,
                }
                continue;
            }
            for parser in &maybe_nodes {
                if let Some(node) = parser(slice, branch.context()) {
                    if fallback_position != current_position - 1 {
                        match &fallback_node {
                            Some(fallback_node) => branch.push(fallback_node(
                                &input[fallback_position..current_position - 1],
                            )),
                            None => return None,
                        }
                    }
                    branch.push(node);
                    current_position = branch.len() - branch.get_outer_token_length();
                    fallback_position = current_position;
                }
            }
        }
        if fallback_position < input.len() {
            match fallback_node {
                Some(fallback_node) => branch.push(fallback_node(&input[fallback_position..])),
                None => return None,
            }
        }

        Some(branch)
    }
}

pub type MaybeNode<'text, BranchNodes> =
    Box<dyn Fn(&'text str, Option<Context>) -> Option<BranchNodes>>;
pub type DefinitelyNode<BranchNodes> = Box<dyn Fn(&str) -> BranchNodes>;

pub trait FallbackNode {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
    where
        Self: Into<BranchNodes>;
}

pub trait Deserializer<'text> {
    fn deserialize_with_context(input: &'text str, ctx: Option<Context>) -> Option<Self>
    where
        Self: Sized;

    fn deserialize(input: &'text str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::deserialize_with_context(input, None)
    }
}
