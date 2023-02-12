use crate::prelude::*;

pub trait Conformable {
    fn conform(ctx: Context) -> Chu;
}
