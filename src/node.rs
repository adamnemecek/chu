pub type NodeRef = std::rc::Rc<Node>;

use std::{
    cell::RefCell,
    collections::LinkedList,
    rc::Rc,
};

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

#[derive(Debug)]
struct NodeData {
    parent: Option<NodeRef>,
    children: Vec<Option<NodeRef>>,
    branch: usize,
    list: Rc<RefCell<LinkedList<usize>>>,
}

impl NodeData {
    pub fn new(parent: Option<NodeRef>, branch: usize) -> Self {
        Self {
            parent,
            children: <_>::default(),
            branch,
            list: <_>::default(),
        }
    }
}

#[derive(Debug)]
pub struct Node(Rc<RefCell<NodeData>>);

impl Node {
    pub fn new(parent: Option<NodeRef>, branch: usize) -> Self {
        Self(Rc::new(NodeData::new(parent, branch).into()))
    }

    pub fn root() -> Self {
        Self::new(None, 0)
    }

    pub fn child(&self, branch: usize) -> Option<NodeRef> {
        let c = &self.borrow().children;
        let Some(c) = c.get(branch) else { return None };
        c.clone().into()
    }

    pub fn parent(&self) -> NodeRef {
        self.borrow().parent.clone().unwrap()
    }

    fn borrow(&self) -> std::cell::Ref<'_, NodeData> {
        self.0.as_ref().borrow()
    }

    fn borrow_mut(&self) -> std::cell::RefMut<'_, NodeData> {
        self.0.as_ref().borrow_mut()
    }

    pub fn list(&self) -> Rc<RefCell<LinkedList<usize>>> {
        self.borrow().list.clone()
    }

    pub fn branch(&self) -> usize {
        self.borrow().branch
    }

    pub fn add_datum_mut(&self, datum: usize) {
        self.borrow_mut().list.borrow_mut().push_front(datum)
    }

    // pub fn datum(&self) -> usize {
    //     // let z = self.borrow().list.borrow();
    //     self.borrow()
    //     0
    // }

    pub fn grow(self: Rc<Self>, branch: usize, arity: usize) -> NodeRef {
        let n = Rc::new(Self::new(self.clone().into(), branch));

        let mut b = self.borrow_mut();
        b.children.resize(branch + 1, None);
        b.children[branch] = Some(n.clone());

        n
    }
}
