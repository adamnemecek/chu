//
use std::rc::Rc;
pub enum Link1 {
    //
    Empty,
    Next(usize, Option<Rc<Self>>), // datum: usize,
                                   // next: Option<std::rc::Rc<Self>>,
}

impl Clone for Link1 {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Next(v, r) => Self::Next(*v, r.clone()),
        }
    }
}

impl Link1 {
    pub fn new(datum: usize, next: Option<Rc<Self>>) -> Self {
        Self::Next(datum, next.clone())
    }

    pub fn datum(&self) -> Option<usize> {
        match self {
            Self::Empty => None,
            Self::Next(v, _) => Some(*v),
        }
    }
}

pub trait Fill<T> {
    fn fill(count: usize, f: impl Fn() -> T) -> Self;
}

impl<T> Fill<T> for Vec<T> {
    fn fill(count: usize, f: impl Fn() -> T) -> Self {
        let mut self_ = Self::with_capacity(count);
        for _ in 0..count {
            self_.push(f());
        }
        self_
    }
}
