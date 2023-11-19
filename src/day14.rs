use anyhow::Result;

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::explicit_counter_loop)]
fn hash(string: &str) -> u128 {
    const LIST_SIZE: i64 = 256;
    let mut lengths = string.chars().map(|c| c as i64).collect::<Vec<i64>>();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut list = (0..LIST_SIZE).collect::<Vec<i64>>();
    let mut skip_size = 0;
    let mut index = 0;

    for _ in 0..64 {
        for length in &lengths {
            for i in 0..(*length / 2) {
                list.swap(
                    ((index + i) % LIST_SIZE) as usize,
                    ((index + *length - i - 1) % LIST_SIZE) as usize,
                );
            }
            index += *length + skip_size;
            skip_size += 1;
        }
    }

    let mut out = 0u128;
    for i in 0..16 {
        let mut num = 0;
        for j in 0..16 {
            num ^= list[i * 16 + j];
        }
        out = (out << 8) | (num as u128);
    }

    out
}

#[allow(clippy::unnecessary_wraps)]
pub fn main() -> Result<(u32, i64)> {
    let mut solution_a = 0;
    let mut solution_b = 0;
    let mut regions = [0; 128 * 128];
    for y in 0..128 {
        let hashed = hash(&format!("oundnydw-{y}"));
        solution_a += hashed.count_ones();
        for x in 0..128 {
            if hashed & (1u128 << x) > 0 {
                let mut region = 0;
                if y > 0 {
                    region |= regions[(y - 1) * 128 + x];
                }
                if region > 0 {
                    if x > 0 {
                        let left = regions[y * 128 + x - 1];
                        if left > 0 && left != region {
                            solution_b -= 1;
                            for i in 0..x {
                                if regions[y * 128 + i] == left {
                                    regions[y * 128 + i] = region;
                                }
                            }
                        }
                    }
                    regions[y * 128 + x] = region;
                } else {
                    if x > 0 {
                        region |= regions[y * 128 + x - 1];
                    }
                    if region > 0 {
                        regions[y * 128 + x] = region;
                    } else {
                        solution_b += 1;
                        regions[y * 128 + x] = solution_b;
                    }
                }
            }
        }
    }

    Ok((solution_a, solution_b))
}
