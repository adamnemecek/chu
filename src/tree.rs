use crate::prelude::*;

// A tree is used to store a collection of equal-length "lines"
// The lines are sequences of integers in the range 0..arity-1

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
        self.arity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn root(&self) -> Option<NodeRef> {
        self.root.clone()
    }

    // find_line: Returns a linked list of the
    // indexes of all lines matching the given line.
    // line is trace i think
    pub fn find_line(&self, line: &[usize]) -> Option<Link> {
        if line.len() != self.len {
            return None;
        }

        let mut cur = self.root.clone();

        for l in line {
            let Some(v) = &cur else { return None; };
            cur = v.child(*l).clone().into();
        }

        cur.map(|x| x.link().clone())
    }

    // addLine: Inserts the given line at the given index.
    // Returns a linked list of the indexes of all other
    // lines which match the new line.
    pub fn add_line(&self, line: &[usize], index: usize) -> Option<Link> {
        if line.len() != self.len {
            return None;
        }

        let mut current = self.root.clone();

        for i in line {
            //
            current = current.unwrap().grow(*i, self.arity).into();
        }

        unimplemented!()
    }
}
