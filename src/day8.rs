use crate::day::{AoCError, Day, Int, Res};
use distances::vectors::euclidean;
use nalgebra::Point3;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
struct JunctionBox {
    pos: Point3<Int>,
    id: usize,
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.pos)
    }
}

impl JunctionBox {
    fn new(pos: &[Int], id: usize) -> Result<Self, AoCError> {
        if pos.len() != 3 {
            return Err(AoCError::ParseError(
                "expected exactly 3 coordinates".to_string(),
            ));
        }

        Ok(Self {
            pos: Point3::new(pos[0], pos[1], pos[2]),
            id,
        })
    }

    fn distance(&self, other: &Self) -> f32 {
        euclidean(self.pos.coords.as_slice(), other.pos.coords.as_slice())
    }
}

struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(num_entries: usize) -> Self {
        Self {
            parent: (0..num_entries).collect(),
        }
    }

    fn representative(&self, elem: usize) -> Result<usize, AoCError> {
        if elem >= self.parent.len() {
            return Err(AoCError::LogicError(format!(
                "elem {} is out of bounds (max: {})",
                elem,
                self.parent.len()
            )));
        }

        if self.parent[elem] == elem {
            return Ok(elem);
        }

        self.representative(self.parent[elem])
    }

    fn merge(&mut self, elem1: usize, elem2: usize) -> Result<(), AoCError> {
        let rep1 = self.representative(elem1)?;
        let rep2 = self.representative(elem2)?;
        self.parent[rep1] = rep2;
        Ok(())
    }
}

pub struct Day8 {
    junction_boxes: Vec<JunctionBox>,
}

impl Day8 {
    pub fn new(data: &str) -> Result<Self, AoCError> {
        let mut junction_boxes = Vec::new();
        for (idx, line) in (1..).zip(data.lines()) {
            let err = |msg| AoCError::ParseError(format!("error in line {idx}: {msg}"));
            let parts = line
                .split(',')
                .map(|s| s.parse::<Int>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| err(e.to_string()))?;
            junction_boxes.push(JunctionBox::new(&parts, idx - 1).map_err(|e| err(e.to_string()))?);
        }

        Ok(Self { junction_boxes })
    }
}

impl Day for Day8 {
    fn part_1(&self) -> Res {
        let mut cartesian: Vec<(&JunctionBox, &JunctionBox)> =
            Vec::with_capacity(self.junction_boxes.len() * (self.junction_boxes.len() - 1) / 2);
        for (i, j1) in self.junction_boxes.iter().enumerate() {
            for j2 in self.junction_boxes.iter().skip(i + 1) {
                cartesian.push((j1, j2));
            }
        }

        cartesian.sort_unstable_by(|&p1, &p2| p1.0.distance(p1.1).total_cmp(&p2.0.distance(p2.1)));
        let mut circuits = DisjointSet::new(self.junction_boxes.len());
        for (j1, j2) in cartesian.into_iter().take(1000) {
            circuits.merge(j1.id, j2.id)?;
        }

        let mut sizes: HashMap<usize, Int> = HashMap::new();
        for id in 0..self.junction_boxes.len() {
            *sizes.entry(circuits.representative(id)?).or_default() += 1;
        }

        let mut largest: Vec<_> = sizes.into_values().collect();
        largest.sort_unstable();
        Ok(largest.iter().rev().take(3).product())
    }

    fn part_2(&self) -> Res {
        todo!()
    }
}
