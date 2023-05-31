use std::collections::LinkedList;

pub type NodeRef = std::rc::Rc<Node>;

pub type Link = LinkedList<usize>;
// use crate::prelude::Link1;

// A Node (of a Tree) represents a given prefix in a set of lines.
// The children of a node are the possible extensions of that prefix.
// A Node consists of:
//  an array of child Nodes (for non-leaves)
//  a List of line indexes (for leaves)
//  an pointer to the parent Node
//   (representing this prefix sans last element)
//  a branch number (the value of the last element)
pub struct Node {
    parent: Option<NodeRef>,
    children: Vec<NodeRef>,
    branch: usize,
    link: Option<Link>,
}

impl Node {
    //
    pub fn new(parent: Option<NodeRef>, branch: usize) -> Self {
        Self {
            parent,
            children: vec![],
            branch,
            link: None,
        }
    }

    pub fn root() -> Self {
        Self::new(None, 0)
    }

    pub fn child(&self, branch: usize) -> Option<NodeRef> {
        self.children.get(branch).cloned()
    }

    pub fn parent(&self) -> NodeRef {
        unimplemented!()
    }

    pub fn link(&self) -> &Link {
        // &self.link.unwrap()
        unimplemented!()
    }

    pub fn branch(&self) -> usize {
        self.branch
    }

    pub fn add(&self, datum: usize) {
        unimplemented!()
    }

    pub fn grow(&self, branch: usize, arity: usize) -> NodeRef {
        //
        unimplemented!()
    }
}
