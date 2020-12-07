use std::collections::HashSet;

fn get_bag_contain(search: &str, rules: Vec<&str>) -> HashSet<String> {
    let mut res = HashSet::new();
    for r in &rules {
        // println!("{}, {} {}", r, search, r.contains(search));
        let mut s = r.splitn(2, " bags");
        let b = s.next().unwrap();
        let c = s.next().unwrap();
        if c.contains(search) {
            // println!("{}", b);
            res.insert(b.to_string());
            res.extend(get_bag_contain(b, rules.clone()));
        }
    }
    res
}

fn count_bag_contain(search: &str, rules: Vec<&str>) -> u32 {
    let mut count = 1;
    for r in &rules {
        // println!("{}, {} {}", r, search, r.contains(search));
        let mut s = r.splitn(2, " bags contain ");
        let b = s.next().unwrap();
        let c = s.next().unwrap();
        if b.contains(search) {
            for bag in c.split(", ") {
                let nb = bag.chars().next().unwrap().to_digit(10).unwrap_or(0);
                if nb > 0 {
                    let tmp_s: Vec<&str> = bag.split_whitespace().collect();
                    count += nb * count_bag_contain(&tmp_s[1..3].join(" "), rules.clone());
                    // println!("{} - {}", count, tmp_s[1..3].join(" "));
                }
            }
        }
    }
    count
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let rules = input.lines().collect();
    get_bag_contain("shiny gold", rules).len()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u32 {
    let rules = input.lines().collect();
    count_bag_contain("shiny gold", rules) - 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(part1(&input), 4);
    }
    #[test]
    fn test_part2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(part2(&input), 32);
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(part2(&input), 126);
    }
}
