use crate::common;
use anyhow::Result;
use std::cmp::Ordering;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/24.txt")?;

    let mut components: Vec<(i64, i64, bool)> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (from, to) = line
            .split_once('/')
            .expect("Expected a valid bridge component");
        components.push((from.parse::<i64>()?, to.parse::<i64>()?, false));
    }

    components.sort_unstable_by_key(|(from, to, _)| (*from).min(*to));

    let solution_a = strongest_bridge(&mut components, 0);
    let solution_b = longest_bridge(&mut components, 0).0;

    Ok((solution_a, solution_b))
}

fn strongest_bridge(components: &mut Vec<(i64, i64, bool)>, required: i64) -> i64 {
    let mut max = 0;
    for j in 0..components.partition_point(|&(l, r, _)| l.min(r) <= required) {
        match components[j] {
            (_, _, true) => continue,
            (l, r, _) if l == required => {
                components[j].2 = true;
                max = max.max(required + r + strongest_bridge(components, r));
                components[j].2 = false;
            }
            (l, r, _) if r == required => {
                components[j].2 = true;
                max = max.max(required + l + strongest_bridge(components, l));
                components[j].2 = false;
            }
            _ => {}
        }
    }
    max
}

fn longest_bridge(components: &mut Vec<(i64, i64, bool)>, required: i64) -> (i64, usize) {
    let mut max = (0, 0);
    for j in 0..components.partition_point(|&(l, r, _)| l.min(r) <= required) {
        match components[j] {
            (_, _, true) => continue,
            (l, r, _) if l == required => {
                components[j].2 = true;
                let (strength, length) = longest_bridge(components, r);
                let strength = l + r + strength;
                let length = length + 1;
                match length.cmp(&max.1) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        max.0 = max.0.max(strength);
                    }
                    Ordering::Greater => max = (strength, length),
                }
                components[j].2 = false;
            }
            (l, r, _) if r == required => {
                components[j].2 = true;
                let (strength, length) = longest_bridge(components, l);
                let strength = l + r + strength;
                let length = length + 1;
                match length.cmp(&max.1) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        max.0 = max.0.max(strength);
                    }
                    Ordering::Greater => max = (strength, length),
                }
                components[j].2 = false;
            }
            _ => {}
        }
    }
    max
}
