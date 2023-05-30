pub struct ExactFromFn<F> {
    len: usize,
    i: std::iter::FromFn<F>,
}

impl<T, F: FnMut() -> Option<T>> ExactFromFn<F> {
    pub fn new(len: usize, f: F) -> Self {
        Self {
            len,
            i: std::iter::from_fn(f),
        }
    }
}

impl<T, F: FnMut() -> Option<T>> Iterator for ExactFromFn<F> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.i.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, self.len.into())
    }
}

impl<T, F: FnMut() -> Option<T>> ExactSizeIterator for ExactFromFn<F> {
    fn len(&self) -> usize {
        self.len
    }
}
