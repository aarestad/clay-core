use std::collections::HashSet;
use crate::Customer;


pub trait Scene: Customer {
    fn source(cache: &mut HashSet<u64>) -> String;
}
