use crate::day::{AoCError, Day, Int, Res};

#[derive(Debug)]
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
        todo!()
    }
}
