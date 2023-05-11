use std::collections::LinkedList;

pub type NodeRef = std::rc::Rc<Node>;

pub type Link = LinkedList<usize>;

pub struct Node {
    //
    parent: Option<NodeRef>,
    children: Vec<NodeRef>,
    branch: usize,
    link: Link,
}

impl Node {
    //
    pub fn new(parent: Option<NodeRef>, branch: usize) -> Self {
        Self {
            parent,
            children: vec![],
            branch,
            link: Link::new(),
        }
    }

    pub fn child(&self, branch: usize) -> NodeRef {
        unimplemented!()
    }

    pub fn parent(&self) -> NodeRef {
        unimplemented!()
    }

    pub fn link(&self) -> &Link {
        &self.link
    }

    pub fn branch(&self) -> usize {
        self.branch
    }

    pub fn add(&self, datum: usize) {
        unimplemented!()
    }

    pub fn grow(&self, branch: usize, arity: usize) {
        //
    }
}
