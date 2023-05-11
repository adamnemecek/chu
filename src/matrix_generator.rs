use crate::prelude::*;
//
// Generates all matrixes whose rows and columns are taken
// from the prefix trees passed at construction time.
//
// After construction, each successful call to next
// produces a representation of a new matrix.
//

pub struct MatrixGenerator {
    // prefix tree of rows
    row_tree: Tree,
    // prefix tree of columns
    col_tree: Tree,
    // shape: (usize, usize),
    rows: usize,
    cols: usize,

    k: usize,

    // The search algorithm works by trial extension of a region
    // of overlapping partial rows and columns.  The two arrays
    // below represent that region by locating the Node for
    // each partial row and column.
    row_nodes: Vec<NodeRef>,
    col_nodes: Vec<NodeRef>,

    // more search variables:  current(Row/Col/Branch)
    // these variables give the cell we are trying to fill,
    // and the value we are trying to fill it with.
    current_row: usize,
    current_col: usize,
    current_branch: usize,

    // if done is true then there are no more matricies
    done: bool,

    // These arrays represent a matrix.  (The arrays point
    // to lists of indexes of lines that form the matrix.)
    // After a successful call to next(), the caller
    // can examine these arrays to extract the matrix.
    row_links: Vec<Option<Link>>,
    col_links: Vec<Option<Link>>,
}

impl MatrixGenerator {
    //

    pub fn new(row_tree: Tree, col_tree: Tree) -> Self {
        let rows = col_tree.len();
        let cols = row_tree.len();

        // let row_nodes =
        let row_links = vec![None; rows];
        let col_links = vec![None; cols];

        Self {
            rows,
            cols,
            k: row_tree.arity(),
            row_tree,
            col_tree,

            row_nodes: vec![],
            col_nodes: vec![],
            current_row: 0,
            current_col: 0,
            current_branch: 0,
            done: false,
            row_links,
            col_links,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        // self.shape
        unimplemented!()
    }

    pub fn k(&self) -> usize {
        unimplemented!()
    }

    // next: Try to find the next morphism
    // If there is no such morphism, return false
    // If there is such a morphism, put lists of the
    // possible rows and columns into rowLinks, colLinks,
    // then return true.
    pub fn next(&mut self) -> bool {
        // Loop Invariants:
        // The prefixes represented by rowNodes and colNodes
        //   cover the same set of cells and match in all values.
        // This set of cells is always the interval before some cell
        //   in the following "herringbone" order:
        //     1  2  3  4
        //     5  9 10 11
        //     6 12 15 16
        //     7 13 17 19
        //     8 14 18 20

        if self.done {
            return false;
        }

        // Outer loop: drive search forward, extending matrix,
        // check for when we go out of bounds.

        'outer: while self.current_row < self.rows && self.current_col < self.cols {
            // Inner loop: go forward one step.
            // Back up as many cells as needed before taking a forward step.
            loop {
                // If all possibilities for this cell are exhausted,
                // then back up until it is possible to go forward.
                // If we have to back up and fail, then return false.
                while self.current_branch == self.k {
                    let success = self.backward();
                    if !success {
                        return false;
                    }
                }

                // If we succeed in going forward, then we re-test bounds.
                // Otherwise we try another value for current_branch
                let success = self.forward();
                if success {
                    continue 'outer;
                } else {
                    self.current_branch += 1;
                }
            }
        }

        // If we get here, the search went out of bounds.
        // Thus we have a matrix to record.
        for r in 0..self.rows {
            //
            self.row_links[r] = Some(self.row_nodes[r].link().clone());
        }
        for c in 0..self.cols {
            //
            self.col_links[c] = Some(self.col_nodes[c].link().clone());
        }

        // move search one step beyond this morphism
        // then return true to indicate we have a morphism
        self.backward();
        true
    }

    pub fn backward(&mut self) -> bool {
        if self.current_row == 0 && self.current_col == 0 {
            //
            self.done = true;
            return false;
        }

        // First step currentRow, currentCol backward
        if self.current_row <= self.current_col {
            // Shrink a row leftwards
            self.current_col -= 1;

            // If the row is entirely empty,
            //  then go to the end of the previous column.

            if self.current_row == self.current_col + 1 {
                self.current_row = self.rows - 1;
            }
        } else {
            // Shrink a column upwards
            self.current_row -= 1;

            // If the column is entirely empty,
            //  then go to the end of the previous row.

            if self.current_row == self.current_col {
                self.current_col = self.cols - 1;
            }
        }

        // Second, restore currentBranch and the prefix trees
        self.current_branch = self.row_nodes[self.current_row].branch();
        self.current_branch += 1;
        self.row_nodes[self.current_row] = self.row_nodes[self.current_row].parent();
        self.col_nodes[self.current_col] = self.col_nodes[self.current_col].parent();

        true
    }

    pub fn forward(&mut self) -> bool {
        // Try the current value of branch in the current cell
        let Some(rn) = self.row_nodes[self.current_row].child(self.current_branch) else { return false};
        let Some(cn) = self.col_nodes[self.current_col].child(self.current_branch) else { return false};

        // If it doesn't work, then report failure
        // if (rn == null || cn == null) return false;

        // First update current_branch and the prefix trees
        self.row_nodes[self.current_row] = rn;
        self.col_nodes[self.current_col] = cn;
        self.current_branch = 0;

        // Second, step current_row, current_col forward
        if self.current_row <= self.current_col {
            self.current_col += 1; // Grow a row rightward

            // If the row is entirely full,
            //  then go to the start of the next column.
            if self.current_col == self.cols {
                self.current_col = self.current_row;
                self.current_row = self.current_col + 1;
            }
        } else {
            self.current_row += 1; // Grow a column downward

            // If the column is entirely full,
            //  then go to the start of the next row.
            if self.current_row == self.rows {
                self.current_row = self.current_col + 1;
                self.current_col = self.current_row;
            }
        }

        true // Report Success
    }
}

// #[test]
mod tests {
    #[test]
    fn testa() {
        //
    }
}
