use std::collections::HashMap;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn use_all_adapters(input: &[u64]) -> (u32, u32, u32) {
    let goal = input.iter().max().unwrap() + 3;
    let mut input = input.to_vec();
    input.push(0);
    input.push(goal);
    input.sort();
    let (mut count_1, mut count_2, mut count_3) = (0, 0, 0);
    for i in 0..input.len() - 1 {
        let diff = input[i + 1] - input[i];
        // println!("{}", diff);

        match diff {
            1 => count_1 += 1,
            2 => count_2 += 1,
            3 => count_3 += 1,
            _ => unimplemented!(),
        }
    }
    (count_1, count_2, count_3)
}

fn count_possibilities(input: &[u64]) -> u64 {
    let mut input = input.to_vec();
    input.sort();
    let mut p = HashMap::new();
    p.insert(0, 1);
    for i in &input {
        let s = if i < &3 {
            if i < &2 {
                1
            } else {
                1 + p.get(&(i - 2)).unwrap_or(&0)
            }
        } else {
            p.get(&(i - 1)).unwrap_or(&0)
                + p.get(&(i - 2)).unwrap_or(&0)
                + p.get(&(i - 3)).unwrap_or(&0)
        };
        p.insert(*i, s);
    }
    *p.get(input.last().unwrap_or(&0)).unwrap_or(&0)
}

#[aoc(day10, part1)]
fn part1(input: &[u64]) -> u32 {
    let (o, _, t) = use_all_adapters(input);
    o * t
}

#[aoc(day10, part2)]
fn part2(input: &[u64]) -> u64 {
    count_possibilities(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let (o, _, t) = use_all_adapters(&input);
        assert_eq!(o * t, 35);
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let (o, _, t) = use_all_adapters(&input);
        assert_eq!(o * t, 220);
    }

    #[test]
    fn test_part2() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(count_possibilities(&input), 8);
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(count_possibilities(&input), 19208);
    }
}
