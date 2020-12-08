use std::num::ParseIntError;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
enum ExitCode {
    Ok,
    InfiniteLoop,
}

#[derive(Debug, Clone)]
enum OperationType {
    None,
    Jump,
    Acc,
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: OperationType,
    value: Option<String>,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let operation = match s.next() {
            Some("nop") => OperationType::None,
            Some("acc") => OperationType::Acc,
            Some("jmp") => OperationType::Jump,
            _ => unimplemented!(),
        };

        Ok(Instruction {
            operation,
            value: s.next().map(|x| x.to_string()),
        })
    }
}

#[derive(Debug)]
struct Handheld {
    index: usize,
    accumulator: i64,
    instructions: Vec<Instruction>,
}

impl Handheld {
    fn new(input: &[Instruction]) -> Self {
        Handheld {
            index: 0,
            accumulator: 0,
            instructions: input.into(),
        }
    }

    fn execute_instruction(&mut self) -> () {
        let i = self.instructions[self.index].clone();
        let tmp = i.value.unwrap();
        let (op, value) = tmp.split_at(1);
        let value: usize = value.parse().unwrap();
        match i.operation {
            OperationType::None => self.index += 1,
            OperationType::Jump => match op {
                "+" => self.index += value,
                "-" => self.index -= value,
                _ => unimplemented!(),
            },
            OperationType::Acc => {
                match op {
                    "+" => self.accumulator += value as i64,
                    "-" => self.accumulator -= value as i64,
                    _ => unimplemented!(),
                }
                self.index += 1
            }
        }
    }

    fn run(&mut self) -> ExitCode {
        let mut used_index = HashSet::new();
        loop {
            self.execute_instruction();
            if used_index.contains(&self.index) {
                return ExitCode::InfiniteLoop;
            }
            if self.index == self.instructions.len() {
                return ExitCode::Ok;
            }

            used_index.insert(self.index);
            // println!("{:?}", h);
        }
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Instruction]) -> i64 {
    let mut h = Handheld::new(input);
    h.run();
    h.accumulator
}

#[aoc(day8, part2)]
fn part2(input: &[Instruction]) -> i64 {
    for (i, inst) in input.iter().enumerate() {
        let op = match inst.operation {
            OperationType::Acc => continue,
            OperationType::None => OperationType::Jump,
            OperationType::Jump => OperationType::None,
        };

        let mut tmp_input = input.to_vec();
        tmp_input[i].operation = op;

        // let mut new_input = input.to_vec();
        // new_input[i] =

        let mut h = Handheld::new(&tmp_input);
        if h.run() == ExitCode::Ok {
            return h.accumulator;
        }
    }
    return 0;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(part1(&input_generator(input)), 5);
    }
    #[test]
    fn test_part2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(part2(&input_generator(input)), 8);
    }
}
