use itertools::Itertools;
use itertools::MinMaxResult::*;
use std::io::BufRead;
use ans::Ans;

trait Checksum {
    fn checksum(iter: impl Iterator<Item = u32>) -> u32;
    fn sum_checksums(r: impl BufRead) -> u32 {
        r.lines()
            .map(|line| {
                let line = line.unwrap();
                let iter = line.split_whitespace().map(|word| {
                    word.parse().unwrap_or_else(
                        |e| panic!("{}: {:?} is not a number", e, word),
                    )
                });
                Self::checksum(iter)
            })
            .sum()
    }
}

pub struct MinMaxDiff();
impl Checksum for MinMaxDiff {
    fn checksum(iter: impl Iterator<Item = u32>) -> u32 {
        match iter.minmax() {
            NoElements => panic!("Line had no numbers"),
            OneElement(_) => 0,
            MinMax(min, max) => max - min,
        }
    }
}
impl Ans for MinMaxDiff {
    type Value = u32;
    fn compute(&self, r: impl BufRead) -> u32 {
        Self::sum_checksums(r)
    }
}

pub struct EvenDiv();
impl Checksum for EvenDiv {
    fn checksum(iter: impl Iterator<Item = u32>) -> u32 {
        let v = iter.collect_vec();
        let mut iter = v.iter().tuple_combinations().filter_map(
            |(a, b)| if a % b == 0 {
                Some(a / b)
            } else if b % a == 0 {
                Some(b / a)
            } else {
                None
            },
        );
        match (iter.next(), iter.next()) {
            (None, _) => panic!("No even divisors found"),
            (Some(ans), None) => ans,
            (Some(_), Some(_)) => panic!("Multiple even divisors found"),
        }
    }
}
impl Ans for EvenDiv {
    type Value = u32;
    fn compute(&self, r: impl BufRead) -> u32 {
        Self::sum_checksums(r)
    }
}
