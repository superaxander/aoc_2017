use anyhow::Result;

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
pub fn main() -> Result<(i64, i64)> {
    let input = 265_149;

    let mut ring: i64 = 1;
    let mut mult = 8;
    let mut width = 3;
    let mut count = 1;

    while count + mult < input {
        count += mult;
        mult += 8;
        width += 2;
        ring += 1;
    }

    let mut x = ring;
    let mut y = ring - 1;

    let mut remaining = input - count;
    width -= 1;

    if remaining - width + 1 > 0 {
        y -= width - 1;
        remaining -= width;
        if remaining - width > 0 {
            x -= width;
            remaining -= width;
            if remaining - width > 0 {
                y += width;
                remaining -= width;
                x += remaining;
            } else {
                y += remaining;
            }
        } else {
            x -= remaining;
        }
    } else {
        y -= remaining - 1;
    }

    let solution_a = x.abs() + y.abs();

    let solution_b: i64;
    let size = (ring * 2 + 1) as usize;
    let mut grid = vec![vec![-1; size]; size];
    let mut x = ring as usize;
    let mut y = ring as usize;
    grid[x][y] = 1;
    x += 1;

    loop {
        let mut val = 0;
        for x2 in -1..=1 {
            for y2 in -1..=1 {
                if grid[(x as i32 + x2) as usize][(y as i32 + y2) as usize] != -1 {
                    val += grid[(x as i32 + x2) as usize][(y as i32 + y2) as usize];
                }
            }
        }
        grid[x][y] = val;
        if val > input {
            solution_b = val;
            break;
        }
        match (
            grid[x][y - 1],
            grid[x - 1][y],
            grid[x + 1][y],
            grid[x][y + 1],
        ) {
            (a, -1, _, -1) if a >= 0 => x -= 1, // go left
            (_, _, -1, -1) => y += 1,           // go up
            (-1, _, -1, _) => x += 1,           // go right
            (-1, _, a, _) if a >= 0 => y -= 1,  // go down
            _ => panic!("It's broken jim"),
        }
    }

    // for line in lines {
    //     if let Ok(num) = line?.trim().parse::<i64>() {

    //     } else {
    //     }
    // }

    Ok((solution_a, solution_b))
}
