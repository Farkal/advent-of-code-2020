use std::collections::HashMap;

enum Instruction {
    Mask(String),
    Mem(u32, String),
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Instruction> {
    let mut res = vec![];
    for l in input.lines() {
        let mut t = l.split(" = ");
        let i = match t.next() {
            Some("mask") => Instruction::Mask(t.next().unwrap().to_string()),
            Some(x) => {
                let sb = x.find("[").unwrap() + 1;
                let eb = x.find("]").unwrap();
                let mem_addr = x[sb..eb].parse().unwrap();
                Instruction::Mem(
                    mem_addr,
                    format!("{:036b}", t.next().unwrap().parse::<u64>().unwrap()),
                )
            }
            _ => unimplemented!(),
        };
        res.push(i)
    }
    res
}

fn apply_mask(mask: &str, value: &str) -> u64 {
    let mut res = 0;
    let mut mult: u32 = 36;
    // println!("m {}", mask);
    // println!("v {}", value);
    for (m, v) in mask.chars().zip(value.chars()) {
        mult -= 1;
        if m == 'X' {
            res += 2u64.pow(mult) as u64 * v.to_digit(10).unwrap() as u64
        } else {
            res += 2u64.pow(mult) as u64 * m.to_digit(10).unwrap() as u64
        }
    }
    // println!("r {}", res);
    res
}

fn to_int(value: &str) -> u64 {
    value
        .chars()
        .enumerate()
        .map(|(i, x)| 2u64.pow(35 - i as u32) as u64 * x.to_digit(10).unwrap() as u64)
        .sum()
}

fn replace_x(input: &str) -> Vec<String> {
    if !input.contains("X") {
        return vec![input.to_string()];
    } else {
        let mut res = vec![];
        let s0 = input.replacen("X", "0", 1);
        let s1 = input.replacen("X", "1", 1);
        res.append(&mut replace_x(&s0));
        res.append(&mut replace_x(&s1));
        return res;
    }
}

fn apply_mask2(mask: &str, value: &str) -> Vec<u64> {
    let s: String = value
        .chars()
        .zip(mask.chars())
        .map(|(v, m)| match m {
            '0' => v,
            '1' | 'X' => m,
            _ => unreachable!(),
        })
        .collect();
    let res = replace_x(&s);
    res.iter().map(|x| to_int(&x)).collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = "";
    for i in input {
        match i {
            Instruction::Mask(v) => mask = v,
            Instruction::Mem(a, v) => {
                mem.insert(a, apply_mask(mask, v));
            }
        }
    }
    mem.values().sum()
}

#[aoc(day14, part2)]
fn part2(input: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = "";
    for i in input {
        match i {
            Instruction::Mask(v) => mask = v,
            Instruction::Mem(a, v) => {
                for m in apply_mask2(mask, &format!("{:036b}", a)) {
                    // println!("memory {}", m);
                    mem.insert(m, to_int(v));
                }
            }
        }
    }
    mem.values().sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(part1(&input_generator(input)), 165)
    }

    #[test]
    fn test_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(part2(&input_generator(input)), 208)
    }
}
