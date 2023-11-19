use anyhow::Result;
use std::collections::HashMap;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/8.txt")?;

    let mut solution_b = i64::MIN;

    let mut registers = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        let parts = common::RE_WS.split(line).collect::<Vec<&str>>();

        let set_reg = parts[0].to_owned();
        let command = parts[1];
        let amount = parts[2].parse::<i64>()?;
        let check_reg = parts[4].to_owned();
        let operator = parts[5];
        let check_amount = parts[6].parse::<i64>()?;

        if match operator {
            ">" => *registers.entry(check_reg).or_insert(0) > check_amount,
            "<" => *registers.entry(check_reg).or_insert(0) < check_amount,
            ">=" => *registers.entry(check_reg).or_insert(0) >= check_amount,
            "<=" => *registers.entry(check_reg).or_insert(0) <= check_amount,
            "==" => *registers.entry(check_reg).or_insert(0) == check_amount,
            "!=" => *registers.entry(check_reg).or_insert(0) != check_amount,
            o => panic!("Unknown operator {o}"),
        } {
            match command {
                "inc" => *registers.entry(set_reg).or_insert(0) += amount,
                "dec" => *registers.entry(set_reg).or_insert(0) -= amount,
                c => panic!("Unknown command {c}"),
            }
        }
        solution_b = solution_b.max(*registers.values().max().unwrap());
    }

    let solution_a = *registers.values().max().unwrap();

    Ok((solution_a, solution_b))
}
