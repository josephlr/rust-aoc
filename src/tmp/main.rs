#![feature(nll)]

extern crate itertools;
#[macro_use]
extern crate nom;

mod util;
mod matching_digits;
mod checksum;
mod spiral_memory;
mod passwords;
mod cpu_jumps;
mod reallocation;
mod circus;
mod registers;
mod garbage_stream;
mod boxid;

use std::env::{args, Args};
use std::io::stdin;
use std::process::exit;
use util::Ans;

macro_rules! show_usage {
    ($a:expr, $($arg:tt)+) => ({
        eprintln!($($arg)+);
        eprintln!();
        eprintln!("USAGE:\n\t{} <year> <question> <part>", $a.bin_name);
        exit(2)
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

    fn parse_next(&mut self, arg_name: &str) -> i32 {
        if let Some(arg) = self.arg_iter.next() {
            arg.parse()
                .unwrap_or_else(|e| show_usage!(&self, "argument <{}>: {:?}", arg_name, e))
        } else {
            show_usage!(&self, "argument <{}> not provided", arg_name)
        }
    }
}

fn run<Phantom>(ans: impl Ans<Phantom>) {
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
        (2017, 3, 1) => run(spiral_memory::CountSteps),
        (2017, 3, 2) => run(spiral_memory::FirstValue),
        (2017, 4, 1) => run(passwords::NoDuplicates),
        (2017, 4, 2) => run(passwords::NoAnagrams),
        (2017, 5, 1) => run(cpu_jumps::Adjust(|x| x + 1)),
        (2017, 5, 2) => run(cpu_jumps::Adjust(|x| if x < 3 { x + 1 } else { x - 1 })),
        (2017, 6, 1) => run(reallocation::RepeatIdx),
        (2017, 6, 2) => run(reallocation::CycleCount),
        (2017, 7, 1) => run(circus::RootName),
        (2017, 7, 2) => run(circus::CorrectWeight),
        (2017, 8, 1) => run(registers::LargestFinal),
        (2017, 8, 2) => run(registers::LargestDuring),
        (2017, 9, 1) => run(garbage_stream::TotalScore),
        (2017, 9, 2) => run(garbage_stream::GarbageLen),
        (2018, 1, 1) => run(checksum::SingleElement),
        (2018, 1, 2) => run(checksum::FirstDuplicate),
        (2018, 2, 1) => run(boxid::TwoOrThree),
        (2018, 2, 2) => run(boxid::Common),
        _ => {
            println!("No solution found for this question.");
            exit(1)
        }
    }
}
