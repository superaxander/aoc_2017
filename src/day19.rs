use anyhow::Result;

use crate::common;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(clippy::match_on_vec_items)]
pub fn main() -> Result<(String, i64)> {
    let lines = common::read_lines("inputs/19.txt")?;

    let mut solution_a = String::new();
    let mut solution_b = 0;

    let mut grid = Vec::new();
    for line in lines {
        grid.push(line?.chars().collect::<Vec<char>>());
    }

    let mut x = grid[0].iter().position(|c| *c == '|').unwrap();
    let mut y = 1;
    let mut direction = Direction::Down;

    loop {
        solution_b += 1;
        match grid[y][x] {
            '|' | '-' => match direction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            },
            ' ' => break,
            '+' => {
                if direction != Direction::Left && direction != Direction::Right {
                    if x > 0 && grid[y][x - 1] != ' ' {
                        x -= 1;
                        direction = Direction::Left;
                        continue;
                    }
                    if x < grid[y].len() - 1 && grid[y][x + 1] != ' ' {
                        x += 1;
                        direction = Direction::Right;
                        continue;
                    }
                }
                if direction != Direction::Up && direction != Direction::Down {
                    if y > 0 && grid[y - 1][x] != ' ' {
                        y -= 1;
                        direction = Direction::Up;
                        continue;
                    }
                    if y < grid.len() - 1 && grid[y + 1][x] != ' ' {
                        y += 1;
                        direction = Direction::Down;
                        continue;
                    }
                }
            }
            letter => {
                solution_a.push(letter);
                match direction {
                    Direction::Up => y -= 1,
                    Direction::Down => y += 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }
            }
        }
    }

    Ok((solution_a, solution_b))
}
