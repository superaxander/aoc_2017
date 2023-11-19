use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let mut lines = common::read_lines("inputs/9.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut skip = false;
    let mut depth = 0;
    let mut garbage = false;

    for c in lines.next().unwrap()?.trim().chars() {
        if skip {
            skip = false;
            continue;
        }
        match c {
            '!' if garbage => skip = true,
            '<' if !garbage => garbage = true,
            '>' if garbage => garbage = false,
            '{' if !garbage => {
                depth += 1;
                solution_a += depth;
            }
            '}' if !garbage => depth -= 1,
            _ => {
                if garbage {
                    solution_b += 1;
                }
            }
        }
    }

    Ok((solution_a, solution_b))
}
