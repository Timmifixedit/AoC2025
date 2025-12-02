mod day;

use crate::day::AoCError;
use crate::day::AoCError::{DayError, InputError};
use std::env;
use std::num::NonZero;
use std::process::exit;

fn run() -> Result<(), AoCError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify the day number");
        exit(1);
    }

    let day = NonZero::new(
        args[1]
            .parse::<u8>()
            .expect("Day must be a positive integer"),
    )
    .ok_or(DayError(0))?;
    let day = day::instantiate(day)?;
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
