use crate::day::AoCError::ParseError;
use crate::day::{AoCError, Day, Int, Res};
use regex::Regex;

#[derive(Debug)]
struct IdRange {
    start: Int,
    end: Int,
}

pub struct Day2 {
    ids: Vec<IdRange>,
}

fn is_doubled(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }

    let pivot = id.len() / 2;
    let left = &id[..pivot];
    let right = &id[pivot..];
    left == right
}

fn repeats(id: &str) -> bool {
    let len = id.len();
    let mut divisor = 1;
    while divisor <= len / 2 {
        if len % divisor == 0 {
            let mut ok = true;
            let seq = &id[..divisor];
            for pos in 1..len / divisor {
                if seq != &id[pos * divisor..(pos + 1) * divisor] {
                    ok = false;
                    break;
                }
            }

            if ok {
                return true;
            }
        }

        divisor += 1;
    }

    false
}

impl Day2 {
    pub fn new(input: &str) -> Result<Self, AoCError> {
        let re = Regex::new(r"((?<s>\d+)-(?<e>\d+))").unwrap();
        let ranges: Vec<&str> = input.split(',').collect();
        let mut ids = Vec::with_capacity(ranges.len());
        for (idx, idr) in (1..).zip(ranges) {
            let parse_error = |msg| ParseError(format!("range {idx} invalid: {msg}"));
            let range = re
                .captures(idr)
                .ok_or(parse_error("no regex match".to_string()))?;
            let start = range["s"]
                .parse::<Int>()
                .map_err(|e| parse_error(e.to_string()))?;
            let end = range["e"]
                .parse::<Int>()
                .map_err(|e| parse_error(e.to_string()))?;
            ids.push(IdRange { start, end });
        }

        Ok(Self { ids })
    }

    fn calc_sum_invalid(&self, predicate: fn(&str) -> bool) -> Res {
        let mut sum_invalid = 0 as Int;
        for id_range in &self.ids {
            for id in id_range.start..id_range.end + 1 {
                if predicate(&id.to_string()) {
                    sum_invalid += id as Int;
                }
            }
        }

        Ok(sum_invalid)
    }
}

impl Day for Day2 {
    fn part_1(&self) -> Res {
        self.calc_sum_invalid(is_doubled)
    }

    fn part_2(&self) -> Res {
        self.calc_sum_invalid(repeats)
    }
}
