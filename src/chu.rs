use crate::prelude::*;

pub struct Matrix<T: Copy + Default> {
    shape: (usize, usize),

    data: Vec<T>,
}

impl<T: Copy + Default> Matrix<T> {
    pub fn new(shape: (usize, usize)) -> Self {
        Self {
            shape,
            data: vec![T::default(); shape.0 * shape.1],
        }
    }

    pub fn transpose(&self) -> Self {
        let mut ret = Self::new((self.shape.1, self.shape.0));
        ret
    }
}

impl<T: Copy + Default> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = self.shape.0 * index.0 + index.1;
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
