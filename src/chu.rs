// use std::fmt::Debug;

use crate::prelude::*;

// enum Standardized {
//     yes,
//     no(Option<Box<Chu>>),
// }
// use std::cmp::

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ordering {
    // < nothing,   > nothing
    Unknown,
    // < something, > nothing
    Initial,
    // < nothing,   > something
    Final,
    // < something, > something
    Middle,
    // == previous something
    Duplicate,
}

impl Default for Ordering {
    fn default() -> Self {
        Self::Unknown
    }
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

    pub fn nrows(&self) -> usize {
        self.shape().0
    }

    pub fn ncols(&self) -> usize {
        self.shape().1
    }

    pub fn row_tree(&self) -> Tree {
        let mut t = Tree::new(self.k, self.ncols());
        for r in 0..self.nrows() {
            let line = self.data.row(r);
            t.add_line(line.into_iter(), r);
        }
        t
    }

    pub fn col_tree(&self) -> Tree {
        let mut t = Tree::new(self.k, self.nrows());
        for c in 0..self.nrows() {
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
            return self.query2();
        }

        // let row =

        let row_tree = self.row_tree();

        for k in 0..self.k {
            //
        }

        unimplemented!()
    }

    fn query2(&self) -> Self {
        // let mut result_rows: Vec<Vec<i32>> = Vec::new();
        // let mut row_tree: HashSet<Vec<i32>> = HashSet::new();
        // let mut future_rows: VecDeque<Vec<i32>> = VecDeque::new();

        // for row in 0..self.nrows() {
        //     future_rows.push_back(row.clone());
        // }

        // let zero_row = vec![0; self.ncols];
        // future_rows.push_back(zero_row.clone());

        // let one_row = vec![1; self.ncols];
        // future_rows.push_back(one_row.clone());

        // while let Some(row) = future_rows.pop_front() {
        //     if !row_tree.contains(&row) {
        //         let old_rows = result_rows.clone();

        //         for old_row in old_rows {
        //             let mut union = vec![0; self.ncols];
        //             for c in 0..self.ncols {
        //                 union[c] = if row[c] == 1 || old_row[c] == 1 { 1 } else { 0 };
        //             }
        //             future_rows.push_back(union);

        //             let mut intersection = vec![0; self.ncols];
        //             for c in 0..self.ncols {
        //                 intersection[c] = if row[c] == 1 && old_row[c] == 1 { 1 } else { 0 };
        //             }
        //             future_rows.push_back(intersection);
        //         }

        //         row_tree.insert(row.clone());
        //         result_rows.push(row);
        //     }
        // }

        // Self::new(2, result_rows.len(), result_rows)
        unimplemented!()
    }

    pub fn choice(&self, other: &Self) -> Self {
        let k = self.k.max(other.k);
        let nrows = self.nrows() + other.nrows();
        let ncols = self.ncols() + other.ncols();

        let mut m = Matrix::new((nrows, ncols));
        for r in 0..nrows {
            for c in 0..ncols {
                m[(r, c)] = if r < self.nrows() {
                    if c < self.ncols() {
                        self[(r, c)]
                    } else {
                        0
                    }
                } else {
                    if c < self.ncols() {
                        0
                    } else {
                        other[(r - self.nrows(), c - self.ncols())]
                    }
                }
            }
        }
        Self::new(k, m, false)
    }

