use std::collections::LinkedList;

pub type NodeRef = std::rc::Rc<Node>;

pub struct Node {
    //
    parent: NodeRef,
    children: Vec<Self>,
    data: LinkedList<usize>,
}

impl Node {
    //
    pub fn new(parent: NodeRef, branch: usize) -> Self {
        Self {
            parent,
            children: vec![],
            data: LinkedList::new(),
        }
    }
}
