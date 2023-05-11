// use std::fmt::Debug;

use crate::prelude::*;

// enum Standardized {
//     yes,
//     no(Option<Box<Chu>>),
// }

pub struct Chu {
    k: usize,
    data: Matrix<usize>,
    std: Option<Box<Self>>,
}

impl Chu {
    pub fn new(k: usize, data: Matrix<usize>, standardized: bool) -> Self {
        let self_ = Self {
            k,
            data,
            std: if standardized {
                None
            } else {
                // Some(None)
                unimplemented!()
            },
        };
        self_
    }

    pub fn new_with_size(size: usize) -> Self {
        unimplemented!()
    }

    pub fn rows(&self) -> usize {
        self.shape().0
    }

    pub fn cols(&self) -> usize {
        self.shape().1
    }

    pub fn row_tree(&self) -> Tree {
        let mut t = Tree::new(self.k, self.shape().1);
        for i in 0..self.rows() {
            //
        }
        t
    }

    pub fn col_tree(&self) -> Tree {
        unimplemented!()
    }

    pub fn shape(&self) -> (usize, usize) {
        self.data.shape()
    }

    //
    pub fn dual(&self) -> Self {
        // Self::new()
        unimplemented!()
        // Self::new(k, shape, standardized)
    }
}

impl Conformable for Chu {
    fn conform(ctx: Context) -> Self {
        unimplemented!()
    }
}

impl std::ops::Mul for Chu {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let k = self.k.max(rhs.k);
        let rows = self.shape().0 * rhs.shape().0;
        let cols = self.shape().1 + rhs.shape().1;

        let mut m = Matrix::<usize>::new((rows, cols));
        unimplemented!()
    }
}

impl Chu {
    fn sequence(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn implication(&self, other: Self) -> Self {
        let k = self.k.max(other.k);
        unimplemented!()
    }
}
