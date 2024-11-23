use crate::common;
use anyhow::Result;
use common::is_prime;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Write};
use std::rc::Rc;
use tracing::debug;

const REG_COUNT: usize = 8;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Value {
    Register(usize),
    Literal(i64),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Command {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jnz(Value, Value),
}

fn parse_value(s: &str) -> Result<Value> {
    let c = s.chars().next().unwrap();
    Ok(if c.is_alphabetic() {
        Value::Register(c as usize - 'a' as usize)
    } else {
        Value::Literal(s.parse()?)
    })
}

fn get_value(registers: &[i64; REG_COUNT], v: Value) -> i64 {
    match v {
        Value::Register(r) => registers[r],
        Value::Literal(v) => v,
    }
}

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/23.txt")?;

    let mut commands = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        let parts = common::RE_WS.split(line).collect::<Vec<&str>>();

        match parts[0] {
            "set" => commands.push(Command::Set(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
                parse_value(parts[2])?,
            )),
            "sub" => commands.push(Command::Sub(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
                parse_value(parts[2])?,
            )),
            "mul" => commands.push(Command::Mul(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
                parse_value(parts[2])?,
            )),
            "jnz" => commands.push(Command::Jnz(parse_value(parts[1])?, parse_value(parts[2])?)),
            p => panic!("Unknown command {p}"),
        }
    }

    let mut solution_a = 0;
    let mut registers = [0; REG_COUNT];
    let mut pc = 0;

    while pc < commands.len() {
        match commands[pc] {
            Command::Set(x, y) => registers[x] = get_value(&registers, y),
            Command::Sub(x, y) => registers[x] -= get_value(&registers, y),
            Command::Mul(x, y) => {
                solution_a += 1;
                registers[x] *= get_value(&registers, y)
            }
            Command::Jnz(x, y) if get_value(&registers, x) != 0 => {
                pc = (pc as i64 + get_value(&registers, y)) as usize;
                continue;
            }
            Command::Jnz(_, _) => {}
        }
        pc += 1;
    }

    // Part b

    // Find jump targets
    let mut targets = BTreeSet::new();
    targets.insert(0);
    for (i, command) in commands.iter().enumerate() {
        match command {
            Command::Jnz(Value::Register(_), Value::Literal(offset)) => {
                let target = (i as i64 + *offset) as usize;
                if target < commands.len() {
                    targets.insert(target);
                }
                let target = i + 1;
                if target < commands.len() {
                    targets.insert(target);
                }
            }
            Command::Jnz(Value::Literal(0), Value::Literal(_)) => {
                let target = i + 1;
                if target < commands.len() {
                    targets.insert(target);
                }
            }
            Command::Jnz(Value::Literal(_), Value::Literal(offset)) => {
                let target = (i as i64 + *offset) as usize;
                if target < commands.len() {
                    targets.insert(target);
                }
            }
            _ => {}
        }
    }

    let mut bbs = Vec::new();
    let mut bb = BB {
        start_index: 0,
        ..Default::default()
    };
    for (i, command) in commands.iter().enumerate() {
        if targets.contains(&i) && bb.start_index != i {
            if bb.ifnz.is_none() && bb.ifz.is_none() {
                bb.ifz = Some(targets.iter().position(|x| *x == i).unwrap());
                bb.ifnz = bb.ifz;
            }
            bbs.push(bb);
            bb = BB {
                start_index: i,
                ..Default::default()
            }
        }
        match command {
            Command::Set(_, _) | Command::Sub(_, _) | Command::Mul(_, _) => {
                bb.commands.push(*command)
            }
            Command::Jnz(Value::Register(v), Value::Literal(offset)) => {
                bb.variable = Some(*v);
                let nz_index = (i as i64 + *offset) as usize;
                if nz_index < commands.len() {
                    bb.ifnz = Some(targets.iter().position(|x| *x == nz_index).unwrap());
                } else {
                    bb.ifnz = Some(commands.len());
                }
                let z_index = i + 1;
                if z_index < commands.len() {
                    bb.ifz = Some(targets.iter().position(|x| *x == z_index).unwrap());
                } else {
                    bb.ifz = Some(commands.len());
                }
            }
            Command::Jnz(Value::Literal(0), Value::Literal(_)) => {
                let z_index = i + 1;
                if z_index < commands.len() {
                    bb.ifz = Some(targets.iter().position(|x| *x == z_index).unwrap());
                } else {
                    bb.ifz = Some(commands.len());
                }
                bb.ifnz = bb.ifz;
            }
            Command::Jnz(Value::Literal(_), Value::Literal(offset)) => {
                let nz_index = (i as i64 + *offset) as usize;
                if nz_index < commands.len() {
                    bb.ifnz = Some(targets.iter().position(|x| *x == nz_index).unwrap());
                } else {
                    bb.ifnz = Some(commands.len());
                }
                bb.ifz = bb.ifnz;
            }
            Command::Jnz(_, Value::Register(_)) => panic!("Jump to register unsupported"),
        }
    }
    bbs.push(bb);

    for i in (0..bbs.len()).rev() {
        if bbs[i].commands.is_empty() && bbs[i].variable.is_none() {
            let target = bbs.remove(i).ifz;
            for bb in bbs.iter_mut() {
                bb.ifz = match bb.ifz {
                    None => None,
                    Some(j) if j == commands.len() => Some(commands.len()),
                    Some(j) if j == i => target,
                    Some(j) if j > i => Some(j - 1),
                    Some(j) => Some(j),
                };
                bb.ifnz = match bb.ifnz {
                    None => None,
                    Some(j) if j == commands.len() => Some(commands.len()),
                    Some(j) if j == i => target,
                    Some(j) if j > i => Some(j - 1),
                    Some(j) => Some(j),
                };
            }
        }
    }

    // Show CFG for manual evaluation:
    let mut graph = String::new();
    graph.push_str("digraph {\n\tnode [shape=record];\n");
    for (idx, bb) in bbs.into_iter().enumerate() {
        const REP_VALUE: Option<Rc<Equation>> = None;
        let mut formulas: [Option<Rc<Equation>>; REG_COUNT] = [REP_VALUE; REG_COUNT];
        for command in bb.commands {
            match command {
                Command::Set(a, b) => {
                    if let Value::Register(b) = b
                        && let Some(b) = formulas[b].as_ref()
                    {
                        formulas[a] = Some(Rc::clone(b));
                    } else {
                        formulas[a] = Some(Rc::new(Equation::Value(b)))
                    }
                }
                Command::Sub(a, b) => {
                    let old_a = formulas[a].take();
                    if let Value::Register(b) = b
                        && let Some(b) = formulas[b].as_ref()
                    {
                        formulas[a] =
                            old_a
                                .map(|a| Equation::sub(a, Rc::clone(b)))
                                .or(Some(Equation::sub(
                                    Equation::value(Value::Register(a)),
                                    Rc::clone(b),
                                )))
                    } else {
                        formulas[a] = old_a.map(|a| Equation::sub(a, Equation::value(b))).or(Some(
                            Equation::sub(Equation::value(Value::Register(a)), Equation::value(b)),
                        ))
                    }
                }
                Command::Mul(a, b) => {
                    let old_a = formulas[a].take();
                    if let Value::Register(b) = b
                        && let Some(b) = formulas[b].as_ref()
                    {
                        formulas[a] =
                            old_a
                                .map(|a| Equation::mul(a, Rc::clone(b)))
                                .or(Some(Equation::mul(
                                    Equation::value(Value::Register(a)),
                                    Rc::clone(b),
                                )))
                    } else {
                        formulas[a] = old_a
                            .map(|a| Equation::mul(a, Rc::new(Equation::Value(b))))
                            .or(Some(Equation::mul(
                                Equation::value(Value::Register(a)),
                                Equation::value(b),
                            )))
                    }
                }
                _ => panic!(),
            }
        }
        let mut row1 = String::new();
        row1.push('{');
        for (i, formula) in formulas.iter().enumerate() {
            if let Some(formula) = formula {
                let c = (i as u8 + b'a') as char;
                write!(&mut row1, "{{{c} | {}}}|", formula)?;
            }
        }
        if row1.len() > 1 {
            row1.pop();
            row1.push('}');
        } else {
            row1 = String::new();
        }
        let row2 = match bb.variable {
            None => "".to_owned(),
            Some(v) => {
                let c = (v as u8 + b'a') as char;
                if row1.is_empty() {
                    format!("{{<z>{c} == 0 | <nz>{c} != 0}}")
                } else {
                    format!("| {{<z>{c} == 0 | <nz>{c} != 0}}")
                }
            }
        };
        write!(&mut graph, "\t{idx} [label=\"{{{row1}{row2}}}\"];")?;
        match bb.variable {
            None => {
                writeln!(&mut graph, "\t{idx} -> {}", bb.ifz.unwrap())?;
            }
            Some(_) => {
                writeln!(
                    &mut graph,
                    "\t{idx}:z -> {}\n\t{idx}:nz -> {}",
                    bb.ifz.unwrap(),
                    bb.ifnz.unwrap()
                )?;
            }
        }
    }
    graph.push('}');
    let encoded = common::url_encode(&graph);
    debug!("CFG: https://dreampuf.github.io/GraphvizOnline/#{encoded}");

    // Optimised
    let mut solution_b = 0;
    for b in (106_517..=123_500).step_by(34) {
        if !is_prime(b) {
            solution_b += 1;
        }
    }
    // We skipped all the even numbers so add those
    solution_b += 1 + (123_500 - 106_500) / 34;

    Ok((solution_a, solution_b))
}

