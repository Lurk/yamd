use super::{
    context::Context,
    deserializer::{Deserializer, MaybeNode},
};

pub trait Node<'text> {
    fn serialize(&self) -> String;
    fn len(&self) -> usize;
    fn maybe_node<BranchNodes>() -> MaybeNode<'text, BranchNodes>
    where
        Self: Sized + Deserializer<'text> + Into<BranchNodes>,
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
