use crate::link::VecExt;
use {
    crate::prelude::ExactFromFn,
    std::{
        fmt::Debug,
        iter::FromFn,
    },
};

// pub trait Inv {
//     fn inv(&self) -> Self;
// }

// impl<T: Clone> Inv for (T, T) {
//     fn inv(&self) -> Self {
//         (self.1.clone(), self.0.clone())
//     }
// }

#[macro_export]
macro_rules! matrix {
    () => {
        $crate::matrix::Matrix::default()
    };
    ($( $( $x: expr ), *); *) => {
        $crate::matrix::Matrix::from_arrays([ $( [ $($x),* ] ),* ])
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    shape: (usize, usize),
    data: Vec<T>,
}

impl<T: Copy + Default> Default for Matrix<T> {
    fn default() -> Self {
        Self::new((0, 0))
    }
}

impl<T: Copy + Default> Matrix<T> {
    fn raw(shape: (usize, usize), data: Vec<T>) -> Self {
        Self { shape, data }
    }

    pub fn from_arrays<const R: usize, const C: usize>(data: [[T; C]; R]) -> Self {
        Self {
            shape: (R, C),
            data: Vec::from_arrays(data),
        }
    }

    pub fn new(shape: (usize, usize)) -> Self {
        Self {
            shape,
            data: vec![T::default(); shape.0 * shape.1],
        }
    }

    pub fn from_vecs(vecs: &Vec<Vec<T>>) -> Self {
        let cols = vecs[0].len();
        println!("cols {cols}");
        let mut m = Self::new((vecs.len(), cols));

        for (i, v) in vecs.iter().enumerate() {
            let l = v.len();
            // assert!(v.len() == cols);
            //
            // m.data[i * cols..(i * cols + cols)].copy_from_slice(&v)
            m.set_row(i, v.iter())
        }
        m
    }

    pub fn fill(&mut self, f: impl Fn((usize, usize)) -> T) {
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                self[(i, j)] = f((i, j));
            }
        }
    }

    pub fn with_fill(shape: (usize, usize), f: impl Fn((usize, usize)) -> T) -> Self {
        let mut m = Self::new(shape);
        m.fill(f);
        m
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn row_range(&self, row: usize) -> std::ops::Range<usize> {
        assert!(row <= self.nrows());
        let start = row * self.ncols();
        start..(start + self.ncols())
    }

    #[inline]
    pub fn nrows(&self) -> usize {
        self.shape().0
    }

    #[inline]
    pub fn ncols(&self) -> usize {
        self.shape().1
    }
    // pub fn col_range(&self, row: usize) -> std::ops::Range<usize> {
    //     let start = row * self.shape.1;
    //     start..(start + self.shape.1)
    // }

    pub fn row(&self, row: usize) -> &[T] {
        &self.data[self.row_range(row)]
    }

    pub fn col<'a>(&self, col: usize) -> impl Iterator<Item = &T> + ExactSizeIterator {
        let mut r = 0..self.nrows();
        ExactFromFn::new(self.nrows(), move || r.next().map(|i| &self[(i, col)]))
    }

    pub fn set_row<B: std::borrow::Borrow<T>>(
        &mut self,
        row_index: usize,
        data: impl Iterator<Item = B> + ExactSizeIterator,
    ) {
        assert_eq!(self.ncols(), data.len());
        for (i, e) in data.enumerate() {
            self[(row_index, i)] = *e.borrow();
        }
    }

    pub fn set_col<B: std::borrow::Borrow<T>>(
        &mut self,
        col_index: usize,
        data: impl Iterator<Item = B> + ExactSizeIterator,
    ) {
        assert_eq!(self.nrows(), data.len());

        for (i, e) in data.enumerate() {
            self[(i, col_index)] = *e.borrow();
        }
    }

    pub fn transpose(&self) -> Self {
        // let mut z = 0;
        // let mut ret = Self::new((self.shape.1, self.shape.0));

        // for i in 0..self.nrows() {
        //     for j in 0..self.ncols() {
        //         ret[(j, i)] = self[(i, j)];
        //     }
        // }
        // ret
        Self::with_fill((self.shape.1, self.shape.0), |(i, j)| self[(j, i)])
    }

    fn offset(&self, index: (usize, usize)) -> usize {
        self.shape.1 * index.0 + index.1
    }
}

impl<T: Copy + Default> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.offset(index)]
    }
}

impl<T: Copy + Default> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let o = self.offset(index);
        &mut self.data[o]
    }
}

impl<T: Copy + Default + Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, c) = self.shape();
        writeln!(f, "Matrix {{ shape: ({r}, {c})")?;
        for row in 0..self.nrows() {
            writeln!(f, "{:?}", self.row(row))?;
        }
        Ok(())
    }
}

mod tests {
    use super::Matrix;

    #[test]
    fn test_from_vecs() {
        let m = Matrix::from_vecs(&vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ]);

        assert_eq!(m, m.transpose().transpose());

        println!("{:?}", m.col(1).collect::<Vec<_>>());
    }

    #[test]
    fn test_macro() {
        let m = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
            7.0, 8.0, 9.0
        ];

        assert_eq!(m, m.transpose().transpose());

        println!("{:?}", m.col(1).collect::<Vec<_>>());
    }

    #[test]
    fn test_transpose() {
        let m = matrix![1.0, 2.0, 3.0];

        println!("{:?}", m);
        println!("{:?}", m.transpose());
    }

    #[test]
    fn test_set_row() {
        let mut m = matrix![
            1.0, 2.0, 3.0, 10.0;
            4.0, 5.0, 6.0, 20.0;
            7.0, 8.0, 9.0, 30.0
        ];

        m.set_row(0, vec![10.0, 10.0, 10.0, 10.0].into_iter());

        m.set_col(0, vec![100.0, 100.0, 100.0].into_iter());

        println!("row 0 {:?}", m.row(0));
        println!("col 0 {:?}", m.col(0).collect::<Vec<_>>());
    }
}
