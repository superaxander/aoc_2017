use anyhow::Result;

#[allow(clippy::unnecessary_wraps)]
pub fn main() -> Result<(usize, usize)> {
    let step_size = 328;
    let mut buffer = vec![0];
    let mut position = 0;

    for i in 1..=2017 {
        position = (position + step_size) % i;
        buffer.insert(position + 1, i);
        position += 1;
    }

    let solution_a = buffer[(position + 1) % 2018];

    let mut solution_b = 0;
    for i in 2018..=50_000_000 {
        position = (position + step_size) % i;
        if position == 0 {
            solution_b = i;
        }
        position += 1;
    }

    Ok((solution_a, solution_b))
}
