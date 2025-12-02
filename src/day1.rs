use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day};

#[derive(Debug)]
struct Rotation {
    increasing: bool,
    steps: u32,
}

pub struct Day1 {
    rotations: Vec<Rotation>,
}

fn wrap_add(n: u8, incr: u32) -> u8 {
    ((n as u32 + incr) % 100) as u8
}

fn wrap_sub(n: u8, decr: u32) -> u8 {
    let decr = (decr % 100) as u8;
    if decr > n { 100 + n - decr } else { n - decr }
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
                .parse::<u32>()
                .map_err(|e| parse_error(e.to_string()))?;
            rotations.push(Rotation { increasing, steps })
        }

        Ok(Self { rotations })
    }
}

impl Day for Day1 {
    fn part_1(&self) -> Result<u32, AoCError> {
        let mut n_zeros = 0u32;
        let mut state = 50u8;
        for r in &self.rotations {
            state = if r.increasing {
                wrap_add(state, r.steps)
            } else {
                wrap_sub(state, r.steps)
            };

            n_zeros += (state == 0) as u32;
        }
        Ok(n_zeros)
    }

    fn part_2(&self) -> Result<u32, AoCError> {
        let mut n_zeros = 0u32;
        let mut state = 50u8;
        for r in &self.rotations {
            n_zeros += r.steps / 100;
            state = if r.increasing {
                n_zeros += (state as u32 + r.steps % 100 >= 100) as u32;
                wrap_add(state, r.steps)
            } else {
                n_zeros += (state != 0 && r.steps % 100 >= state as u32) as u32;
                wrap_sub(state, r.steps)
            };

        }
        Ok(n_zeros)
    }
}
