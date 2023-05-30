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
    std: Option<Box<Matrix<usize>>>,
}

impl Chu {
    pub fn new(k: usize, data: Matrix<usize>, standardized: bool) -> Self {
        let mut s = Self { k, data, std: None };

        // s.std = if standardized {
        //     // Some(Box::new(s))
        //     Some(data.clone())
        // } else {
        //     None
        // };
        s
    }

    pub fn new_with_size(size: usize) -> Self {
        let m = Matrix::new((1, size));
        Self::new(size, m, true)
    }

    pub fn rows(&self) -> usize {
        self.shape().0
    }

    pub fn cols(&self) -> usize {
        self.shape().1
    }

    pub fn row_tree(&self) -> Tree {
        let mut t = Tree::new(self.k, self.cols());
        for r in 0..self.rows() {
            let line = self.data.row(r);
            t.add_line(line, r);
        }
        t
    }

    pub fn col_tree(&self) -> Tree {
        let mut t = Tree::new(self.k, self.rows());
        for c in 0..self.rows() {
            let line = self.data.col(c);
            t.add_line(line, c);
        }
        t
    }

    pub fn shape(&self) -> (usize, usize) {
        self.data.shape()
    }

    //
    pub fn dual(&self) -> Self {
        Self::new(self.k, self.data.transpose(), self.std.is_none())
    }

    pub fn query(&self) -> Self {
        if self.k == 2 {
            return Self::query2();
        }

        // let row =

        let row_tree = self.row_tree();

        for k in 0..self.k {
            //
        }

        unimplemented!()
    }

    fn query2() -> Self {
        // let row_tree = Tree::new(2, self.cols());

        unimplemented!()
    }

    fn choice(&self, other: &Self) -> Self {
        let k = self.k.max(other.k);
        let rows = self.rows() + other.rows();
        let cols = self.cols() + other.cols();

        let mut m = Matrix::<usize>::new((rows, cols));
        for r in 0..rows {
            for c in 0..cols {
                m[(r, c)] = if r < self.rows() {
                    if c < self.cols() {
                        self[(r, c)]
                    } else {
                        0
                    }
                } else {
                    if c < self.cols() {
                        0
                    } else {
                        other[(r - self.rows(), c - self.cols())]
                    }
                }
            }
        }
        Self::new(k, m, false)
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
        let rows = self.rows() * rhs.rows();
        let cols = self.cols() + rhs.cols();

        let mut m = Matrix::<usize>::new((rows, cols));

        let mut r = 0;
        let mut c = 0;
        // Loop over rows of A

        for ar in 0..self.rows() {
            // Loop over rows of B
            for br in 0..rhs.rows() {
                // Create concatination of A.matrix[ar] and B.matrix[br]

                for ac in 0..self.cols() {
                    m[(r, c)] = self[(ar, ac)];
                    c += 1;
                }

                for bc in 0..rhs.cols() {
                    m[(r, c)] = self[(br, bc)];
                    c += 1;
                }

                r += 1;
                c = 0;
            }
        }
        Self::new(k, m, false)
    }
}

impl std::ops::Index<(usize, usize)> for Chu {
    type Output = usize;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index]
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
