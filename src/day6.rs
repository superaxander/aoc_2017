use anyhow::Result;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let mut lines = common::read_lines("inputs/6.txt")?;

    let mut solution_a = 0;
    let solution_b: i64;

    let mut seen = HashMap::new();
    let mut current = common::RE_WS
        .split(lines.next().unwrap()?.trim())
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    loop {
        match seen.entry(current.clone()) {
            Entry::Occupied(o) => {
                solution_b = solution_a - o.get();
                break;
            }
            Entry::Vacant(v) => {
                v.insert(solution_a);
            }
        }

        let (mut idx, mut remaining) = current
            .iter()
            .copied()
            .enumerate()
            .max_by(|(i, a), (j, b)| match a.cmp(b) {
                a @ (Ordering::Less | Ordering::Greater) => a,
                Ordering::Equal => j.cmp(i),
            })
            .unwrap();
        current[idx] = 0;
        idx = (idx + 1) % current.len();
        while remaining > 0 {
            remaining -= 1;
            current[idx] += 1;
            idx = (idx + 1) % current.len();
        }

        solution_a += 1;
    }

    Ok((solution_a, solution_b))
}
