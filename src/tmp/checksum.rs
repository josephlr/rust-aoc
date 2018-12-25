use std::io::BufRead;
use std::iter::once;
use itertools::Itertools;
use itertools::MinMaxResult::*;
use util::{force_lines, force_parse, IterUtil, Ans};

pub trait Checksum {
    fn checksum(iter: impl Iterator<Item = i32>) -> i32;
}

pub struct MinMaxDiff;
impl Checksum for MinMaxDiff {
    fn checksum(iter: impl Iterator<Item = i32>) -> i32 {
        match iter.minmax() {
            NoElements => panic!("Line had no numbers"),
            OneElement(_) => 0,
            MinMax(min, max) => max - min,
        }
    }
}

pub struct EvenDiv;
impl Checksum for EvenDiv {
    fn checksum(iter: impl Iterator<Item = i32>) -> i32 {
        iter.make_clonable().tuple_combinations().filter_map(|(a, b)| {
                if a % b == 0 {
                    Some(a / b)
                } else if b % a == 0 {
                    Some(b / a)
                } else {
                    None
                }
            }).force_single()
    }
}

pub struct SingleElement;
impl Checksum for SingleElement {
    fn checksum(iter: impl Iterator<Item = i32>) -> i32 {
        iter.force_single()
    }
}

impl<T: Checksum> Ans<Phantom> for T {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        force_lines(r)
            .map(|line| {
                let nums = line.split_whitespace().map(force_parse);
                Self::checksum(nums)
            })
            .sum()
    }
}

pub struct FirstDuplicate;
impl Ans for FirstDuplicate {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        let frequencies = force_lines(r).map(force_parse).make_clonable().cycle().scan(0, |state, i: i32| {
            *state += i;
            Some(*state)
        });
        once(0).chain(frequencies).first_duplicate().expect("No dupliacates found")
    }
}

#[allow(dead_code)]
pub struct Phantom;
