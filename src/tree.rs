use crate::prelude::*;

pub struct Tree {
    //
    arity: usize,
    length: usize,
    // top:
}

impl Tree {
    pub fn new(arity: usize, lenght: usize) -> Self {
        unimplemented!()
    }

    pub fn arity(&self) -> usize {
        unimplemented!()
    }

    pub fn length(&self) -> usize {
        unimplemented!()
    }

    pub fn top(&self) -> Node {
        unimplemented!()
    }

    pub fn find_line(&self, line: &[usize]) -> Link {
        unimplemented!()
    }

    pub fn add_line(&self, line: &[usize], index: usize) -> Link {
        unimplemented!()
    }
}
