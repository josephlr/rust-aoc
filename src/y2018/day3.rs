use std::{ops::Range, str::FromStr};

use nom::*;

use crate::util::{number, overlap, to_result, Answer, ByLine, Error, IterExtra, Result};

#[derive(PartialEq, Eq, Debug)]
pub struct Claim {
    id: usize,
    corner: (usize, usize),
    dims: (usize, usize),
}

impl Claim {
    fn x(&self) -> Range<usize> {
        self.corner.0..(self.corner.0 + self.dims.0)
    }
    fn y(&self) -> Range<usize> {
        self.corner.1..(self.corner.1 + self.dims.1)
    }
}

named!(claim<&str,Claim>, do_parse!(
    tag!("#")     >>
    id: number    >>
    tag!(" @ ")   >>
    corner: separated_pair!(number, tag!(","), number) >>
    tag!(": ")    >>
    dims:  separated_pair!(number, tag!("x"), number) >>
    (Claim{id, corner, dims})
));

impl FromStr for Claim {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        to_result(claim(s))
    }
}

pub struct TotalOverlapping(pub usize);
impl Answer for TotalOverlapping {
    type Input = ByLine<Claim>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Claim>) -> Result<Self::Output> {
        let mut v = vec![0; self.0 * self.0];

        for input in inputs {
            for x in input.x() {
                for y in input.y() {
                    v[x * self.0 + y] += 1;
                }
            }
        }

        Ok(v.into_iter().filter(|&x| x > 1).count())
    }
}

fn claims_overlap(c1: &Claim, c2: &Claim) -> bool {
    overlap(c1.x(), c2.x()).is_some() && overlap(c1.y(), c2.y()).is_some()
}

pub struct NonOverlapping;
impl Answer for NonOverlapping {
    type Input = ByLine<Claim>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Claim>) -> Result<Self::Output> {
        let v: Vec<_> = inputs.collect();
        v.iter()
            .filter_map(|c1| {
                if v.iter()
                    .all(|c2| (c1.id == c2.id) || !claims_overlap(c1, c2))
                {
                    Some(c1.id)
                } else {
                    None
                }
            })
            .single()
    }
}
