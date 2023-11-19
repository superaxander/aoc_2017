use anyhow::Result;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/13.txt")?;

    let mut intervals = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (layer, interval) = line.split_once(": ").unwrap();
        let layer = layer.parse::<usize>()?;
        intervals.resize(layer, 0);
        intervals.push(interval.parse::<usize>()?);
    }

    let mut solution_a = 0;
    let mut solution_b = 1;
    for (i, interval) in intervals.iter().copied().enumerate() {
        if interval == 0 {
            continue;
        }

        if i == 0 || i % ((interval - 1) * 2) == 0 {
            solution_a += i * interval;
        }
    }

    'outer: for j in 0.. {
        for (i, interval) in intervals.iter().copied().enumerate() {
            if interval == 0 {
                continue;
            }

            if (i + j) % ((interval - 1) * 2) == 0 {
                continue 'outer;
            }
        }
        solution_b = j;
        break;
    }

    Ok((solution_a, solution_b))
}
