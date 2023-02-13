use crate::prelude::*;

struct Matrix<T: Copy + Default> {
    k: usize
    r: usize,
    c: usize,
    data: Vec<T>,
}

impl<T: Copy + Default> Matrix<T> {
    fn new(k: usize ,rows: usize, cols: usize) -> Self {
        Self {
            k,
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
    //
}

impl Conformable for Chu {
    fn conform(ctx: Context) -> Self {
        unimplemented!()
    }
}
