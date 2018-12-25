use std::iter;

use crate::util::{Answer, ByLine, IterExtra, Result};

pub struct Sum;
impl Answer for Sum {
    type Input = ByLine<i32>;
    type Output = i32;
    fn ans(&self, inputs: impl Iterator<Item = i32>) -> Result<Self::Output> {
        Ok(inputs.sum())
    }
}

pub struct FirstRepeat;
impl Answer for FirstRepeat {
    type Input = ByLine<i32>;
    type Output = i32;
    fn ans(&self, inputs: impl Iterator<Item = i32>) -> Result<Self::Output> {
        let frequencies = inputs.make_clonable().cycle().scan(0, |state, i: i32| {
            *state += i;
            Some(*state)
        });
        Ok(iter::once(0)
            .chain(frequencies)
            .duplicates()
            .next()
            .expect("chain should create infinite iterator"))
    }
}
