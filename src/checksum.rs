use std::io::BufRead;
use itertools::Itertools;
use itertools::MinMaxResult::*;
use util::{force_parse, Ans};

trait Checksum {
    fn checksum(iter: impl Iterator<Item = i32>) -> i32;
    fn sum_checksums(r: impl BufRead) -> i32 {
        r.lines()
            .map(|line| {
                let line = line.unwrap();
                let iter = line.split_whitespace().map(force_parse);
                Self::checksum(iter)
            })
            .sum()
    }
}

pub struct MinMaxDiff();
impl Checksum for MinMaxDiff {
    fn checksum(iter: impl Iterator<Item = i32>) -> i32 {
        match iter.minmax() {
            NoElements => panic!("Line had no numbers"),
            OneElement(_) => 0,
            MinMax(min, max) => max - min,
        }
    }
}

pub struct EvenDiv();
impl Checksum for EvenDiv {
    fn checksum(iter: impl Iterator<Item = i32>) -> i32 {
        let v = iter.collect_vec();
        let mut iter = v.iter().tuple_combinations().filter_map(|(a, b)| {
            if a % b == 0 {
                Some(a / b)
            } else if b % a == 0 {
                Some(b / a)
            } else {
                None
            }
        });
        match (iter.next(), iter.next()) {
            (None, _) => panic!("No even divisors found"),
            (Some(ans), None) => ans,
            (Some(_), Some(_)) => panic!("Multiple even divisors found"),
        }
    }
}

// Cannot use a generic impl Ans here as that would conflict with other mods.
impl Ans for MinMaxDiff {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        Self::sum_checksums(r)
    }
}
impl Ans for EvenDiv {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        Self::sum_checksums(r)
    }
}
