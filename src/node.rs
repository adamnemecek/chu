use std::collections::LinkedList;

pub type NodeRef = std::rc::Rc<Node>;

pub type Link = LinkedList<usize>;

pub struct Node {
    //
    parent: NodeRef,
    children: Vec<Self>,
    branch: usize,
    data: Link,
}

impl Node {
    //
    pub fn new(parent: NodeRef, branch: usize) -> Self {
        Self {
            parent,
            children: vec![],
            branch,
            data: Link::new(),
        }
    }

    pub fn child(&self, branch: usize) -> Self {
        unimplemented!()
    }

    pub fn parent(&self) -> Node {
        unimplemented!()
    }

    pub fn branch(&self) -> usize {
        self.branch
    }
}
