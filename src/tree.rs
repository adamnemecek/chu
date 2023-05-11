use crate::prelude::*;

pub struct Tree {
    //
    arity: usize,
    len: usize,
    root: Option<NodeRef>,
}

impl Tree {
    pub fn new(arity: usize, len: usize) -> Self {
        Self {
            arity,
            len,
            root: Some(Node::new(None, 0).into()),
        }
    }

    pub fn arity(&self) -> usize {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn root(&self) -> Option<NodeRef> {
        self.root.clone()
    }

    pub fn find_line(&self, line: &[usize]) -> Option<Link> {
        if line.len() != self.len {
            return None;
        }

        let mut cur = self.root.clone();

        for e in line {
            if let Some(v) = &cur {
                cur = v.child(*e).clone().into();
            } else {
                return None;
            }
        }
        // cur.da
        unimplemented!()
    }

    pub fn add_line(&self, line: &[usize], index: usize) -> Link {
        unimplemented!()
    }
}
