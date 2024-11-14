use crate::common;
use anyhow::Result;

pub fn main() -> Result<(i64, i64)> {
    let mut lines = common::read_lines("inputs/15.txt")?;
    let mut last_a = lines
        .next()
        .unwrap()?
        .trim()
        .split_once(" with ")
        .unwrap()
        .1
        .parse::<u64>()?;
    let mut last_b = lines
        .next()
        .unwrap()?
        .trim()
        .split_once(" with ")
        .unwrap()
        .1
        .parse::<u64>()?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut answers_a = Vec::with_capacity(5_000_000);
    let mut answers_b = Vec::with_capacity(5_000_000);
    let mut i = 0;

    while i < 40_000_000 || answers_b.len() < 5_000_000 {
        let a = (last_a * 16807) % 2_147_483_647;
        let b = (last_b * 48271) % 2_147_483_647;

        if i < 40_000_000 && a & 0xFFFF == b & 0xFFFF {
            solution_a += 1;
        }

        if answers_b.len() < 5_000_000 {
            if a % 4 == 0 && answers_a.len() < 5_000_000 {
                answers_a.push(a);
            }

            if b % 8 == 0 {
                answers_b.push(b);
            }
        }

        last_a = a;
        last_b = b;
        i += 1;
    }

    for i in 0..5_000_000 {
        if answers_a[i] & 0xFFFF == answers_b[i] & 0xFFFF {
            solution_b += 1;
        }
    }

    Ok((solution_a, solution_b))
}
