use std::{cmp::Ordering, str::FromStr};

use nom::*;

use crate::util::{number, to_result, Answer, ByLine, Error, Result};

pub struct Point(pub i32, pub i32);

named!(pair<&str, (i32, i32)>, separated_pair!(number, tag!(", "), number));

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = to_result(pair(s))?;
        Ok(Point(x, y))
    }
}

fn manhattan(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

const NO_POINTS: Error = Error::Custom("No points in input");

pub struct LargestFinite(pub i32, pub i32);
impl Answer for LargestFinite {
    type Input = ByLine<Point>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Point>) -> Result<Self::Output> {
        let points = inputs.collect::<Vec<_>>();
        let mut counts = vec![Some(0); points.len()];

        for x in self.0..=self.1 {
            for y in self.0..=self.1 {
                // Find the closest point
                let p = Point(x, y);
                let mut idx = 0;
                let mut best_distance = i32::max_value();
                let mut num_ties = 0;

                for (i, q) in points.iter().enumerate() {
                    let d = manhattan(&p, q);
                    match d.cmp(&best_distance) {
                        Ordering::Less => {
                            num_ties = 1;
                            best_distance = d;
                            idx = i;
                        }
                        Ordering::Equal => {
                            num_ties += 1;
                        }
                        Ordering::Greater => {}
                    };
                }
                if num_ties == 1 {
                    if let Some(x) = counts[idx].as_mut() {
                        *x += 1;
                    }
                }

                // Kill any on the edges
                if x == self.0 || y == self.0 || x == self.1 || y == self.1 {
                    counts[idx] = None
                }
            }
        }

        counts.into_iter().filter_map(|x| x).max().ok_or(NO_POINTS)
    }
}

pub struct Close(pub i32, pub i32, pub i32);
impl Answer for Close {
    type Input = ByLine<Point>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Point>) -> Result<Self::Output> {
        let points = inputs.collect::<Vec<_>>();
        let mut num_points = 0;

        for x in self.0..=self.1 {
            for y in self.0..=self.1 {
                // Sum manhattan distance
                let p = Point(x, y);
                let s: i32 = points.iter().map(|q| manhattan(&p, q)).sum();
                if s < self.2 {
                    num_points += 1;
                }
            }
        }

        Ok(num_points)
    }
}
