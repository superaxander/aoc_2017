use anyhow::Result;

use crate::common;

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/5.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut jumps = lines
        .filter_map(|line| line.ok()?.trim().parse::<i64>().ok())
        .collect::<Vec<i64>>();
    {
        let mut jumps = jumps.clone();
        let mut index = 0;
        loop {
            solution_a += 1;
            let dir = jumps[index as usize];
            if index + dir < 0 || (index + dir) as usize >= jumps.len() {
                break;
            }
            jumps[index as usize] += 1;
            index += dir;
        }
    }

    let mut index = 0;
    loop {
        solution_b += 1;
        let dir = jumps[index as usize];
        if index + dir < 0 || (index + dir) as usize >= jumps.len() {
            break;
        }
        if dir >= 3 {
            jumps[index as usize] -= 1;
        } else {
            jumps[index as usize] += 1;
        }
        index += dir;
    }

    Ok((solution_a, solution_b))
}
