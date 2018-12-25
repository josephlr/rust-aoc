use std::{
    collections::{
        hash_map::{Entry::*, HashMap},
    },  
    hash::Hash,
};

use nom::*;

use crate::util::{to_result, Answer, Parse, Result};

pub struct DaThing<J> {
    first: J,
    next: J,
}

named!(da_thing<&str,DaThing<String>>, do_parse!(
    tag!("Step ") >> first: map!(alphanumeric, String::from) >>
    tag!(" must be finished before step ") >>
    next: map!(alphanumeric, String::from) >> tag!(" can begin.") >>
    (DaThing{first, next})
));

impl Parse for DaThing<String> {
    fn parse(s: &str) -> Result<Self> {
        to_result(da_thing(s))
    }
}

pub struct Day1;
impl Answer for Day1 {
    type Input = DaThing<String>;
    type Output = String;
    fn ans(&self, inputs: impl Iterator<Item = Self::Input>) -> Result<Self::Output> {
        unimplemented!()
    }
}

pub struct Day2;
impl Answer for Day2 {
    type Input = DaThing<String>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Self::Input>) -> Result<Self::Output> {
        unimplemented!()
    }
}
