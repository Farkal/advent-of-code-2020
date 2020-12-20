use regex::bytes::Regex;
use std::collections::HashMap;

#[aoc_generator(day19, part1)]
fn input_generator(input: &str) -> (String, Vec<String>) {
    let mut input = input.split("\n\n");
    let rules = input.next().unwrap();
    let messages = input.next().unwrap().lines().map(|x| x.into()).collect();

    let mut hash_rules = HashMap::new();
    for r in rules.lines() {
        let mut r = r.split(":");
        let nb = r.next().unwrap().parse().unwrap();
        let rule = r.next().unwrap();
        hash_rules.insert(nb, rule);
    }
    (regex_from_rule(&hash_rules, 0, false), messages)
}
#[aoc_generator(day19, part2)]
fn input_generator2(input: &str) -> (String, Vec<String>) {
    let mut input = input.split("\n\n");
    let rules = input.next().unwrap();
    let messages = input.next().unwrap().lines().map(|x| x.into()).collect();

    let mut hash_rules = HashMap::new();
    for r in rules.lines() {
        let mut r = r.split(":");
        let nb = r.next().unwrap().parse().unwrap();
        let rule = r.next().unwrap();
        hash_rules.insert(nb, rule);
    }
    (regex_from_rule(&hash_rules, 0, true), messages)
}

fn regex_from_rule(rules: &HashMap<usize, &str>, index: usize, part2: bool) -> String {
    let init = rules.get(&index).unwrap();
    if init.contains("\"") {
        return init.replace("\"", "").trim().to_string();
    }
    if part2 && index == 8 {
        return format!("({}+)", regex_from_rule(&rules, 42, part2));
    }
    if part2 && index == 11 {
        let r42 = regex_from_rule(&rules, 42, part2);
        let r31 = regex_from_rule(&rules, 31, part2);
        return format!("({r42}{r31}|({r42}){{2}}({r31}){{2}}|({r42}){{3}}({r31}){{3}}|({r42}){{4}}({r31}){{4}}|({r42}){{5}}({r31}){{5}}|({r42}){{6}}({r31}){{6}}|({r42}){{7}}({r31}){{7}}|({r42}){{8}}({r31}){{8}}|({r42}){{9}}({r31}){{9}})", r42 = r42, r31 = r31);
    }
    let mut res = String::new();
    for v in init.split_whitespace() {
        match v {
            "|" => res.push_str("|"),
            c => res.push_str(&regex_from_rule(&rules, c.parse().unwrap(), part2)),
        }
    }
    format!("({})", res)
}

#[aoc(day19, part1)]
fn part1(input: &(String, Vec<String>)) -> usize {
    let (rule, messages) = input;
    // println!("rule {}", rule);
    let regex = Regex::new(&format!("^{}$", rule)).unwrap();
    messages
        .iter()
        .filter(|x| regex.is_match(x.as_bytes()))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &(String, Vec<String>)) -> usize {
    let (rule, messages) = input;
    // println!("rule {}", rule);
    let regex = Regex::new(&format!("^{}$", rule)).unwrap();
    messages
        .iter()
        .filter(|x| regex.is_match(x.as_bytes()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        assert_eq!(part1(&input_generator(input)), 2);
    }

    #[test]
    fn test_part2() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(part2(&input_generator2(input)), 12);
    }
}
