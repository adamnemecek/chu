use crate::prelude::*;

pub struct Matrix<T: Copy + Default> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Copy + Default> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn transpose(&self) -> Self {
        let mut ret = Self::new(self.cols, self.rows);
        ret
    }
}

impl<T: Copy + Default> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unimplemented!()
    }
}

impl<T: Copy + Default> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
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
