use crate::common;
use anyhow::Result;
use std::cmp::Ordering;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/24.txt")?;

    let mut components: Vec<(i64, i64)> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (from, to) = line
            .split_once('/')
            .expect("Expected a valid bridge component");
        components.push((from.parse::<i64>()?, to.parse::<i64>()?));
    }

    components.sort_unstable_by_key(|(from, to)| (*from).min(*to));

    let solution_a = strongest_bridge(&components, &mut Vec::new(), 0);
    let solution_b = longest_bridge(&components, &mut Vec::new(), 0).0;

    Ok((solution_a, solution_b))
}

fn strongest_bridge(
    components: &Vec<(i64, i64)>,
    used: &mut Vec<(i64, i64)>,
    required: i64,
) -> i64 {
    let mut max = 0;
    let i = components.partition_point(|&(l, r)| l.min(r) <= required);
    for component in components[..i].iter() {
        if used.contains(component) {
            continue;
        }
        match component {
            (l, r) if *l == required => {
                used.push(*component);
                max = max.max(required + *r + strongest_bridge(components, used, *r));
                used.pop();
            }
            (l, r) if *r == required => {
                used.push(*component);
                max = max.max(required + *l + strongest_bridge(components, used, *l));
                used.pop();
            }
            _ => {}
        }
    }
    max
}

fn longest_bridge(
    components: &Vec<(i64, i64)>,
    used: &mut Vec<(i64, i64)>,
    required: i64,
) -> (i64, usize) {
    let mut max = (0, 0);
    let i = components.partition_point(|&(l, r)| l.min(r) <= required);
    for component in components[..i].iter() {
        if used.contains(component) {
            continue;
        }
        match component {
            (l, r) if *l == required => {
                used.push(*component);
                let (strength, length) = longest_bridge(components, used, *r);
                let strength = *l + *r + strength;
                let length = length + 1;
                match length.cmp(&max.1) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        max.0 = max.0.max(strength);
                    }
                    Ordering::Greater => max = (strength, length),
                }
                used.pop();
            }
            (l, r) if *r == required => {
                used.push(*component);
                let (strength, length) = longest_bridge(components, used, *l);
                let strength = *l + *r + strength;
                let length = length + 1;
                match length.cmp(&max.1) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        max.0 = max.0.max(strength);
                    }
                    Ordering::Greater => max = (strength, length),
                }
                used.pop();
            }
            _ => {}
        }
    }
    max
}
