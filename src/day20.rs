use anyhow::Result;
use regex::Regex;

use crate::common;

#[derive(Copy, Clone, Debug)]
struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}

fn manhattan((x, y, z): (i64, i64, i64)) -> i64 {
    x.abs() + y.abs() + z.abs()
}

pub fn main() -> Result<(usize, usize)> {
    let regex =
        Regex::new("p=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, v=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, a=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>")?;
    let lines = common::read_lines("inputs/20.txt")?;

    let mut particles = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        if let Some(caps) = regex.captures(line) {
            particles.push(Particle {
                position: (caps[1].parse()?, caps[2].parse()?, caps[3].parse()?),
                velocity: (caps[4].parse()?, caps[5].parse()?, caps[6].parse()?),
                acceleration: (caps[7].parse()?, caps[8].parse()?, caps[9].parse()?),
            });
        }
    }

    let solution_a = particles
        .iter()
        .enumerate()
        .min_by(|(_, p), (_, q)| {
            manhattan(p.acceleration)
                .cmp(&manhattan(q.acceleration))
                .then_with(|| {
                    manhattan(p.velocity)
                        .cmp(&manhattan(q.velocity))
                        .then_with(|| manhattan(p.position).cmp(&manhattan(q.position)))
                })
        })
        .unwrap()
        .0;

    let mut counter = 100;
    while !particles.is_empty() && counter >= 0 {
        particles.sort_unstable_by_key(|p| p.position);

        let mut last_pos = particles[0].position;
        let mut chaining = false;
        let mut i = 1;
        while i < particles.len() {
            if last_pos == particles[i].position {
                if chaining {
                    particles.remove(i);
                } else {
                    particles.remove(i - 1);
                    particles.remove(i - 1);
                    i -= 1;
                    chaining = true;
                }
            } else {
                last_pos = particles[i].position;
                chaining = false;
                i += 1;
            }
        }

        for p in &mut particles {
            p.velocity.0 += p.acceleration.0;
            p.velocity.1 += p.acceleration.1;
            p.velocity.2 += p.acceleration.2;
            p.position.0 += p.velocity.0;
            p.position.1 += p.velocity.1;
            p.position.2 += p.velocity.2;
        }
        counter -= 1;
    }

    let solution_b = particles.len();

    Ok((solution_a, solution_b))
}
