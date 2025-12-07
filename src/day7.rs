use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day, Int, Res};
use nalgebra::DMatrix;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct Manifold(DMatrix<char>);

impl Display for Manifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Manifold {
    fn new(input: &str) -> Result<Self, AoCError> {
        let lines = input.lines().collect::<Vec<_>>();
        let num_cols = lines
            .first()
            .ok_or(ParseError("no lines".to_string()))?
            .len();
        Ok(Self {
            0: DMatrix::from_row_iterator(
                lines.len(),
                num_cols,
                lines.iter().flat_map(|&l| l.chars()),
            ),
        })
    }

    fn step(&mut self, height: usize) -> Int {
        if height >= self.0.nrows() - 1 {
            return 0;
        }

        let beams = self
            .0
            .row(height)
            .iter()
            .enumerate()
            .filter_map(|(idx, &c)| {
                if c == '|' || c == 'S' {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let r = height;
        let mut num_splits = 0;
        for col in beams {
            if self.0[(r + 1, col)] == '.' {
                self.0[(r + 1, col)] = '|';
            } else if self.0[(r + 1, col)] == '^' {
                self.0[(r + 1, col - 1)] = '|';
                self.0[(r + 1, col + 1)] = '|';
                num_splits += 1;
            }
        }

        num_splits
    }
}

pub struct Day7 {
    manifold: Manifold,
}

impl Day7 {
    pub fn new(input: &str) -> Result<Self, AoCError> {
        Ok(Self {
            manifold: Manifold::new(input)?,
        })
    }
}

impl Day for Day7 {
    fn part_1(&self) -> Res {
        let mut manifold = self.manifold.clone();
        let mut num_splits_total = 0;
        for height in 0..manifold.0.nrows() - 1 {
            num_splits_total += manifold.step(height);
        }

        Ok(num_splits_total)
    }

    fn part_2(&self) -> Res {
        todo!()
    }
}
