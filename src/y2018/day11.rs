use itertools::iproduct;

use crate::util::{Answer, ByWhitespace, IterExtra, Result};

struct Grid {
    size: usize,
    levels: Vec<i32>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            size,
            levels: vec![0; size * size],
        }
    }
    fn compute(&mut self, serial: i32) {
        for (x, y) in iproduct!(0..self.size, 0..self.size) {
            let rack_id = (x + 1) as i32 + 10;
            let mut power = rack_id * (y + 1) as i32;
            power += serial;
            power *= rack_id;
            power = (power / 100) % 10;
            power -= 5;

            self.levels[x * self.size + y] = power;
        }
    }
    fn find(&self, sz: usize) -> (usize, usize) {
        let points = iproduct!(0..=(self.size - sz), 0..=(self.size - sz));
        points
            .max_by_key(|&(x, y)| -> i32 {
                iproduct!(x..(x + sz), y..(y + sz))
                    .map(|(xi, yi)| self.levels[xi * self.size + yi])
                    .sum()
            })
            .unwrap()
    }

    fn overall_find(&self) -> (usize, usize, usize) {
        (1..(self.size))
            .flat_map(|sz| {
                iproduct!(0..=(self.size - sz), 0..=(self.size - sz)).map(move |(x, y)| (x, y, sz))
            })
            .max_by_key(|&(x, y, sz)| -> i32 {
                iproduct!(x..(x + sz), y..(y + sz))
                    .map(|(xi, yi)| self.levels[xi * self.size + yi])
                    .sum()
            })
            .unwrap()
    }
}

pub struct LargestPower(pub usize, pub usize);
impl Answer for LargestPower {
    type Input = ByWhitespace<i32>;
    type Output = String;
    fn ans(&self, inputs: impl Iterator<Item = i32>) -> Result<Self::Output> {
        let mut g = Grid::new(self.0);
        g.compute(inputs.single()?);
        let (x, y) = g.find(self.1);
        Ok(format!("{},{}", x + 1, y + 1))
    }
}

pub struct Overall(pub usize);
impl Answer for Overall {
    type Input = ByWhitespace<i32>;
    type Output = String;
    fn ans(&self, inputs: impl Iterator<Item = i32>) -> Result<Self::Output> {
        let mut g = Grid::new(self.0);
        g.compute(inputs.single()?);
        let (x, y, sz) = g.overall_find();
        Ok(format!("{},{},{}", x + 1, y + 1, sz))
    }
}
