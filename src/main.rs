mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use crate::day::AoCError;
use clap::Parser;
use std::num::NonZero;
use std::process::exit;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(required = true)]
    day: NonZero<u8>,

    #[arg(short, long, default_value = "false")]
    test: bool,
}

fn run() -> Result<(), AoCError> {
    let args = Args::parse();
    let day = day::instantiate(args.day, args.test)?;
    println!(
        "Launching day {} on {}",
        args.day,
        if args.test {
            "test data"
        } else {
            "the actual data"
        }
    );
    println!("Result part 1: {}", day.part_1()?);
    println!("Result part 2: {}", day.part_2()?);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error {e}");
        exit(1);
    }
}
