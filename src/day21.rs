use anyhow::Result;

use crate::common;
use bit_vec::BitVec;
use itertools::repeat_n;
use std::collections::HashMap;

type Grid = Vec<BitVec>;

fn small_grids(grid: &Grid, size: usize) -> Vec<Grid> {
    let mut small_grids = Vec::new();
    if size == 2 {
        for y in 0..(grid.len() / size) {
            for x in 0..(grid.len() / size) {
                small_grids.push(vec![
                    BitVec::from_fn(2, |i| grid[y * size][x * size + i]),
                    BitVec::from_fn(2, |i| grid[y * size + 1][x * size + i]),
                ]);
            }
        }
    } else {
        for y in 0..(grid.len() / size) {
            for x in 0..(grid.len() / size) {
                small_grids.push(vec![
                    BitVec::from_fn(3, |i| grid[y * size][x * size + i]),
                    BitVec::from_fn(3, |i| grid[y * size + 1][x * size + i]),
                    BitVec::from_fn(3, |i| grid[y * size + 2][x * size + i]),
                ]);
            }
        }
    }

    small_grids
}

fn flip(grid: &Grid) -> Grid {
    let mut new_grid = Vec::new();
    for row in grid {
        new_grid.push(BitVec::from_iter(row.iter().rev()));
    }
    new_grid
}

fn rotate(grid: &Grid) -> Grid {
    if grid.len() == 2 {
        vec![
            BitVec::from_fn(2, |i| grid[i][1]),
            BitVec::from_fn(2, |i| grid[i][0]),
        ]
    } else {
        vec![
            BitVec::from_fn(3, |i| grid[i][2]),
            BitVec::from_fn(3, |i| grid[i][1]),
            BitVec::from_fn(3, |i| grid[i][0]),
        ]
    }
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/21.txt")?;

    let mut two_rules: HashMap<Grid, Grid> = HashMap::new();
    let mut three_rules: HashMap<Grid, Grid> = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (input, output) = line.split_once(" => ").unwrap();

        if input.len() == 5 {
            two_rules.insert(
                input
                    .split('/')
                    .map(|r| r.chars().map(|c| c == '#').collect())
                    .collect(),
                output
                    .split('/')
                    .map(|r| r.chars().map(|c| c == '#').collect())
                    .collect(),
            );
        } else {
            three_rules.insert(
                input
                    .split('/')
                    .map(|r| r.chars().map(|c| c == '#').collect())
                    .collect(),
                output
                    .split('/')
                    .map(|r| r.chars().map(|c| c == '#').collect())
                    .collect(),
            );
        }
    }

    let mut solution_a = 0;

    let mut grid: Grid = vec![
        BitVec::from_fn(3, |i| i == 1),
        BitVec::from_fn(3, |i| i == 2),
        BitVec::from_elem(3, true),
    ];

    for iteration in 0..18 {
        let current_size = grid.len();
        if current_size % 2 == 0 {
            let small_grids = small_grids(&grid, 2);
            for row in &mut grid {
                row.extend(repeat_n(false, (current_size / 2) * 3 - current_size));
            }
            grid.resize(
                (current_size / 2) * 3,
                BitVec::from_elem((current_size / 2) * 3, false),
            );

            let mut y = 0;
            let mut x = 0;
            'grid_loop: for (i, mini_grid) in small_grids.into_iter().enumerate() {
                if i % (current_size / 2) == 0 {
                    if i != 0 {
                        y += 1;
                    }
                    x = 0;
                } else {
                    x += 1;
                }
                let mut current = mini_grid;
                for _ in 0..2 {
                    for _ in 0..4 {
                        if let Some(output) = two_rules.get(&current) {
                            grid[y * 3].set(x * 3, output[0][0]);
                            grid[y * 3].set(x * 3 + 1, output[0][1]);
                            grid[y * 3].set(x * 3 + 2, output[0][2]);

                            grid[y * 3 + 1].set(x * 3, output[1][0]);
                            grid[y * 3 + 1].set(x * 3 + 1, output[1][1]);
                            grid[y * 3 + 1].set(x * 3 + 2, output[1][2]);

                            grid[y * 3 + 2].set(x * 3, output[2][0]);
                            grid[y * 3 + 2].set(x * 3 + 1, output[2][1]);
                            grid[y * 3 + 2].set(x * 3 + 2, output[2][2]);

                            continue 'grid_loop;
                        }
                        current = rotate(&current);
                    }
                    current = flip(&current);
                }
                panic!("No rule matches {current:?}");
            }
        } else {
            let small_grids = small_grids(&grid, 3);
            for row in &mut grid {
                row.extend(repeat_n(false, (current_size / 3) * 4 - current_size));
            }
            grid.resize(
                (current_size / 3) * 4,
                BitVec::from_elem((current_size / 3) * 4, false),
            );

            let mut y = 0;
            let mut x = 0;
            'grid_loop: for (i, mini_grid) in small_grids.into_iter().enumerate() {
                if i % (current_size / 3) == 0 {
                    if i != 0 {
                        y += 1;
                    }
                    x = 0;
                } else {
                    x += 1;
                }
                let mut current = mini_grid;
                for _ in 0..2 {
                    for _ in 0..4 {
                        if let Some(output) = three_rules.get(&current) {
                            grid[y * 4].set(x * 4, output[0][0]);
                            grid[y * 4].set(x * 4 + 1, output[0][1]);
                            grid[y * 4].set(x * 4 + 2, output[0][2]);
                            grid[y * 4].set(x * 4 + 3, output[0][3]);

                            grid[y * 4 + 1].set(x * 4, output[1][0]);
                            grid[y * 4 + 1].set(x * 4 + 1, output[1][1]);
                            grid[y * 4 + 1].set(x * 4 + 2, output[1][2]);
                            grid[y * 4 + 1].set(x * 4 + 3, output[1][3]);

                            grid[y * 4 + 2].set(x * 4, output[2][0]);
                            grid[y * 4 + 2].set(x * 4 + 1, output[2][1]);
                            grid[y * 4 + 2].set(x * 4 + 2, output[2][2]);
                            grid[y * 4 + 2].set(x * 4 + 3, output[2][3]);

                            grid[y * 4 + 3].set(x * 4, output[3][0]);
                            grid[y * 4 + 3].set(x * 4 + 1, output[3][1]);
                            grid[y * 4 + 3].set(x * 4 + 2, output[3][2]);
                            grid[y * 4 + 3].set(x * 4 + 3, output[3][3]);
                            continue 'grid_loop;
                        }
                        current = rotate(&current);
                    }
                    current = flip(&current);
                }
                panic!("No rule matches {current:?}");
            }
        }
        if iteration == 4 {
            solution_a = grid.iter().map(|r| r.iter().filter(|b| *b).count()).sum();
        }
    }

    let solution_b = grid
        .into_iter()
        .map(|r| r.into_iter().filter(|b| *b).count())
        .sum();

    Ok((solution_a, solution_b))
}
