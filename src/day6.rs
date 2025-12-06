use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day, Int, Res};
use nalgebra::DMatrix;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn new(op: &str) -> Result<Self, AoCError> {
        match op {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(ParseError(format!("invalid op {op}"))),
        }
    }

    fn apply(&self, a: Int, b: Int) -> Int {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<Int>,
    op: Op,
}

pub struct Day6 {
    problems: Vec<Problem>,
    problems_part_2: Vec<Problem>,
}

fn parse_part_2(input: &str, lines: Vec<&str>) -> Result<Vec<Problem>, AoCError> {
    let mat = DMatrix::from_row_iterator(
        lines.len() - 1,
        lines.first().unwrap().len(),
        input.chars().filter(|c| *c != '\n'),
    );

    let mut numbers: Vec<Vec<Int>> = Vec::new();
    numbers.push(Vec::new());
    for col in mat.column_iter().rev() {
        let number = col
            .iter()
            .filter_map(|&c| c.to_digit(10))
            .fold(0, |acc, n| acc * 10 + n) as Int;
        // Only works because there are no zeros in the input
        if number == 0 {
            numbers.push(Vec::new());
        } else {
            numbers.last_mut().unwrap().push(number);
        }
    }

    let mut problems = Vec::new();
    for (numbers, op_str) in numbers
        .into_iter()
        .zip(lines.last().unwrap().split_whitespace().rev())
    {
        problems.push(Problem {
            numbers,
            op: Op::new(op_str)?,
        })
    }

    Ok(problems)
}

impl Day6 {
    pub fn new(input: &str) -> Result<Self, AoCError> {
        let mut numbers: Vec<Vec<Int>> = Vec::new();
        let lines = input.lines().collect::<Vec<_>>();
        for (l, &line) in (1..).zip(lines.iter().take(lines.len() - 1)) {
            for (p, n) in line.split_whitespace().enumerate() {
                if p >= numbers.len() {
                    numbers.push(Vec::new());
                }

                numbers[p].push(
                    n.parse::<Int>()
                        .map_err(|e| ParseError(format!("line {l}, pos {}: {e}", p + 1)))?,
                );
            }
        }

        let mut problems = Vec::new();
        for (numbers, op_str) in numbers
            .into_iter()
            .zip(lines.last().unwrap().split_whitespace())
        {
            problems.push(Problem {
                numbers,
                op: Op::new(op_str)?,
            });
        }

        Ok(Self {
            problems,
            problems_part_2: parse_part_2(input, lines)?,
        })
    }
}

fn grand_total(problems: &[Problem]) -> Int {
    problems
        .iter()
        .map(|p| {
            p.numbers
                .iter()
                .skip(1)
                .fold(*p.numbers.first().unwrap(), |acc, &val| {
                    p.op.apply(acc, val)
                })
        })
        .sum::<Int>()
}

impl Day for Day6 {
    fn part_1(&self) -> Res {
        Ok(grand_total(&self.problems))
    }

    fn part_2(&self) -> Res {
        Ok(grand_total(&self.problems_part_2))
    }
}
