use std::collections::{HashMap, HashSet};

fn count_yes(input: &str) -> usize {
    let mut answers = HashSet::new();
    for a in input.lines() {
        for c in a.chars() {
            answers.insert(c);
        }
    }
    answers.len()
}
fn count_everyone_yes(input: &str) -> usize {
    let mut answers = HashMap::new();
    let answer_nb = input.lines().count();
    for a in input.lines() {
        for c in a.chars() {
            if let Some(v) = answers.get_mut(&c) {
                *v += 1
            } else {
                answers.insert(c, 1);
            }
        }
    }
    answers.values().filter(|&x| *x == answer_nb).count()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    input.split("\n\n").map(|l| count_yes(l)).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input.split("\n\n").map(|l| count_everyone_yes(l)).sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part2(input), 6);
    }
}
