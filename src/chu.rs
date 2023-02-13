use crate::prelude::*;

pub struct Matrix<T: Copy + Default> {
    r: usize,
    c: usize,
    data: Vec<T>,
}

impl<T: Copy + Default> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            r: rows,
            c: cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn transpose(&self) -> Self {
        unimplemented!()
    }
}

pub struct Chu {
    k: usize,
}

impl Chu {
    pub fn new() -> Self {
        unimplemented!()
    }

    //
    pub fn dual(&self) -> Self {
        unimplemented!()
    }
}

impl Conformable for Chu {
    fn conform(ctx: Context) -> Self {
        unimplemented!()
    }
}
