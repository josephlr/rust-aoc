use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use itertools::Itertools;
use util::{force_parse, force_string, Ans};

fn banks(r: impl BufRead) -> Vec<usize> {
    force_string(r)
        .split_whitespace()
        .map(force_parse)
        .collect_vec()
}

fn reallocate(banks: &mut [usize]) {
    let mut max = 0;
    let mut max_idx = 0;
    for (idx, &bank) in banks.iter().enumerate() {
        if bank > max {
            max = bank;
            max_idx = idx;
        }
    }
    banks[max_idx] = 0;

    let num = banks.len();
    for i in 1..=max % num {
        banks[(i + max_idx) % num] += 1;
    }
    for bank in banks {
        *bank += max / num;
    }
}

pub struct RepeatIdx;
impl Ans for RepeatIdx {
    type Value = usize;
    fn compute(&self, r: impl BufRead) -> usize {
        let mut banks = banks(r);
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(banks.clone()) {
                return seen.len();
            }
            reallocate(&mut banks);
        }
    }
}

pub struct CycleCount;
impl Ans for CycleCount {
    type Value = usize;
    fn compute(&self, r: impl BufRead) -> usize {
        let mut banks = banks(r);
        let mut seen = HashMap::new();
        loop {
            if let Some(first) = seen.insert(banks.clone(), seen.len()) {
                return seen.len() - first;
            }
            reallocate(&mut banks);
        }
    }
}
