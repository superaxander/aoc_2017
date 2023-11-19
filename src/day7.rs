use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::common;

pub fn main() -> Result<(String, i64)> {
    let regex =
        Regex::new("([A-z]+)\\s+\\(([0-9]+)\\)(?:\\s*->\\s*((?:(?:[A-z]+),\\s*)*(?:[A-z]+)))?")?;
    let lines = common::read_lines("inputs/7.txt")?;

    let mut tops = HashSet::new();
    let mut tree = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        if let Some(caps) = regex.captures(line) {
            let name = &caps[1];
            let weight = caps[2].parse::<i64>()?;
            let mut vec = Vec::new();
            if let Some(atops) = caps.get(3) {
                for atop in atops.as_str().split(", ") {
                    tops.insert(atop.to_owned());
                    vec.push(atop.to_owned());
                }
            }
            tree.insert(name.to_owned(), (weight, vec));
        }
    }

    let solution_a = tree.keys().find(|it| !tops.contains(*it)).unwrap();
    let solution_b = get_weight(solution_a, &tree).unwrap_err();

    Ok((solution_a.to_owned(), solution_b))
}

#[allow(clippy::cast_possible_wrap)]
fn get_weight(
    bottom: &str,
    tree: &HashMap<String, (i64, Vec<String>)>,
) -> std::result::Result<i64, i64> {
    let mut child_weight_1 = -1;
    let mut count_1 = 0;
    let mut last_1 = -1;
    let mut child_weight_2 = -1;
    let mut count_2 = 0;
    let mut last_2 = -1;
    for child in &tree[bottom].1 {
        match get_weight(child, tree) {
            Ok(w) => {
                if child_weight_1 == -1 || child_weight_1 == w {
                    child_weight_1 = w;
                    count_1 += 1;
                    last_1 = tree[child].0;
                } else if child_weight_2 == -1 || child_weight_2 == w {
                    child_weight_2 = w;
                    count_2 += 1;
                    last_2 = tree[child].0;
                } else {
                    panic!("This approach doesn't work apparently");
                }
            }
            Err(b) => return Err(b),
        }
    }

    if child_weight_2 == -1 {
        Ok(tree[bottom].0 + child_weight_1 * tree[bottom].1.len() as i64)
    } else if count_1 > count_2 {
        Err(last_2 + child_weight_1 - child_weight_2)
    } else {
        Err(last_1 + child_weight_2 - child_weight_1)
    }
}
