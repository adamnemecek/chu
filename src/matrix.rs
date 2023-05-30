use std::fmt::Debug;

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
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn row_range(&self, row: usize) -> std::ops::Range<usize> {
        let start = row * self.shape.1;
        start..(start + self.shape.1)
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
        let mut ret = Self::new((self.shape.1, self.shape.0));
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                ret[(i, j)] = self[(j, i)];
            }
        }
        ret
    }

    fn offset(&self, index: (usize, usize)) -> usize {
        self.shape.0 * index.0 + index.1
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
    }
}

// #[test]
// mod tests {

//     // fn test_
// }
