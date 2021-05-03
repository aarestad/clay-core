use crate::{map::Chain, prelude::*};

/// Some invertible mapping that could be applied to vectors.
pub trait Map: Pack + Instance<MapClass> {
    /// Create a new mapping where this one is followed by another one.
    fn chain<M: Map>(self, other: M) -> Chain<Self, M> {
        Chain::<Self, M>::new(self, other)
    }
}

/// Device interface for mapping.
pub enum MapClass {}
impl Class for MapClass {
    fn name() -> String {
        "map".to_string()
    }
    fn methods() -> Vec<String> {
        ["rel", "abs", "rel_inv", "abs_inv", "norm"]
            .iter()
            .map(|m| m.to_string())
            .collect()
    }
}
