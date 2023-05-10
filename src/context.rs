// use crate::prelude::*;
pub struct Context {
    pub k: usize,
    pub standardization: bool,
}

impl Context {
    pub fn new(k: usize, standardization: bool) -> Self {
        Self { k, standardization }
    }
}
