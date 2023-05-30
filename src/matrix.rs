use std::{
    fmt::Debug,
    iter::FromFn,
};

#[derive(PartialEq, Eq)]
pub struct Matrix<T: Copy + Default> {
    shape: (usize, usize),

    pub data: Vec<T>,
}

// fn t<T>(a: (T, T)) -> (T, T) {
//     (a.1, a.0)
// }

impl<T: Copy + Default> Matrix<T> {
    pub fn new(shape: (usize, usize)) -> Self {
        Self {
            shape,
            data: vec![T::default(); shape.0 * shape.1],
        }
    }

    pub fn fill(&mut self, f: impl Fn((usize, usize)) -> T) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                let r = f((i, j));
                self[(i, j)] = r;
            }
        }
    }

    pub fn new_with_fill(shape: (usize, usize), f: impl Fn((usize, usize)) -> T) -> Self {
        let mut m = Self::new(shape);
        m.fill(f);
        m
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn row_range(&self, row: usize) -> std::ops::Range<usize> {
        assert!(row <= self.nrows());
        let start = row * self.shape.1;
        start..(start + self.shape.1)
    }

    pub fn nrows(&self) -> usize {
        self.shape().0
    }

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

    pub fn col(&self, col: usize) -> &[T] {
        // &self.data[self.c
        // self.data.co
        unimplemented!()
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
        Self::new_with_fill((self.shape.1, self.shape.0), |(i, j)| self[(j, i)])
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

// impl<T: Copy + Default + PartialEq> PartialEq for Matrix<T> {
//     fn eq(&self, other: &Self) -> bool {
//         true
//     }
// }

impl<T: Copy + Default + Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for row in 0..self.shape.0 {
            writeln!(f, "{:?}", self.row(row))?;
        }
        Ok(())
        // self.data.fmt(f)
    }
}

// #[test]
// mod tests {

//     // fn test_
// }

// #[derive(Clone)]
// #[stable(feature = "iter_from_fn", since = "1.34.0")]
// pub struct FromFn<F>(F);

// #[stable(feature = "iter_from_fn", since = "1.34.0")]
// impl<T, F> Iterator for FromFn<F>
// where
//     F: FnMut() -> Option<T>,
// {
//     type Item = T;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         (self.0)()
//     }
// }

struct ExactFromFn<F> {
    len: usize,
    i: FromFn<F>,
}

impl<T, F: FnMut() -> Option<T>> ExactFromFn<F> {
    pub fn new(len: usize, f: F) -> Self {
        Self {
            len,
            i: std::iter::from_fn(f),
        }
    }
}

impl<T, F: FnMut() -> Option<T>> Iterator for ExactFromFn<F> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.i.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, self.len.into())
    }
}

impl<T, F: FnMut() -> Option<T>> ExactSizeIterator for ExactFromFn<F> {
    fn len(&self) -> usize {
        self.len
    }
}
