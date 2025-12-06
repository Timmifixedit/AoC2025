use crate::day::AoCError::{DayError, DayNotImplemented};
use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;
use std::num::NonZero;
use thiserror::Error;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;

const INPUTS: &[&'static str] = &[
    include_str!("../inputs/day1.txt"),
    include_str!("../inputs/day2.txt"),
    include_str!("../inputs/day3.txt"),
    include_str!("../inputs/day4.txt"),
    include_str!("../inputs/day5.txt"),
    include_str!("../inputs/day6.txt"),
];
const TEST_INPUTS: &[&'static str] = &[
    include_str!("../tests/day1.txt"),
    include_str!("../tests/day2.txt"),
    include_str!("../tests/day3.txt"),
    include_str!("../tests/day4.txt"),
    include_str!("../tests/day5.txt"),
    include_str!("../tests/day6.txt"),
];

const fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

const N_DAYS: usize = min(INPUTS.len(), TEST_INPUTS.len());

#[derive(Error, Debug)]
pub enum AoCError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid day {0}. Must be in [1, {N_DAYS}]")]
    DayError(u8),
    #[error("Day {0} not implemented yet")]
    DayNotImplemented(u8),
    #[error("Logic error: {0}")]
    LogicError(String)
}

pub type Int = u64;
pub type Res = Result<Int, AoCError>;

pub trait Day {
    fn part_1(&self) -> Res;
    fn part_2(&self) -> Res;
}

pub fn instantiate(day: NonZero<u8>, test_instance: bool) -> Result<Box<dyn Day>, AoCError> {
    let day = day.get();
    let day_idx = (day - 1) as usize;
    if day_idx >= INPUTS.len() {
        return Err(DayError(day));
    }

    let data = if test_instance {
        TEST_INPUTS[day_idx]
    } else {
        INPUTS[day_idx]
    };

    match day {
        1 => Ok(Box::new(Day1::new(data)?)),
        2 => Ok(Box::new(Day2::new(data)?)),
        3 => Ok(Box::new(Day3::new(data)?)),
        4 => Ok(Box::new(Day4::new(data)?)),
        5 => Ok(Box::new(Day5::new(data)?)),
        6 => Ok(Box::new(Day6::new(data)?)),
        _ => Err(DayNotImplemented(day)),
    }
}
