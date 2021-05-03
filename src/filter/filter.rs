use crate::{Push, TypeHash};
use std::collections::HashSet;

/// Filter is applied to each pixel of rendered picture.
pub trait Filter: Push + TypeHash + 'static {
    fn inst_name() -> String;
    fn source(cache: &mut HashSet<u64>) -> String;
}
