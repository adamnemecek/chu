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
    pub fn new(k: usize, shape: (usize, usize), standardized: bool) -> Self {
        let c = Self {
            k,
            data: Matrix::new(shape),
            std: if standardized {
                None
            } else {
                // Some(None)
                unimplemented!()
            },
        };
        unimplemented!()
    }

    pub fn new_with_size() -> Self {
        unimplemented!()
    }

    pub fn row_tree(&self) {
        // Tree
        unimplemented!()
    }

    pub fn col_tree(&self) {
        unimplemented!()
    }

    pub fn shape(&self) -> (usize, usize) {
        self.data.shape()
    }

    //
    pub fn dual(&self) -> Self {
        // Self::new()
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
    fn sequence(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn implication(&self, other: Self) -> Self {
        let k = self.k.max(other.k);
        unimplemented!()
    }
}