    pub fn seq(&self, other: &Self) -> Self {
        let k = self.k.max(other.k);

        // Classify columns of A and B
        let cls_a = self.classify_cols();
        let cls_b = other.classify_cols();

        // Count rows and columns of answer.
        // A column of the answer consists of the concatenation of
        // a column of A and a column of B. Duplicates are not allowed.
        // The column (state) of A must be final (= FINAL || UNKNOWN).
        // The column (state) of B must be initial (= INITIAL || UNKNOWN).
        let nrows = self.nrows() + other.nrows();
        let mut ncols = 0;

        for ac in 0..self.ncols() {
            if cls_a[ac] == Ordering::Duplicate {
                continue;
            }

            for bc in 0..other.ncols() {
                if cls_b[bc] == Ordering::Duplicate {
                    continue;
                }

                if matches!(cls_a[ac], Ordering::Unknown | Ordering::Final)
                    || matches!(cls_b[ac], Ordering::Unknown | Ordering::Initial)
                {
                    ncols += 1;
                }
            }
        }

        // // Form answer, column by column
        let mut m = Matrix::new((nrows, ncols));
        let (mut r, mut c) = (0, 0);

        for ac in 0..self.ncols() {
            if cls_a[ac] == Ordering::Duplicate {
                continue;
            }

            for bc in 0..other.ncols() {
                if cls_b[bc] == Ordering::Duplicate {
                    continue;
                }

                if matches!(cls_a[ac], Ordering::Unknown | Ordering::Final)
                    || matches!(cls_b[ac], Ordering::Unknown | Ordering::Initial)
                {
                    // Create concatenation of A.matrix[*][ac] and B.matrix[*][bc]
                    for ar in 0..self.nrows() {
                        m[(r, c)] = self[(ar, ac)];
                        r += 1;
                    }

                    for br in 0..other.nrows() {
                        m[(r, c)] = other[(br, bc)];
                        r += 1;
                    }

                    r = 0;
                    c += 1;
                }
            }
        }

        Self::new(k, m, false)
    }

    // classify_cols: Returns an array of integers which classify
    // the columns of a Chu space into the five catagories above.

    pub fn classify_cols(&self) -> Vec<Ordering> {
        let mut cls = vec![Ordering::Unknown; self.ncols()];

        'outer: for c in 0..self.ncols() {
            cls[c] = Ordering::Unknown;

            for d in 0..c {
                // Skip comparisons against duplicates or middle elements.
                if matches!(cls[d], Ordering::Duplicate | Ordering::Middle) {
                    continue;
                }

                // Col c <> col d, so nothing can be inferred.
                let Some(res) = self.compare_cols(c, d) else { continue };

                match res {
                    // Col c == col d, throw out c by classifying it as Ordering::Duplicate.
                    std::cmp::Ordering::Equal => {
                        cls[c] = Ordering::Duplicate;
                        continue 'outer;
                    }

                    // Col c < col d.
                    std::cmp::Ordering::Less => {
                        match cls[c] {
                            Ordering::Unknown => cls[c] = Ordering::Initial,
                            Ordering::Final => cls[c] = Ordering::Middle,
                            _ => {}
                        }
                        match cls[d] {
                            Ordering::Unknown => cls[d] = Ordering::Final,
                            Ordering::Initial => cls[d] = Ordering::Middle,
                            _ => {}
                        }
                    }

                    // Col c > col d.
                    std::cmp::Ordering::Greater => {
                        match cls[c] {
                            Ordering::Unknown => cls[c] = Ordering::Final,
                            Ordering::Initial => cls[c] = Ordering::Middle,
                            _ => {}
                        }
                        match cls[d] {
                            Ordering::Unknown => cls[d] = Ordering::Initial,
                            Ordering::Final => cls[d] = Ordering::Middle,
                            _ => {}
                        }
                    }

                    _ => {}
                }
            } // INNER
        } // OUTER

        cls
    }
}

impl Conformable for Chu {
    fn conform(&self, ctx: Context) -> Self {
        if ctx.standardization {
            // self.
            self.standardsize()
        } else {
            // self
            unimplemented!()
        }
    }
}

impl std::ops::Mul for Chu {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let k = self.k.max(rhs.k);
        let nrows = self.nrows() * rhs.nrows();
        let ncols = self.ncols() + rhs.ncols();

        let mut m = Matrix::new((nrows, ncols));

        let mut r = 0;
        let mut c = 0;
        // Loop over rows of A

