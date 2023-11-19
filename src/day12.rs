use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::common;

pub fn main() -> Result<(usize, i64)> {
    let lines = common::read_lines("inputs/12.txt")?;

    let mut connections = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (program, rest) = line.split_once(" <-> ").unwrap();
        let program = program.parse::<usize>()?;
        let vec: &mut Vec<usize> = connections.entry(program).or_default();
        for connection in rest.split(", ") {
            let connection = connection.parse::<usize>()?;
            vec.push(connection);
        }
    }

    let mut solution_a = 0;
    let mut solution_b = 0;
    let mut visited = HashSet::new();
    while visited.len() < connections.len() {
        let mut frontier = vec![if visited.is_empty() {
            0
        } else {
            *connections
                .keys()
                .find(|it| !visited.contains(*it))
                .unwrap()
        }];

        while let Some(current) = frontier.pop() {
            if !visited.insert(current) {
                continue;
            }
            frontier.extend(&connections[&current]);
        }

        if solution_a == 0 {
            solution_a = visited.len();
        }
        solution_b += 1;
    }

    Ok((solution_a, solution_b))
}
