use crate::common::*;
use anyhow::Result;
use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum State {
    Infected,
    Weakened,
    Flagged,
}

impl CharConvertable for State {
    fn to_char(option: Option<&Self>) -> char {
        match option {
            None => '.',
            Some(State::Infected) => '#',
            Some(State::Weakened) => 'W',
            Some(State::Flagged) => 'F',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => None,
            '#' => Some(State::Infected),
            c => panic!("unexpected character `{c}`"),
        }
    }
}

pub fn main() -> Result<(i64, i64)> {
    let lines = read_lines("inputs/22.txt")?;

    let start_grid = lines.process_results(|lines| {
        InfiniteGrid::<SignedCoordinate, State, true, true>::read(lines)
    })?;
    let extents = start_grid.extents();

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut position = (extents.0 + extents.1) / 2;
    let mut facing = Facing::North;
    let mut grid = start_grid.clone();
    for _ in 0..10_000 {
        match grid.get(&position) {
            None => {
                facing = facing.left();
                grid.set(position, Some(State::Infected));
                solution_a += 1;
            }
            Some(State::Infected) => {
                facing = facing.right();
                grid.set(position, None);
            }
            _ => panic!(),
        }
        position = position.forward(facing);
    }

    let mut position = (extents.0 + extents.1) / 2;
    let mut facing = Facing::North;
    let mut grid = start_grid;
    for _ in 0..10_000_000 {
        match grid.get(&position) {
            None => {
                facing = facing.left();
                grid.set(position, Some(State::Weakened));
            }
            Some(State::Weakened) => {
                grid.set(position, Some(State::Infected));
                solution_b += 1;
            }
            Some(State::Infected) => {
                facing = facing.right();
                grid.set(position, Some(State::Flagged));
            }
            Some(State::Flagged) => {
                facing = facing.flip();
                grid.set(position, None);
            }
        }
        position = position.forward(facing);
    }

    Ok((solution_a, solution_b))
}
