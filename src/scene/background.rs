use crate::Store;
use std::collections::HashSet;

/// Background of the scene.
///
/// It defines that happens to the ray is it hasn't hit any object in the scene.
pub trait Background: Store {
    fn source(cache: &mut HashSet<u64>) -> String;
}
