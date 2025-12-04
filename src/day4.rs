use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day, Int, Res};
use nalgebra::{max, min, DMatrix};

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

fn removable(grid: &DMatrix<char>, r: usize, c: usize) -> bool {
    let start = (max(r as i32 - 1, 0) as usize, max(c as i32 - 1, 0) as usize);
    let size = (
        min(2 + (r - start.0), grid.nrows() - start.0),
        min(2 + (c - start.1), grid.ncols() - start.1),
    );
    let view = grid.view(start, size);
    let num_rolls = view.iter().filter(|&&v| v == '@').count();
    num_rolls < 5
}

impl Day for Day4 {
    fn part_1(&self) -> Res {
        let mut num_movable = 0 as Int;
        matrix_iter(&self.grid, |r, c, &val| {
            if val != '@' {
                return;
            }

            num_movable += removable(&self.grid, r, c) as Int;
        });

        Ok(num_movable)
    }

    fn part_2(&self) -> Res {
        let mut num_removed_total: Int = 0;
        let mut grid = self.grid.clone();
        loop {
            let mut num_removed: Int = 0;
            for r in 0..grid.nrows() {
                for c in 0..grid.ncols() {
                    if grid[(r, c)] == '@' && removable(&grid, r, c) {
                        grid[(r, c)] = 'x';
                        num_removed += 1;
                    }
                }
            }

            num_removed_total += num_removed;
            if num_removed == 0 {
                break;
            }
        }

        Ok(num_removed_total)
    }
}
