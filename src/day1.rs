use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day};

#[derive(Debug)]
struct Rotation {
    increasing: bool,
    steps: u16,
}

pub struct Day1 {
    rotations: Vec<Rotation>,
}

impl Day1 {
    pub fn new(input: &str) -> Result<Self, AoCError> {
        let lines: Vec<&str> = input.split_whitespace().collect();
        let mut rotations = Vec::with_capacity(lines.len());
        for (l, line) in (1..).zip(lines.into_iter()) {
            let parse_error = |msg| ParseError(format!("Error at line {l}: {msg}"));
            if line.len() < 2 {
                return Err(parse_error("too few chars".to_string()));
            }

            let increasing = match line.chars().nth(0).unwrap() {
                'R' => true,
                'L' => false,
                _ => return Err(parse_error("invalid rotation".to_string())),
            };

            let steps = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<u16>()
                .map_err(|e| parse_error(e.to_string()))?;
            rotations.push(Rotation { increasing, steps })
        }

        Ok(Self { rotations })
    }
}

impl Day for Day1 {
    fn part_1(&self) -> Result<u32, AoCError> {
        println!("rotations: {:?}", self.rotations);
        Ok(0)
    }

    fn part_2(&self) -> Result<u32, AoCError> {
        todo!()
    }
}
