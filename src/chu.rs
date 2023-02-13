use crate::prelude::*;

pub struct Matrix<T: Copy + Default> {
    shape: (usize, usize),

    data: Vec<T>,
}

fn t<T>(a: (T, T))-> (T, T){
    (a.1, a.0)
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
        for i in 0..self.shape.0{
            for j in 0..self.shape.1 {
                ret[(j,i)] = self[(i,j)];
            }
        }
        ret
    }
}

impl<T: Copy + Default> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = self.shape.0 * index.0 + index.1;
        &self.data[idx]
    }
}

impl<T: Copy + Default> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.shape.0 * index.0 + index.1;
        &mut self.data[idx]
    }
}

pub struct Chu {
    k: usize,
}
g
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
