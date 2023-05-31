//
use std::rc::Rc;
pub enum Link1 {
    //
    None,
    Some(usize, Option<Rc<Self>>), // datum: usize,
                                   // next: Option<std::rc::Rc<Self>>,
}

impl Clone for Link1 {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Some(v, r) => Self::Some(*v, r.clone()),
        }
    }
}

impl Link1 {
    pub fn new(datum: usize, next: Option<Rc<Self>>) -> Self {
        Self::Some(datum, next.clone())
    }

    pub fn datum(&self) -> Option<usize> {
        match self {
            Self::None => None,
            Self::Some(v, _) => Some(*v),
        }
    }
}

impl Iterator for Link1 {
    type Item = Rc<Self>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::Some(v, n) => n.clone(),
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
