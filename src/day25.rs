use crate::common;
use anyhow::Result;
use std::collections::HashSet;

enum Direction {
    Left,
    Right,
}

struct State {
    false_write_value: bool,
    false_direction: Direction,
    false_next_state: usize,
    true_write_value: bool,
    true_direction: Direction,
    true_next_state: usize,
}

pub fn main() -> Result<(usize, usize)> {
    let mut lines = common::read_lines("inputs/25.txt")?;

    let start = lines
        .next()
        .unwrap()?
        .split_once("state ")
        .unwrap()
        .1
        .chars()
        .next()
        .unwrap() as usize
        - 'A' as usize;
    let checksum_after = lines
        .next()
        .unwrap()?
        .split_once("after ")
        .unwrap()
        .1
        .split_once(" ")
        .unwrap()
        .0
        .parse::<usize>()?;
    let mut states = Vec::new();

    while let Some(empty) = lines.next() {
        debug_assert!(empty?.is_empty());

        let state = lines
            .next()
            .unwrap()?
            .split_once("state ")
            .unwrap()
            .1
            .chars()
            .next()
            .unwrap() as usize
            - 'A' as usize;
        debug_assert_eq!(state, states.len());
        let line = lines.next().unwrap()?;
        debug_assert_eq!(line, "  If the current value is 0:");
        let false_write_value = lines
            .next()
            .unwrap()?
            .split_once("value ")
            .unwrap()
            .1
            .starts_with('1');
        let false_direction = match lines.next().unwrap()?.split_once("the ").unwrap().1 {
            "left." => Direction::Left,
            "right." => Direction::Right,
            direction => panic!("Unexpected direction: {direction}"),
        };
        let false_next_state = lines
            .next()
            .unwrap()?
            .split_once("state ")
            .unwrap()
            .1
            .chars()
            .next()
            .unwrap() as usize
            - 'A' as usize;
        let line = lines.next().unwrap()?;
        debug_assert_eq!(line, "  If the current value is 1:");
        let true_write_value = lines
            .next()
            .unwrap()?
            .split_once("value ")
            .unwrap()
            .1
            .starts_with('1');
        let true_direction = match lines.next().unwrap()?.split_once("the ").unwrap().1 {
            "left." => Direction::Left,
            "right." => Direction::Right,
            direction => panic!("Unexpected direction: {direction}"),
        };
        let true_next_state = lines
            .next()
            .unwrap()?
            .split_once("state ")
            .unwrap()
            .1
            .chars()
            .next()
            .unwrap() as usize
            - 'A' as usize;
        states.push(State {
            false_write_value,
            false_direction,
            false_next_state,
            true_write_value,
            true_direction,
            true_next_state,
        })
    }

    let mut tape = HashSet::new();
    let mut position = 0;
    let mut state = start;

    for _ in 0..checksum_after {
        if tape.contains(&position) {
            if !states[state].true_write_value {
                tape.remove(&position);
            }
            match states[state].true_direction {
                Direction::Left => position -= 1,
                Direction::Right => position += 1,
            }
            state = states[state].true_next_state;
        } else {
            if states[state].false_write_value {
                tape.insert(position);
            }
            match states[state].false_direction {
                Direction::Left => position -= 1,
                Direction::Right => position += 1,
            }
            state = states[state].false_next_state;
        }
    }

    let solution_a = tape.len();

    Ok((solution_a, 0))
}
