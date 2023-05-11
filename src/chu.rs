// use std::fmt::Debug;

use crate::prelude::*;

// enum Standardized {
//     yes,
//     no(Option<Box<Chu>>),
// }

enum A {
    Unknown,
    Initial,
    Final,
    Middle,
    Duplicate,
}

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
        Self::new(self.k, self.data.transpose(), self.std.is_none())
    }

    fn query2() -> Self {
        unimplemented!()
    }

    pub fn query(&self) -> Self {
        if self.k == 2 {
            return Self::query2();
        }

        unimplemented!()
    }

    pub fn seq(&self, other: &Self) -> Self {
        let k = self.k.max(other.k);

        let class_a = self.classify_cols();
        let class_b = other.classify_cols();
        unimplemented!()
    }

    pub fn classify_cols(&self) {
        unimplemented!()
    }

    pub fn compare_cols(col1: usize, col2: usize) -> Option<std::cmp::Ordering> {
        unimplemented!()
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
    fn implication(&self, other: Self) -> Self {
        let k = self.k.max(other.k);
        let size = self.rows() * other.cols();

        let mut mg = MatrixGenerator::new(other.row_tree(), self.col_tree());

        while mg.next() {
            //
            let mut num_instances = 1;
            // for r in mg.nrows() {
            //
            // }
        }
        unimplemented!()
    }
}
