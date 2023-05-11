use crate::prelude::*;

pub struct Tree {
    //
    arity: usize,
    length: usize,
    root: NodeRef,
}

impl Tree {
    pub fn new(arity: usize, length: usize) -> Self {
        Self {
            arity,
            length,
            root: Node::new(None, 0).into(),
        }
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
