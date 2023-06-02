///
/// Like from `FromFn` but with `ExactSize`
///
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

pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, x: T) {
        self.0.push(x)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}
