use anyhow::Result;

use crate::common;
use std::collections::HashSet;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/4.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let words = common::RE_WS
            .split(line?.trim())
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>();
        if words.iter().collect::<HashSet<&String>>().len() != words.len() {
            continue;
        }
        solution_a += 1;

        let anagrams = words
            .iter()
            .map(|w| {
                let mut c = w.chars().collect::<Vec<char>>();
                c.sort_unstable();
                c
            })
            .collect::<HashSet<Vec<char>>>();

        if anagrams.len() == words.len() {
            solution_b += 1;
        }
    }

    Ok((solution_a, solution_b))
}
