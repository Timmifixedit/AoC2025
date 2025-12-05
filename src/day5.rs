use crate::day::{AoCError, Day, Int, Res};
use std::cmp::max;

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(data: &str) -> Result<Self, AoCError> {
        let parts = data.split('-').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(AoCError::ParseError(format!("invalid range: {}", data)));
        }

        let start = parts
            .first()
            .unwrap()
            .parse::<usize>()
            .map_err(|e| AoCError::ParseError(format!("invalid start: {}", e)))?;
        let end = parts
            .last()
            .unwrap()
            .parse::<usize>()
            .map_err(|e| AoCError::ParseError(format!("invalid end: {}", e)))?;
        Ok(Self { start, end })
    }

    fn contains(&self, x: usize) -> bool {
        x >= self.start && x <= self.end
    }

    fn len(&self) -> usize {
        self.end - self.start + 1
    }
}

pub struct Day5 {
    fresh: Vec<Range>,
    ids: Vec<usize>,
}

impl Day5 {
    pub fn new(data: &str) -> Result<Self, AoCError> {
        let mut first_part = true;
        let mut fresh = Vec::new();
        let mut ids = Vec::new();
        for (l, line) in (1..).zip(data.lines()) {
            let parse_error =
                |msg: String| AoCError::ParseError(format!("error in line {l}: {msg}"));
            if line.is_empty() {
                first_part = false;
                continue;
            }

            if first_part {
                fresh.push(Range::new(&line).map_err(|e| parse_error(e.to_string()))?);
            } else {
                ids.push(
                    line.parse::<usize>()
                        .map_err(|e| parse_error(e.to_string()))?,
                );
            }
        }

        Ok(Self { fresh, ids })
    }
}

impl Day for Day5 {
    fn part_1(&self) -> Res {
        let mut res: Int = 0;
        for &id in &self.ids {
            res += self.fresh.iter().any(|r| r.contains(id)) as Int;
        }

        Ok(res)
    }

    fn part_2(&self) -> Res {
        let mut fresh = self.fresh.clone();
        fresh.sort_unstable_by_key(|r| r.start);
        let mut concatenated: Vec<Range> = Vec::new();
        for r in fresh {
            if concatenated.is_empty() || concatenated.last().unwrap().end < r.start {
                concatenated.push(r);
            } else {
                let last = concatenated.last_mut().unwrap();
                last.end = max(last.end, r.end);
            }
        }

        Ok(concatenated.iter().fold(0, |acc, r| acc + r.len()) as Int)
    }
}
