use crate::prelude::*;

pub trait Conformable {
    fn conform(&self, ctx: Context) -> Chu;
}
