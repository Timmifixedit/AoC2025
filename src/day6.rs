use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day, Int, Res};

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
        Ok(Self { problems })
    }
}

impl Day for Day6 {
    fn part_1(&self) -> Res {
        Ok(self
            .problems
            .iter()
            .map(|p| {
                p.numbers
                    .iter()
                    .skip(1)
                    .fold(*p.numbers.first().unwrap(), |acc, &val| {
                        p.op.apply(acc, val)
                    })
            })
            .sum::<Int>())
    }

    fn part_2(&self) -> Res {
        todo!()
    }
}
