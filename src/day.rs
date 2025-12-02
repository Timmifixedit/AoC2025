use crate::day::AoCError::{DayError, DayNotImplemented, ParseError};
use std::num::NonZero;
use thiserror::Error;

const INPUTS: &[&'static str] = &[include_str!("../inputs/day1.txt")];

#[derive(Error, Debug)]
pub enum AoCError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid day {0}. Must be in [1, {{INPUTS.len()}}]")]
    DayError(u8),
    #[error("Day {0} not implemented yet")]
    DayNotImplemented(u8),
    #[error("Input error: {0}")]
    InputError(String),
}

pub trait Day {
    fn part_1(&self) -> Result<u32, AoCError>;
    fn part_2(&self) -> Result<u32, AoCError>;
}

pub fn instantiate(day: NonZero<u8>) -> Result<Box<dyn Day>, AoCError> {
    let day = day.get() - 1;
    if day as usize >= INPUTS.len() {
        Err(DayError(day))
    } else {
        match day {
            1 => {
                todo!()
            }
            _ => Err(DayNotImplemented(day)),
        }
    }
}
