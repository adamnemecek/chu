use std::cell::RefCell;
use std::rc::Rc;

// use std::borrow::Borrow;

use crate::prelude::*;

// A tree is used to store a collection of equal-length "lines"
// The lines are sequences of integers in the range 0..arity-1

#[derive(Clone, Debug)]
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
            root: Some(Node::root().into()),
        }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn root(&self) -> NodeRef {
        self.root.clone().unwrap()
    }

    // find_line: Returns a linked list of the
    // indexes of all lines matching the given line.
    // line is trace i think
    pub fn find_line(
        &self,
        line: &[usize],
    ) -> Option<Rc<RefCell<std::collections::LinkedList<usize>>>> {
        if line.len() != self.len() {
            return None;
        }

        let mut cur = self.root();

        for l in line {
            // current = current
            let Some(v) = cur.child(*l) else { return None; };
            // cur = v.child(*l).clone().into();
            cur = v;
        }
        Some(cur.list())
        // cur.map(|x| x.link().clone())
        // unimplemented!()
    }

    // addLine: Inserts the given line at the given index.
    // Returns a linked list of the indexes of all other
    // lines which match the new line.
    pub fn add_line<'a, T: std::borrow::Borrow<usize>>(
        &mut self,
        line: impl Iterator<Item = T> + ExactSizeIterator,
        index: usize,
    ) -> Option<Rc<RefCell<Link>>> {
        if line.len() != self.len {
            return None;
        }

        let mut current = self.root().clone();

        for i in line {
            //
            current = current.grow(*i.borrow(), self.arity).into();
        }

        current.add_datum_mut(index);
        Some(current.list())
    }
}
