use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day, Int, Res};
use nalgebra::DMatrix;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct Manifold {
    field: DMatrix<char>,
    num_visits: DMatrix<Int>,
}

impl Display for Manifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.field, self.num_visits)
    }
}

impl Manifold {
    fn new(input: &str) -> Result<Self, AoCError> {
        let lines = input.lines().collect::<Vec<_>>();
        let num_cols = lines
            .first()
            .ok_or(ParseError("no lines".to_string()))?
            .len();
        let field = DMatrix::from_row_iterator(
            lines.len(),
            num_cols,
            lines.iter().flat_map(|&l| l.chars()),
        );
        let mut num_visits = DMatrix::zeros(field.nrows(), field.ncols());
        let start_pos = field
            .row(0)
            .iter()
            .position(|&c| c == 'S')
            .ok_or(ParseError("No start".to_string()))?;
        num_visits[(0, start_pos)] = 1;
        Ok(Self { field, num_visits })
    }

    fn step(&mut self, height: usize) -> Int {
        if height >= self.field.nrows() - 1 {
            return 0;
        }

        let beams = self
            .field
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
            let current_n_visits = self.num_visits[(r, col)];
            if self.field[(r + 1, col)] != '^' {
                self.field[(r + 1, col)] = '|';
                self.num_visits[(r + 1, col)] += current_n_visits;
            } else {
                self.field[(r + 1, col - 1)] = '|';
                self.field[(r + 1, col + 1)] = '|';
                self.num_visits[(r + 1, col - 1)] += current_n_visits;
                self.num_visits[(r + 1, col + 1)] += current_n_visits;
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
        for height in 0..manifold.field.nrows() - 1 {
            num_splits_total += manifold.step(height);
        }

        Ok(num_splits_total)
    }

    fn part_2(&self) -> Res {
        let mut manifold = self.manifold.clone();
        for height in 0..manifold.field.nrows() - 1 {
            manifold.step(height);
        }

        Ok(manifold.num_visits.row(manifold.field.nrows() - 1).iter().sum::<Int>())
    }
}
