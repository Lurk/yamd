use super::{
    context::Context,
    deserializer::{Deserializer, MaybeNode},
};

pub trait Node {
    fn len(&self) -> usize;
    fn maybe_node<BranchNodes>() -> MaybeNode<BranchNodes>
    where
        Self: Sized + Deserializer + Into<BranchNodes>,
    {
        Box::new(|input, ctx| {
            if let Some(node) = Self::deserialize_with_context(input, ctx) {
                return Some(node.into());
            }
            None
        })
    }

    fn context(&self) -> Option<Context> {
        None
    }
}
