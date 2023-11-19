use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let mut lines = common::read_lines("inputs/11.txt")?;

    let line = lines.next().unwrap()?;
    let line = line.trim();

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;
    let mut solution_b: i64 = 0;
    for step in line.split(',') {
        match step {
            "ne" => {
                x += 1;
                y -= 1;
            }
            "n" => {
                y -= 1;
                z += 1;
            }
            "nw" => {
                x -= 1;
                z += 1;
            }
            "se" => {
                x += 1;
                z -= 1;
            }
            "s" => {
                y += 1;
                z -= 1;
            }
            "sw" => {
                x -= 1;
                y += 1;
            }
            s => panic!("Unknown direction {s}"),
        }
        solution_b = solution_b.max((x.abs() + y.abs() + z.abs()) / 2);
    }
    let solution_a: i64 = (x.abs() + y.abs() + z.abs()) / 2;

    Ok((solution_a, solution_b))
}
