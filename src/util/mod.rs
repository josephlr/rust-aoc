mod error;
pub use self::error::*;
mod iter;
pub use self::iter::Extra as IterExtra;
mod runner;
pub use self::runner::*;

use std::{
    cmp::{max, min},
    ops::Range,
};

pub fn overlap<Idx: Ord>(a: Range<Idx>, b: Range<Idx>) -> Option<Range<Idx>> {
    let start = max(a.start, b.start);
    let end = min(a.end, b.end);
    if start > end {
        return None;
    }
    Some(Range { start, end })
}
