use std::io::BufRead;
use itertools::Itertools;
use util::{force_parse, Ans};

pub struct Adjust(pub fn(i64) -> i64);
impl Ans for Adjust {
    type Value = usize;
    fn compute(&self, r: impl BufRead) -> usize {
        let mut list = r.lines()
            .map(|line| force_parse(&line.unwrap()))
            .collect_vec();

        let mut jumps = 0;
        let mut index = 0;
        while index >= 0 && (index as usize) < list.len() {
            let jump = list[index as usize];
            list[index as usize] = self.0(jump);
            index += jump;
            jumps += 1;
        }
        jumps
    }
}
