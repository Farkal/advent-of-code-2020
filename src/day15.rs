use std::collections::HashMap;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn run(input: &[i32], rounds: usize) -> i32 {
    let mut values = HashMap::with_capacity(rounds);
    for (i, v) in input[..input.len() - 1].iter().enumerate() {
        values.insert(*v, i as u32 + 1);
    }
    let mut last = *input.last().unwrap();
    // println!("V {:?}", values);
    // let mut values = input.to_vec();
    for round in input.len()..rounds {
        // println!("R: {} Last value {}", round, last);
        last = if let Some(r_p) = values.insert(last, round as u32) {
            round as i32 - r_p as i32
        } else {
            0
        };
        // println!("res {}", last);
    }

    last
}

#[aoc(day15, part1)]
fn part1(input: &[i32]) -> i32 {
    run(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &[i32]) -> i32 {
    run(input, 30000000)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0,3,6";
        assert_eq!(part1(&input_generator(input)), 436);
        let input = "1,3,2";
        assert_eq!(part1(&input_generator(input)), 1);
        let input = "2,1,3";
        assert_eq!(part1(&input_generator(input)), 10);
        let input = "1,2,3";
        assert_eq!(part1(&input_generator(input)), 27);
        let input = "3,1,2";
        assert_eq!(part1(&input_generator(input)), 1836);
    }

    #[test]
    fn test_part2() {
        let input = "0,3,6";
        assert_eq!(part2(&input_generator(input)), 175594);
    }
}
