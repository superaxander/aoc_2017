use anyhow::Result;

use crate::common;

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::explicit_counter_loop)]
pub fn main() -> Result<(i64, String)> {
    const LIST_SIZE: i64 = 256;
    let mut lines = common::read_lines("inputs/10.txt")?;

    let line = lines.next().unwrap()?;
    let line = line.trim();

    let solution_a: i64;
    {
        let lengths = line
            .split(',')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<i64>>();
        let mut list = (0..LIST_SIZE).collect::<Vec<i64>>();
        let mut skip_size = 0;
        let mut index = 0;

        for length in lengths {
            for i in 0..(length / 2) {
                list.swap(
                    ((index + i) % LIST_SIZE) as usize,
                    ((index + length - i - 1) % LIST_SIZE) as usize,
                );
            }
            index += length + skip_size;
            skip_size += 1;
        }

        solution_a = list[0] * list[1];
    }

    let mut solution_b: String;
    {
        let mut lengths = line.chars().map(|c| c as i64).collect::<Vec<i64>>();
        lengths.extend([17, 31, 73, 47, 23]);

        let mut list = (0..LIST_SIZE).collect::<Vec<i64>>();
        let mut skip_size = 0;
        let mut index = 0;

        for _ in 0..64 {
            for length in &lengths {
                for i in 0..(*length / 2) {
                    list.swap(
                        ((index + i) % LIST_SIZE) as usize,
                        ((index + *length - i - 1) % LIST_SIZE) as usize,
                    );
                }
                index += *length + skip_size;
                skip_size += 1;
            }
        }

        solution_b = String::new();
        for i in 0..16 {
            let mut num = 0;
            for j in 0..16 {
                num ^= list[i * 16 + j];
            }
            solution_b.push_str(&format!("{num:02x}"));
        }
    }

    Ok((solution_a, solution_b))
}
