use crate::day::AoCError::{LogicError, ParseError};
use crate::day::{AoCError, Day, Res};

pub struct Day3 {
    banks: Vec<Vec<u8>>,
}

fn max<Seq: Iterator<Item: PartialOrd>>(mut seq: Seq) -> Option<(usize, Seq::Item)> {
    let mut max = seq.next()?;
    let mut idx = 0;
    for (i, item) in (1..).zip(seq) {
        if item > max {
            max = item;
            idx = i;
        }
    }
    Some((idx, max))
}

fn joltage_rec(bank: &[u8], mut top: Vec<u8>, num_batteries: usize) -> Vec<u8> {
    if num_batteries == 0 {
        return top;
    }

    let sub_view = &bank[..bank.len() - num_batteries + 1];
    let (idx, largest) = max(sub_view.iter()).unwrap();
    top.push(*largest);
    joltage_rec(&bank[idx + 1..], top, num_batteries - 1)
}

fn joltage(bank: &Vec<u8>, num_batteries: usize) -> Result<Res, AoCError> {
    if bank.len() < num_batteries {
        return Err(LogicError(format!(
            "cannot chose {} batteries from bank of size {}",
            bank.len(),
            num_batteries
        )));
    }

    let largest = joltage_rec(&bank, Vec::with_capacity(num_batteries), num_batteries);
    Ok(largest.iter().fold(0, |acc, &x| acc * 10 + x as u64))
}

impl Day3 {
    pub fn new(data: &str) -> Result<Self, AoCError> {
        let lines: Vec<&str> = data.lines().collect();
        let mut banks: Vec<Vec<u8>> = Vec::with_capacity(lines.len());
        for (l, line) in (1..).zip(lines) {
            let parse_error = |msg| ParseError(format!("error in line {l}: {msg}"));
            let bank: Result<Vec<u8>, _> = line
                .chars()
                .enumerate()
                .map(|(idx, c)| {
                    c.to_digit(10)
                        .ok_or(parse_error(format!("position {idx} is not a digit")))
                        .map(|d| d as u8)
                })
                .collect();
            banks.push(bank?)
        }

        Ok(Self { banks })
    }

    fn sum_joltage(&self, num_batteries: usize) -> Result<Res, AoCError> {
        let mut joltage_sum = 0;
        for bank in &self.banks {
            let j = joltage(bank, num_batteries)?;
            println!("{:?} -> {}", bank, j);
            joltage_sum += j;
        }

        Ok(joltage_sum)
    }
}

impl Day for Day3 {
    fn part_1(&self) -> Result<Res, AoCError> {
        self.sum_joltage(2)
    }

    fn part_2(&self) -> Result<Res, AoCError> {
        self.sum_joltage(12)
    }
}
