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
    shape: (usize, usize),
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
            shape: (0, 0),
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
        self.shape
    }

    pub fn k(&self) -> usize {
        unimplemented!()
    }

    pub fn next(&self) -> bool {
        unimplemented!()
    }

    pub fn backwards(&self) -> bool {
        unimplemented!()
    }

    pub fn forward(&self) -> bool {
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
