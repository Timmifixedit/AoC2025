use crate::day::AoCError::{LogicError, ParseError};
use crate::day::{AoCError, Day, Int, Res};
use nalgebra::{DMatrix, max, min};

pub struct Day4 {
    grid: DMatrix<char>,
}

impl Day4 {
    pub fn new(data: &str) -> Result<Self, AoCError> {
        let grid = data.lines().collect::<Vec<_>>();
        if grid.is_empty() {
            return Err(ParseError("No data".to_string()));
        }

        let n_rows = data.lines().count();
        let n_cols = data.lines().next().unwrap().len();
        let grid = DMatrix::from_row_iterator(n_rows, n_cols, data.chars().filter(|c| *c != '\n'));
        Ok(Self { grid })
    }
}

fn matrix_iter(matrix: &DMatrix<char>, mut f: impl FnMut(usize, usize, &char)) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            f(r, c, &matrix[(r, c)]);
        }
    }
}

impl Day for Day4 {
    fn part_1(&self) -> Res {
        let mut num_movable = 0 as Int;
        matrix_iter(&self.grid, |r, c, &val| {
            if val != '@' {
                return;
            }

            let start = (max(r as i32 - 1, 0) as usize, max(c as i32 - 1, 0) as usize);
            let size = (
                min(2 + (r - start.0), self.grid.nrows() - start.0),
                min(2 + (c - start.1), self.grid.ncols() - start.1),
            );
            let view = self.grid.view(start, size);
            let num_rolls = view.iter().filter(|&&v| v == '@').count();
            num_movable += (num_rolls  < 5) as Int;
        });

        Ok(num_movable)
    }

    fn part_2(&self) -> Res {
        todo!()
    }
}
