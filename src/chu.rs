use crate::prelude::*;

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
    std: Option<Matrix<usize>>,
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

    pub fn row(&self, index: usize) -> &[usize] {
        self.data.row(index)
    }

    pub fn ncols(&self) -> usize {
        self.shape().1
    }

    pub fn row_tree(&self) -> Tree {
        let mut t = Tree::new(self.k, self.ncols());
        for r in 0..self.nrows() {
            t.add_line(self.row(r).iter(), r);
        }
        t
    }

    pub fn col_tree(&self) -> Tree {
        let mut t = Tree::new(self.k, self.nrows());
        for c in 0..self.ncols() {
            t.add_line(self.data.col(c), c);
        }
        t
    }

    pub fn shape(&self) -> (usize, usize) {
        self.data.shape()
    }

    //
    pub fn dual(&self) -> Self {
        let s = self.std.as_ref().map_or(false, |x| &self.data == x);
        Self::new(self.k, self.data.transpose(), s)
    }

    // query: The rows of `A` are closed under the following operation:
    // Form a square matrix whose rows and columns are rows of A, and
    // build a new row from the diagonal.  The implementation below
    // simply performs this operation repeatedly until there is nothing
    // new generated.
    pub fn query(&self) -> Self {
        if self.k == 2 {
            return self.query2();
        }

        // The final number of rows is unknown,
        // so for now hold them in a Vector.
        let mut result_row = vec![];

        for r in 0..self.nrows() {
            result_row.push(self.row(r).to_vec());
        }

        // row_tree holds the same rows as result_rows.
        //  (the Tree form is useful for feeding the MatrixGenerator)
        let mut row_tree = self.row_tree();

        let mut const_row = vec![0; self.ncols()];

        // ?A must contain all constant rows
        for k in 0..self.k {
            const_row.clear();
            const_row.fill(k);

            if row_tree.find_line(&const_row).is_none() {
                //
                row_tree.add_line(const_row.iter(), result_row.len());
                result_row.push(const_row.clone());
            }
        }

        loop {
            //
            // let mut future_rows = vec![];

            let mut mg = MatrixGenerator::new(&row_tree, &row_tree);

            while mg.next() {
                let mut diag = vec![0; self.ncols()];

                for i in 0..self.ncols() {
                    //
                    // datum
                    let row_index = mg.row_link(i).unwrap();
                    let datum = row_index.borrow().front();
                    // result_ro
                }
                // future_rows.push(diagonal);
            }
            //
        }

        unimplemented!()
    }

    // query2: Closes the rows of A under union and instersection.
    fn query2(&self) -> Self {
        //
        // The final number of rows is unknown,
        // so for now hold them in a Vector.
        //
        let mut result_rows: Vec<Vec<usize>> = vec![];
        //
        // row_tree holds the same rows as result_rows.
        // (The purpose of the Tree is simply to make
        // checking for duplicates faster)
        //
        let mut row_tree = Tree::new(2, self.ncols());
        let mut future_rows = Stack::new();

        //
        // Put all the rows of original space on the stack
        //
        for row in 0..self.nrows() {
            future_rows.push(self.row(row).to_vec());
        }

        //
        // Don't forget the union and intersection of the empty set of rows:
        //
        let zero_row = vec![0usize; self.ncols()];
        future_rows.push(zero_row);

        let one_row = vec![1usize; self.ncols()];
        future_rows.push(one_row);

        //
        // Loop until no rows remain to insert
        //
        while let Some(row) = future_rows.pop() {
            if row_tree.find_line(&row).is_some() {
                continue;
            };
            for old_row in &result_rows {
                let mut union = vec![0usize; self.ncols()];
                let mut intersection = vec![0usize; self.ncols()];

                for c in 0..self.ncols() {
                    union[c] = (row[c] == 1 || old_row[c] == 1).into();
                    intersection[c] = (row[c] == 1 && old_row[c] == 1).into();
                }
                future_rows.push(union);
                future_rows.push(intersection);
            }
            row_tree.add_line(row.iter(), result_rows.len());
            result_rows.push(row);
        }

        // let mut m = Matrix::new((result_rows.len(), self.ncols()));
        let m = Matrix::from_vecs(&result_rows);
        assert!(m.shape() == (result_rows.len(), self.ncols()));
        Self::new(2, m, false)
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
        let mut r = 0;
        let mut c = 0;

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
            }
        }

        cls
    }
}

