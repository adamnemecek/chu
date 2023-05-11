use crate::prelude::*;
/* Generates all matrixes whose rows and columns are taken
 * from the prefix trees passed at construction time.
 *
 * After construction, each successful call to next
 * produces a representation of a new matrix.
 */
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
    row_nodes: Vec<Node>,
    col_nodes: Vec<Node>,

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
    row_links: Vec<Link>,
    col_links: Vec<Link>,
}

impl MatrixGenerator {
    //

    pub fn new(row_tree: Tree, col_tree: Tree) -> Self {
        Self {
            row_tree: Tree::new(0, 0),
            col_tree: Tree::new(0, 0),
            // shape: (0, 0),
            rows: 0,
            cols: 0,
            k: 0,
            row_nodes: vec![],
            col_nodes: vec![],
            current_row: 0,
            current_col: 0,
            current_branch: 0,
            done: false,
            row_links: vec![],
            col_links: vec![],
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
        // unimplemented!()

        // Loop Invariants:
        // The prefixes represented by rowNodes and colNodes
        //   cover the same set of cells and match in all values.
        // This set of cells is always the interval before some cell
        //   in the following "herringbone" order:
        //      1  2  3  4
        //     5   9 10 11
        //     6 12  15 16
        //     7 13 17  19
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
                // Otherwise we try another value for currentBranch
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
        // for(int r=0;r<nrows;r++)
        //   rowLinks[r] = rowNodes[r].link();
        for r in 0..self.rows {
            //
            self.row_links[r] = self.row_nodes[r].link().clone();
        }
        // for(int c=0;c<ncols;c++)
        //   colLinks[c] = colNodes[c].link();
        for c in 0..self.cols {
            //
            self.col_links[c] = self.col_nodes[c].link().clone();
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

        if self.current_row <= self.current_col {
            //
            self.current_col -= 1;

            if self.current_row == self.current_col + 1 {
                self.current_row = self.rows - 1;
            }
        } else {
            //
            self.current_row -= 1;

            if self.current_row == self.current_col {
                self.current_col = self.cols - 1;
            }
        }

        unimplemented!()
    }

    pub fn forward(&mut self) -> bool {
        unimplemented!()
    }
}

// #[test]
mod tests {
    #[test]
    fn testa() {
        //
    }
}
