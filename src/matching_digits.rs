use itertools::Itertools;
use std::io::{Read, BufRead};
use std::iter::once;
use ans::Ans;

fn digits(r: impl BufRead) -> impl Iterator<Item = u8> {
    r.bytes().filter_map(|b| match b.unwrap() {
        b @ b'0'...b'9' => Some(b - b'0'),
        b'\n' => None,
        b => panic!("Found non-digit {:?} in input", b as char),
    })
}

fn sum_matching(pairs: impl Iterator<Item = (u8, u8)>) -> u32 {
    pairs.filter(|p| p.0 == p.1).map(|p| p.0 as u32).sum()
}

pub struct Sequential();
impl Ans for Sequential {
    type Value = u32;
    fn compute(&self, r: impl BufRead) -> Self::Value {
        let mut digits = digits(r).peekable();
        let &first = digits.peek().expect("No input provided");

        let sequential_pairs = digits.chain(once(first)).tuple_windows();
        sum_matching(sequential_pairs)
    }
}

pub struct Halfway();
impl Ans for Halfway {
    type Value = u32;
    fn compute(&self, r: impl BufRead) -> Self::Value {
        let v = digits(r).collect_vec();
        assert!(v.len() % 2 == 0, "Input length of {} is not even", v.len());

        let start = v.iter().cloned();
        let halfway = v[v.len() / 2..].iter().cloned();

        let halfway_pairs = halfway.chain(start.clone()).zip(start);
        sum_matching(halfway_pairs)
    }
}
