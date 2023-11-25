use anyhow::Result;
use std::collections::VecDeque;

use crate::common;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Value {
    Register(usize),
    Literal(i64),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Command {
    Snd(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
}

fn parse_value(s: &str) -> Result<Value> {
    let c = s.chars().next().unwrap();
    Ok(if c.is_alphabetic() {
        Value::Register(c as usize - 'a' as usize)
    } else {
        Value::Literal(s.parse()?)
    })
}

fn get_value(registers: &[i64; 26], v: Value) -> i64 {
    match v {
        Value::Register(r) => registers[r],
        Value::Literal(v) => v,
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::match_on_vec_items)]
pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/18.txt")?;

    let mut commands = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        let parts = common::RE_WS.split(line).collect::<Vec<&str>>();

        match parts[0] {
            "snd" => commands.push(Command::Snd(parse_value(parts[1])?)),
            "set" => commands.push(Command::Set(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
                parse_value(parts[2])?,
            )),
            "add" => commands.push(Command::Add(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
                parse_value(parts[2])?,
            )),
            "mul" => commands.push(Command::Mul(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
                parse_value(parts[2])?,
            )),
            "mod" => commands.push(Command::Mod(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
                parse_value(parts[2])?,
            )),
            "rcv" => commands.push(Command::Rcv(
                parts[1].chars().next().unwrap() as usize - 'a' as usize,
            )),
            "jgz" => commands.push(Command::Jgz(parse_value(parts[1])?, parse_value(parts[2])?)),
            p => panic!("Unknown command {p}"),
        }
    }

    let solution_a;
    let mut registers = [0; 26];
    let mut last_sound = 0;
    let mut pc = 0;

    loop {
        match commands[pc] {
            Command::Snd(x) => last_sound = get_value(&registers, x),
            Command::Set(x, y) => registers[x] = get_value(&registers, y),
            Command::Add(x, y) => registers[x] += get_value(&registers, y),
            Command::Mul(x, y) => registers[x] *= get_value(&registers, y),
            Command::Mod(x, y) => registers[x] %= get_value(&registers, y),
            Command::Rcv(x) if registers[x] == 0 => {}
            Command::Rcv(_) => {
                solution_a = last_sound;
                break;
            }
            Command::Jgz(x, y) if get_value(&registers, x) > 0 => {
                pc = (pc as i64 + get_value(&registers, y)) as usize;
                continue;
            }
            Command::Jgz(_, _) => {}
        }
        pc += 1;
    }

    // Part b
    let mut solution_b = 0;
    let mut rcv_queue_left = VecDeque::new();
    let mut rcv_queue_right = VecDeque::new();

    let mut left = Cpu::new(1);
    let mut right = Cpu::new(0);

    loop {
        if let Command::Snd(_) = commands[left.pc] {
            solution_b += 1;
        }
        let diff = left.step(commands[left.pc], &mut rcv_queue_left, &mut rcv_queue_right);
        if left.waiting && (!right.running || right.waiting) {
            break;
        }
        let new_pc = left.pc as i64 + diff;
        if new_pc < 0 || new_pc as usize >= commands.len() {
            break;
        }
        left.pc = new_pc as usize;
        if right.running {
            let diff = right.step(
                commands[right.pc],
                &mut rcv_queue_right,
                &mut rcv_queue_left,
            );
            if left.waiting && right.waiting {
                break;
            }
            let new_pc = right.pc as i64 + diff;
            if new_pc < 0 || new_pc as usize >= commands.len() {
                right.running = false;
            } else {
                right.pc = new_pc as usize;
            }
        }
    }

    Ok((solution_a, solution_b))
}

struct Cpu {
    registers: [i64; 26],
    pc: usize,
    running: bool,
    waiting: bool,
}

impl Cpu {
    fn new(program_id: i64) -> Self {
        let mut registers = [0; 26];
        registers['p' as usize - 'a' as usize] = program_id;
        Cpu {
            registers,
            pc: 0,
            running: true,
            waiting: false,
        }
    }

    fn step(
        &mut self,
        command: Command,
        rcv_queue: &mut VecDeque<i64>,
        snd_queue: &mut VecDeque<i64>,
    ) -> i64 {
        self.waiting = false;
        match command {
            Command::Snd(x) => snd_queue.push_back(get_value(&self.registers, x)),
            Command::Set(x, y) => self.registers[x] = get_value(&self.registers, y),
            Command::Add(x, y) => self.registers[x] += get_value(&self.registers, y),
            Command::Mul(x, y) => self.registers[x] *= get_value(&self.registers, y),
            Command::Mod(x, y) => self.registers[x] %= get_value(&self.registers, y),
            Command::Rcv(x) => match rcv_queue.pop_front() {
                None => {
                    self.waiting = true;
                    return 0;
                }
                Some(v) => self.registers[x] = v,
            },
            Command::Jgz(x, y) if get_value(&self.registers, x) > 0 => {
                return get_value(&self.registers, y)
            }
            Command::Jgz(_, _) => {}
        }
        1
    }
}
