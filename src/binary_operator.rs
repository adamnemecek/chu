use crate::prelude::*;

pub trait BinOp {
    fn apply(lhs: Chu, rhs: Chu) -> Chu;

    fn apply1(lhs: Chu, rhs: Chu, ctx: Context) -> Chu {
        unimplemented!()
    }
}
