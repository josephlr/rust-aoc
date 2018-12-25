#![feature(
    nll,
    never_type,
    specialization,
    map_entry_replace,
    uniform_paths,
    drain_filter,
    try_from,
    try_trait
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::expl_impl_clone_on_copy,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

// mod y2016;
// mod y2017;
mod util;
mod y2018;

use std::{env, io, process};

use util::{Answer, Error};

macro_rules! show_usage {
    ($a:expr, $($arg:tt)+) => ({
        eprintln!($($arg)+);
        eprintln!();
        eprintln!("USAGE:\n\t{} <year> <question> <part>", $a.bin_name);
        process::exit(2)
    });
}

struct CommandLine {
    bin_name: String,
    arg_iter: env::Args,
}

impl CommandLine {
    fn new() -> Self {
        let mut arg_iter = env::args();
        let bin_name = arg_iter.next().expect("Malformed Argv");
        Self { bin_name, arg_iter }
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

fn main() {
    let mut cli = CommandLine::new();
    let year = cli.parse_next("year");
    let day = cli.parse_next("question");
    let part = cli.parse_next("part");

    println!("AoC {} - Day {} - Part {}:", year, day, part);

    let s = io::stdin();
    let r = s.lock();
    let result = match (year, day, part) {
        (2018, 1, 1) => y2018::day1::Sum.run(r),
        (2018, 1, 2) => y2018::day1::FirstRepeat.run(r),
        (2018, 2, 1) => y2018::day2::Checksum([2, 3]).run(r),
        (2018, 2, 2) => y2018::day2::ExactDiff(1).run(r),
        (2018, 3, 1) => y2018::day3::TotalOverlapping(1000).run(r),
        (2018, 3, 2) => y2018::day3::NonOverlapping.run(r),
        (2018, 4, 1) => y2018::day4::SleepyMinute.run(r),
        (2018, 4, 2) => y2018::day4::SleepyGuard.run(r),
        (2018, 5, 1) => y2018::day5::FullReact.run(r),
        (2018, 5, 2) => y2018::day5::BestReact.run(r),
        (2018, 6, 1) => y2018::day6::LargestFinite(-100, 400).run(r),
        (2018, 6, 2) => y2018::day6::Close(-100, 400, 10_000).run(r),
        (2018, 7, 1) => y2018::day7::Order.run(r),
        (2018, 7, 2) => y2018::day7::Workers(5).run(r),
        (2018, 8, 1) => y2018::day8::Sum.run(r),
        (2018, 8, 2) => y2018::day8::Value.run(r),
        (2018, 9, _) => y2018::day9::WinningScore.run(r),
        (2018, 10, _) => y2018::day10::Sky(60_000).run(r),
        (2018, 11, 1) => y2018::day11::LargestPower(300, 3).run(r),
        (2018, 11, 2) => y2018::day11::Overall(300).run(r),
        (2018, 12, 1) => y2018::day12::PlantSum(20).run(r),
        (2018, 13, 1) => y2018::day13::FirstCrash.run(r),
        (2018, 13, 2) => y2018::day13::LastCart.run(r),
        (2018, 14, 1) => y2018::day14::ScoreList(10).run(r),
        (2018, 14, 2) => y2018::day14::FirstOccurance.run(r),
        _ => Err(Error::Custom("Problem not implemented")),
    };

    match result {
        Ok(o) => println!("\t{}", o),
        Err(e) => {
            println!("\tError: {:?}", e);
            process::exit(1);
        }
    };
}
