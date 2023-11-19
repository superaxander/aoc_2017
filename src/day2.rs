use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/2.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let nums = common::RE_WS
            .split(line?.trim())
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<i64>>();
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        solution_a += max - min;
        for (i, n) in nums.iter().enumerate() {
            for m in nums.iter().skip(i + 1) {
                if n % m == 0 {
                    solution_b += n / m;
                } else if m % n == 0 {
                    solution_b += m / n;
                }
            }
        }
    }

    Ok((solution_a, solution_b))
}
