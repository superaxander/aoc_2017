use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/1.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let line = lines
        .into_iter()
        .next()
        .unwrap()?
        .chars()
        .collect::<Vec<char>>();

    for (i, c) in line.iter().enumerate() {
        if *c == line[(i + 1) % line.len()] {
            solution_a += *c as i64 - '0' as i64;
        }
        if *c == line[(i + (line.len() / 2)) % line.len()] {
            solution_b += *c as i64 - '0' as i64;
        }
    }
    // for line in lines {
    //     if let Ok(num) = line?.trim().parse::<i64>() {

    //     } else {
    //     }
    // }

    Ok((solution_a, solution_b))
}
