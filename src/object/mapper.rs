use crate::{map::*, object::*, prelude::*};
use std::collections::HashSet;

/// A new object obtained by applying some mapping to another object.
pub struct ObjectMapper<O: Object, M: Map> {
    pub object: O,
    pub map: M,
}

impl<O: Object, M: Map> ObjectMapper<O, M> {
    pub fn new(object: O, map: M) -> Self {
        Self { object, map }
    }
}

impl<O: Object, M: Map> Object for ObjectMapper<O, M> {}

impl<O: Object, M: Map> Instance<ObjectClass> for ObjectMapper<O, M> {
    fn source(cache: &mut HashSet<u64>) -> String {
        if !cache.insert(Self::type_hash()) {
            return String::new();
        }
        [
            O::source(cache),
            M::source(cache),
            "#include <clay_core/object/mapper.h>".to_string(),
            format!(
                "MAP_OBJECT_FN_DEF({}, {}, {}, {}, {})",
                Self::inst_name(),
                O::inst_name(),
                M::inst_name(),
                O::size_int(),
                O::size_float(),
            ),
        ]
        .join("\n")
    }
    fn inst_name() -> String {
        format!("__mapper_{:x}", Self::type_hash(),)
    }
}

impl<O: Object, M: Map> Pack for ObjectMapper<O, M> {
    fn size_int() -> usize {
        O::size_int() + M::size_int()
    }
    fn size_float() -> usize {
        O::size_float() + M::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
            .pack(&self.object)
            .pack(&self.map);
    }
}
