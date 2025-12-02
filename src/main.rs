mod day;
mod day1;
mod day2;

use crate::day::AoCError;
use std::num::NonZero;
use std::process::exit;
use clap::Parser;


#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(required = true)]
    day: NonZero<u8>,

    #[arg(short, long, default_value = "false")]
    test: bool
}

fn run() -> Result<(), AoCError> {
    let args = Args::parse();
    let day = day::instantiate(args.day, args.test)?;
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
