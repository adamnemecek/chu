use std::collections::LinkedList;

pub type NodeRef = std::rc::Rc<Node>;

pub type Link = LinkedList<usize>;

pub struct Node {
    //
    parent: Option<NodeRef>,
    children: Vec<NodeRef>,
    branch: usize,
    data: Link,
}

impl Node {
    //
    pub fn new(parent: Option<NodeRef>, branch: usize) -> Self {
        Self {
            parent,
            children: vec![],
            branch,
            data: Link::new(),
        }
    }

    pub fn child(&self, branch: usize) -> NodeRef {
        unimplemented!()
    }

    pub fn parent(&self) -> NodeRef {
        unimplemented!()
    }

    pub fn branch(&self) -> usize {
        self.branch
    }
}