        for ar in 0..self.nrows() {
            // Loop over rows of B
            for br in 0..rhs.nrows() {
                // Create concatination of A.matrix[ar] and B.matrix[br]

                for ac in 0..self.ncols() {
                    m[(r, c)] = self[(ar, ac)];
                    c += 1;
                }

                for bc in 0..rhs.ncols() {
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
    pub fn standardsize(&self) -> Self {
        unimplemented!()
    }

    // None == incomparable
    fn compare_cols(&self, col1: usize, col2: usize) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        let mut result = Ordering::Equal;

        for r in 0..self.nrows() {
            match self[(r, col1)].cmp(&self[(r, col2)]) {
                Ordering::Less => match result {
                    Ordering::Equal => result = Ordering::Less,
                    Ordering::Greater => return None,
                    _ => {}
                },
                Ordering::Equal => {
                    //
                }
                Ordering::Greater => match result {
                    Ordering::Equal => result = Ordering::Greater,
                    Ordering::Less => return None,
                    _ => {}
                },
            }
        }

        Some(result)
    }

    pub fn row_sort(&self, unique_rows: &mut [usize]) -> usize {
        let mut num_unique = 0;

        'sort: for r in 0..self.nrows() {
            let (mut l, mut h) = (0, num_unique);

            'search: while l < h {
                let m = (l + h) / 2;

                'compare: for c in 0..self.ncols() {
                    if self[(unique_rows[m], c)] == self[(r, c)] {
                        continue 'compare;
                    }

                    if self[(unique_rows[m], c)] > self[(r, c)] {
                        h = m;
                    } else {
                        l = m + 1;
                    }

                    continue 'search;
                }

                continue 'sort;
            }

            for i in (l + 1..=num_unique).rev() {
                unique_rows[i] = unique_rows[i - 1];
            }
            unique_rows[l] = r;
            num_unique += 1;
        }

        num_unique
    }

    pub fn col_sort(&self, unique_cols: &mut [usize]) -> usize {
        let mut num_unique = 0;

        'sort: for c in 0..self.ncols() {
            let (mut l, mut h) = (0, num_unique);

            'search: while l < h {
                let m = (l + h) / 2;

                'compare: for r in 0..self.nrows() {
                    if self[(r, unique_cols[m])] == self[(r, c)] {
                        continue 'compare;
                    }

                    if self[(r, unique_cols[m])] > self[(r, c)] {
                        h = m;
                    } else {
                        l = m + 1;
                    }

                    continue 'search;
                }

                continue 'sort;
            }

            for i in (l + 1..=num_unique).rev() {
                unique_cols[i] = unique_cols[i - 1];
            }
            unique_cols[l] = c;
            num_unique += 1;
        }

        num_unique
    }

    pub fn implication(&self, other: Self) -> Self {
        let k = self.k.max(other.k);
        // let size = self.rows() * other.cols();

        // let mut mg = MatrixGenerator::new(other.row_tree(), self.col_tree());

        // while mg.next() {
        //     //
        //     let mut num_instances = 1;
        //     // for r in mg.nrows() {
        //     //
        //     // }
        // }
        // unimplemented!()

        let size = self.nrows() * other.ncols();
        let mut transforms: Vec<Vec<i32>> = Vec::new();
        let MG = MatrixGenerator::new(other.row_tree(), self.col_tree());

        // while MG.next() {
        //     let mut num_instances = 1;
        //     for r in 0..MG.nrows() {
        //         let mut l = MG.row_links[r];
        //         let mut length = 0;
        //         while let Some(next_l) = l.next() {
        //             l = next_l;
        //             length += 1;
        //         }
        //         num_instances *= length;
        //     }
        //     for c in 0..MG.ncols() {
        //         let mut l = MG.col_links[c];
        //         let mut length = 0;
        //         while let Some(next_l) = l.next() {
        //             l = next_l;
        //             length += 1;
        //         }
        //         num_instances *= length;
        //     }

        //     let mut transform: Vec<i32> = vec![0; size];
        //     for r in 0..MG.rows() {
        //         for c in 0..MG.cols() {
        //             let row_index = MG.row_links[r].datum();
        //             let row = B.matrix[row_index];
        //             let entry = row[c];
        //             transform[r * MG.cols() + c] = entry;
        //         }
        //     }

        //     for _ in 0..num_instances {
        //         transforms.push(transform.clone());
        //     }
        // }

        // let new_nrows = transforms.len();
        // // let mut matrix: Vec<Vec<i32>> = vec![vec![0; size]; new_nrows];
        // let mut m = Matrix::<usize>::new((new_rows, size));
        // for (i, transform) in transforms.iter().enumerate() {
        //     m[i] = transform.clone();
        // }

        // Chu {
        //     K,
        //     nrows: new_nrows,
        //     ncols: size,
        //     matrix,
        //     transpose: false,
        // }
        unimplemented!()
        // Self::new(k, data, false)
    }
}
