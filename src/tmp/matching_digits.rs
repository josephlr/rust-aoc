use std::io::BufRead;
use std::iter::once;
use itertools::Itertools;
use util::Ans;

fn digits(r: impl BufRead) -> impl Iterator<Item = u8> {
    r.bytes().filter_map(|b| match b.unwrap() {
        b @ b'0'...b'9' => Some(b - b'0'),
        b'\n' => None,
        b => panic!("Found non-digit {:?} in input", b as char),
    })
}

fn sum_matching(pairs: impl Iterator<Item = (u8, u8)>) -> i32 {
    pairs.filter(|p| p.0 == p.1).map(|p| p.0 as i32).sum()
}

pub struct Sequential;
impl Ans for Sequential {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        let mut digits = digits(r).peekable();
        let &first = digits.peek().expect("No input provided");

        let sequential_pairs = digits.chain(once(first)).tuple_windows();
        sum_matching(sequential_pairs)
    }
}

pub struct Halfway;
impl Ans for Halfway {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        let v = digits(r).collect_vec();
        assert!(v.len() % 2 == 0, "Input length of {} is not even", v.len());

        let (half1, half2) = v.split_at(v.len() / 2);
        let halfway_pairs = half1.iter().cloned().zip(half2.iter().cloned());
        2 * sum_matching(halfway_pairs)
    }
}
