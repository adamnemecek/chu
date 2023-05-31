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
