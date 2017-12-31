use std::cmp::max;
use std::io::BufRead;
use util::{force_parse_line, Ans};

// Returns n s.t. x ∈ [n^2, (n+1)^2)
// Note: this would not work for an i64 as f64 only has 53 bits of precision.
fn isqrt(x: i32) -> i32 {
    (x as f64).sqrt() as i32
}

fn to_point(spiral: usize) -> (i32, i32) {
    if spiral == 1 {
        return (0, 0);
    }
    let spiral = spiral as i32;
    // r is the ring containg spiral:          (2r-1)^2 < spiral <= (2r+1)^2
    let r = (isqrt(spiral - 1) + 1) / 2;
    // c is the 1-indexed count in ring r:     c ∈ (0, 8r]
    let c = spiral - (2 * r - 1).pow(2);
    match (c - 1) / (2 * r) {
        0 => (r, c - r),      // right:        c ∈ (0 , 2r]
        1 => (3 * r - c, r),  // top:          c ∈ (2r, 4r]
        2 => (-r, 5 * r - c), // left:         c ∈ (4r, 6r]
        3 => (c - 7 * r, -r), // bottom:       c ∈ (6r, 8r]
        _ => unreachable!(),
    }
}

fn to_spiral(x: i32, y: i32) -> usize {
    // r is the ring containing (x,y):         x or y should be ±r
    let r = max(x.abs(), y.abs());
    // c is the 1-indexed count in ring r:     c ∈ (0, 8r]
    let c = match () {
        _ if y == r => 3 * r - x,  // top:     c ∈ [2r, 4r]
        _ if y == -r => 7 * r + x, // bottom:  c ∈ [6r, 8r]
        _ if x == r => r + y,      // right:   c ∈ (0 , 2r)
        _ if x == -r => 5 * r - y, // left:    c ∈ (4r, 6r)
        _ => unreachable!(),
    };
    (c + (2 * r - 1).pow(2)) as usize
}

pub struct CountSteps;
impl Ans for CountSteps {
    type Value = i32;
    fn compute(&self, mut r: impl BufRead) -> i32 {
        let (x, y) = to_point(force_parse_line(&mut r));
        x.abs() + y.abs()
    }
}

#[derive(Default)]
struct SpiralIter {
    computed: Vec<usize>,
}

fn compute_next_value(computed: &Vec<usize>) -> usize {
    if computed.is_empty() {
        return 1;
    }
    let (a, b) = to_point(computed.len() + 1);

    let mut sum = 0;
    for x in (a - 1)..=(a + 1) {
        for y in (b - 1)..=(b + 1) {
            let spiral = to_spiral(x, y);
            if spiral <= computed.len() {
                sum += computed[spiral - 1];
            }
        }
    }
    sum
}

impl Iterator for SpiralIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let v = compute_next_value(&self.computed);
        self.computed.push(v);
        Some(v)
    }
}

pub struct FirstValue;
impl Ans for FirstValue {
    type Value = usize;
    fn compute(&self, mut r: impl BufRead) -> usize {
        let input = force_parse_line(&mut r);
        SpiralIter::default().filter(|&x| x > input).next().unwrap()
    }
}
