use anyhow::Result;
use std::collections::HashMap;
use std::fs::read_to_string;

fn dance(m: &str, positions: &mut [char; 16]) -> Result<()> {
    match m.chars().next().unwrap() {
        // Spin
        's' => positions.rotate_right(m[1..].parse::<usize>()?),
        // Exchange
        'x' => {
            let (a, b) = &m[1..].split_once('/').unwrap();
            let (a, b) = (a.parse::<usize>()?, b.parse::<usize>()?);
            positions.swap(a, b);
        }
        // Partner
        'p' => {
            let mut chars = m.chars();
            let a = chars.nth(1).unwrap();
            let b = chars.nth(1).unwrap();
            let a = positions.iter().position(|c| *c == a).unwrap();
            let b = positions.iter().position(|c| *c == b).unwrap();
            positions.swap(a, b);
        }
        c => panic!("Unknown move {c}"),
    }

    Ok(())
}

pub fn main() -> Result<(String, String)> {
    let text = read_to_string("inputs/16.txt")?;
    let trimmed = text.trim();
    let moves = trimmed.split(',').collect::<Vec<&str>>();

    let mut positions = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    let mut visited: HashMap<[char; 16], usize> = HashMap::new();
    let mut solution_a = None;

    let mut skipped = false;
    let mut i = 0;
    while i < 1_000_000_000 {
        for m in &moves {
            dance(m, &mut positions)?;
        }
        if i == 0 {
            solution_a = Some(positions.iter().collect());
        }
        if !skipped && let Some(old) = visited.insert(positions, i) {
            let period = i - old;
            let remaining = (1_000_000_000 - i - 1) % period;
            i = 1_000_000_000 - remaining;
            skipped = true;
            continue;
        }
        i += 1;
    }

    Ok((solution_a.unwrap(), positions.into_iter().collect()))
}