impl Conformable for Chu {
    fn conform(&self, ctx: Context) -> Self {
        if ctx.standardization {
            // self.
            // self.standardize()
            unimplemented!()
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
    pub fn standardize(&mut self) {
        if self.std.is_some() {
            return;
        }

        let mut unique_rows = vec![0; self.nrows()];
        let mut unique_cols = vec![0; self.ncols()];

        let new_nrows = self.row_sort(&mut unique_rows);
        let new_ncols = self.col_sort(&mut unique_cols);

        if self.shape() == (new_nrows, new_ncols) {
            return;
        }

        self.std = Some(Matrix::with_fill((new_nrows, new_ncols), |(i, j)| {
            self[(unique_rows[i], unique_cols[j])]
        }));
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
        //
        // Record(unique_rows) and count(num_unique) all unique rows.
        // Throw out all copies.
        //
        let mut num_unique = 0;

        'sort: for r in 0..self.nrows() {
            // Look for row r in the current set of unique rows.
            // If row r is not a copy, insert it into the set.
            // l,h mark bounds of possible insertion locations
            //
            let mut l = 0;
            let mut h = num_unique;

            'search: while l < h {
                //
                // Does row unique_rows[m] match row r?
                //
                let m = (l + h) / 2;

                'compare: for c in 0..self.ncols() {
                    //
                    // scan quickly for differences
                    //
                    if self[(unique_rows[m], c)] == self[(r, c)] {
                        continue 'compare;
                    }

                    //
                    // row unique_rows[m] does not match row r.
                    // narrow range and continue search.
                    //
                    if self[(unique_rows[m], c)] > self[(r, c)] {
                        h = m
                    } else {
                        l = m + 1
                    };

                    continue 'search;
                }

                //
                // If we get here, we have a match.
                // Throw out row r
                //
                continue 'sort;
            }

            //
            // We have a new row.  Insert it!
            //
            for i in (l + 1..=num_unique).rev() {
                unique_rows[i] = unique_rows[i - 1];
            }
            unique_rows[l] = r;
            num_unique += 1;
        }

        num_unique
    }

    pub fn col_sort(&self, unique_cols: &mut [usize]) -> usize {
        //
        // Record (in unique_cols) and count (in num_unique) all unique cols.
        // Throw out all copies.
        //
        let mut num_unique = 0;

        'sort: for c in 0..self.ncols() {
            //
            // Look for col c in the current set of unique cols.
            // If col c is not a copy, insert it into the set.
            // l,h mark bounds of possible insertion locations
            //
            let mut l = 0;
            let mut h = num_unique;

            'search: while l < h {
                //
                // Does col unique_cols[m] match col c?
                //
                let m = (l + h) / 2;

                'compare: for r in 0..self.nrows() {
                    //
                    // scan quickly for differences
                    //
                    if self[(r, unique_cols[m])] == self[(r, c)] {
                        continue 'compare;
                    }
                    //
                    // col unique_rcols[m] does not match col c.
                    // narrow range and continue search.
                    //
                    if self[(r, unique_cols[m])] > self[(r, c)] {
                        h = m
                    } else {
                        l = m + 1
                    };

                    //
                    // If we get here, we have a match.
                    // Throw out col c
                    //
                    continue 'search;
                }

                continue 'sort;
            }

            //
            // We have a new col.  Insert it!
            //
            for i in (l + 1..=num_unique).rev() {
                unique_cols[i] = unique_cols[i - 1];
            }
            unique_cols[l] = c;
            num_unique += 1;
        }

        num_unique
    }

    pub fn implication(&self, other: &Self) -> Self {
        let k = self.k.max(other.k);
        //
        // The "rows" of implication are Chu transforms from A to B
        // These transforms consist of matrices that are ambigiously
        // composed of columns of A or rows of B.  Thus the size of
        // these rows/transforms/matrices is:
        //
        let row_tree = other.row_tree();
        let col_tree = self.col_tree();
        let mut mg = MatrixGenerator::new(&row_tree, &col_tree);

        let size = self.nrows() * other.ncols();
        let mut transforms: Vec<Vec<usize>> = vec![];

        while mg.next() {
            let mut num_instances = 1;

            for r in 0..mg.nrows() {
                let l = mg.row_link(r).unwrap();
                num_instances *= l.borrow().iter().count();
            }

            for c in 0..mg.ncols() {
                let l = mg.col_link(c).unwrap();
                num_instances *= l.borrow().iter().count();
            }

            let mut transform: Vec<usize> = vec![0; size];

            for r in 0..mg.nrows() {
                for c in 0..mg.ncols() {
                    let row_index = unimplemented!();
                    // let row_index = mg.row_link(r).unwrap().datum();
                    let row = other.row(row_index);
                    let entry = row[c];
                    transform[r * mg.ncols() + c] = entry;
                }
            }

            for _ in 0..num_instances {
                transforms.push(transform.clone());
            }
        }

        let m = Matrix::from_vecs(&transforms);

        Self::new(k, m, false)
    }
}

mod tests {
    #[test]
    fn test() {
        //
    }
}
