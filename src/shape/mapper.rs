use crate::{map::Map, prelude::*, shape::*};
use std::collections::HashSet;

/// A new shape obtained by applying some mapping to another shape.
pub struct ShapeMapper<S: Shape, M: Map> {
    pub shape: S,
    pub map: M,
}

impl<S: Shape, M: Map> ShapeMapper<S, M> {
    pub fn new(shape: S, map: M) -> Self {
        Self { shape, map }
    }
}

impl<S: Shape, M: Map> Shape for ShapeMapper<S, M> {}

impl<S: Shape, M: Map> Instance<ShapeClass> for ShapeMapper<S, M> {
    fn source(cache: &mut HashSet<u64>) -> String {
        if !cache.insert(Self::type_hash()) {
            return String::new();
        }
        [
            S::source(cache),
            M::source(cache),
            "#include <clay_core/shape/mapper.h>".to_string(),
            format!(
                "MAP_SHAPE_FN_DEF({}, {}, {}, {}, {})",
                Self::inst_name(),
                S::inst_name(),
                M::inst_name(),
                S::size_int(),
                S::size_float(),
            ),
        ]
        .join("\n")
    }
    fn inst_name() -> String {
        format!("__mapper_{:x}", Self::type_hash(),)
    }
}

impl<S: Shape, M: Map> Pack for ShapeMapper<S, M> {
    fn size_int() -> usize {
        S::size_int() + M::size_int()
    }
    fn size_float() -> usize {
        S::size_float() + M::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
            .pack(&self.shape)
            .pack(&self.map);
    }
}