#[derive(Default, Debug)]
struct BB {
    commands: Vec<Command>,
    start_index: usize,
    variable: Option<usize>,
    ifz: Option<usize>,
    ifnz: Option<usize>,
}

#[derive(Clone, Debug)]
enum Equation {
    Value(Value),
    Mul(Rc<Equation>, Rc<Equation>),
    Sub(Rc<Equation>, Rc<Equation>),
}

impl Equation {
    fn value(value: Value) -> Rc<Self> {
        Rc::new(Equation::Value(value))
    }

    fn mul(l: Rc<Equation>, r: Rc<Equation>) -> Rc<Self> {
        Rc::new(match (&*l, &*r) {
            (Equation::Value(Value::Literal(l)), Equation::Value(Value::Literal(r))) => {
                Equation::Value(Value::Literal(l * r))
            }
            (_, _) => Equation::Mul(l, r),
        })
    }

    fn sub(l: Rc<Equation>, r: Rc<Equation>) -> Rc<Self> {
        Rc::new(match (&*l, &*r) {
            (Equation::Value(Value::Literal(l)), Equation::Value(Value::Literal(r))) => {
                Equation::Value(Value::Literal(l - r))
            }
            (_, _) => Equation::Sub(l, r),
        })
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Equation::Value(Value::Register(r)) => f.write_char((*r as u8 + b'a') as char),
            Equation::Value(Value::Literal(v)) => write!(f, "{v}"),
            Equation::Mul(a, b) => write!(f, "({a} * {b})"),
            Equation::Sub(a, b) => write!(f, "({a} - {b})"),
        }
    }
}
