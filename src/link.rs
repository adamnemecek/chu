//
pub struct Link1 {
    //
    datum: usize,
    next: Option<std::rc::Rc<Self>>,
}

impl Link1 {
    pub fn new(datum: usize, next: Option<std::rc::Rc<Self>>) -> Self {
        Self { datum, next }
    }

    pub fn datum(&self) -> usize {
        self.datum
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
