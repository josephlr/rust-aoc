use std::fmt;
use std::io;
use std::ops;
use std::str;

pub trait Parseable {
    fn parse_or_else<P, F>(&self, op: F) -> P
    where
        P: str::FromStr,
        F: FnOnce(P::Err) -> P;
}

impl Parseable for str {
    fn parse_or_else<P, F>(&self, op: F) -> P
    where
        P: str::FromStr,
        F: FnOnce(P::Err) -> P,
    {
        self.parse().unwrap_or_else(op)
    }
}

pub fn force_parse<P>(input: impl ops::Deref<Target = str>) -> P
where
    P: str::FromStr,
    P::Err: fmt::Display,
{
    input.parse_or_else(|e| panic!("Could not parse {:?}: {}", input.deref(), e))
}

pub fn force_string(mut r: impl io::BufRead) -> String {
    let mut s = String::new();
    r.read_to_string(&mut s).expect("Reading to String");
    s
}

pub fn force_lines(r: impl io::BufRead) -> impl Iterator<Item = String> {
    r.lines().map(|line| line.expect("Forcing into lines"))
}

pub trait IterUtil: Iterator + Sized {
    fn force_single(mut self) -> Self::Item {
        match self.next() {
            Some(item) => match self.next() {
                Some(_) => panic!("Single value expected, got multiple"),
                None => item,
            },
            None => panic!("Single value expected, got none"),
        }
    }
    fn count_which<P: FnMut(Self::Item) -> bool>(self, mut predicate: P) -> usize {
        self.fold(0, |count, item| match predicate(item) {
            true => count + 1,
            false => count,
        })
    }
}

impl<T: Iterator> IterUtil for T {}

pub trait Ans<Phantom = ()> {
    type Value: fmt::Display;
    fn compute(&self, r: impl io::BufRead) -> Self::Value;
}
