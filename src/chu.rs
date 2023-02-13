use crate::prelude::*;

struct Matrix<T: Copy + Default> {
    r: usize,
    c: usize,
    data: Vec<T>,
}

impl<T: Copy + Default> Matrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            r: rows,
            c: cols,
            data: vec![T::default(); rows * cols],
        }
    }
}

pub struct Chu {
    k: usize,
}

impl Chu {
    fn new() -> Self {
        unimplemented!()
    }

    //
    fn dual(&self) -> Self {
        unimplemented!()
    }
}

impl Conformable for Chu {
    fn conform(ctx: Context) -> Self {
        unimplemented!()
    }
}
