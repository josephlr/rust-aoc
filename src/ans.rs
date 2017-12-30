use std::io;
use std::fmt;

pub trait Ans {
    type Value: fmt::Display;

    fn compute(&self, r: impl io::BufRead) -> Self::Value;
}
