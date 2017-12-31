#![feature(conservative_impl_trait, universal_impl_trait)]
#![feature(nll)]
#![allow(non_camel_case_types)] // rust-lang/rust#46959

extern crate itertools;
extern crate num;

mod util;
mod matching_digits;
mod checksum;

use std::env::{args, Args};
use std::io::stdin;
use std::process::exit;
use util::{Ans, Parseable};

macro_rules! show_usage {
    ($a:expr) => ({
        eprintln!("USAGE:\n\t{} <year> <question> <part>", $a.bin_name);
        exit(2)
    });
    ($a:expr, $($arg:tt)+) => ({
        eprintln!($($arg)+);
        eprintln!();
        show_usage!($a)
    });
}

struct CommandLine {
    bin_name: String,
    arg_iter: Args,
}

impl CommandLine {
    fn new() -> Self {
        let mut arg_iter = args();
        let bin_name = arg_iter.next().expect("Malformed Argv");
        CommandLine { bin_name, arg_iter }
    }

    fn parse_next<P: Parseable>(&mut self, arg_name: &str) -> P {
        if let Some(arg) = self.arg_iter.next() {
            P::parse(&arg).unwrap_or_else(|e| show_usage!(&self, "argument <{}>: {}", arg_name, e))
        } else {
            show_usage!(&self, "argument <{}> not provided", arg_name)
        }
    }
}

fn run(ans: impl Ans) {
    let s = stdin();
    println!("{}", ans.compute(s.lock()));
}

fn main() {
    let mut cli = CommandLine::new();
    let year = cli.parse_next("year");
    let day = cli.parse_next("question");
    let part = cli.parse_next("part");
    print!("AoC {} - Day {} - Part {}:\n\n", year, day, part);

    match (year, day, part) {
        (2017, 1, 1) => run(matching_digits::Sequential),
        (2017, 1, 2) => run(matching_digits::Halfway),
        (2017, 2, 1) => run(checksum::MinMaxDiff),
        (2017, 2, 2) => run(checksum::EvenDiv),
        _ => {
            println!("No solution found for this question.");
            exit(1)
        }
    }
}
