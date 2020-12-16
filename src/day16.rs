#[aoc_generator(day16)]
fn input_generator(input: &str) -> (Vec<(u32, u32)>, Vec<u32>, Vec<Vec<u32>>) {
    let mut blocks = input.split("\n\n");
    let rules = blocks.next().unwrap();
    let my_ticket = blocks
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let nearby_tickets = blocks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();
    let mut res_rules = vec![];
    for l in rules.lines() {
        let t = l.split(": ").nth(1);
        for d in t.unwrap().split(" or ") {
            let mut v = d.split("-");
            res_rules.push((
                v.next().unwrap().parse().unwrap(),
                v.next().unwrap().parse().unwrap(),
            ));
        }
    }
    (res_rules, my_ticket, nearby_tickets)
}

fn is_valid(rules: &Vec<(u32, u32)>, ticket: &Vec<u32>) -> bool {
    for v in ticket {
        if rules.iter().all(|(min, max)| v < min || v > max) {
            return false;
        }
    }
    true
}

fn find_error_rate(rules: &Vec<(u32, u32)>, nearby_tickets: &Vec<Vec<u32>>) -> u32 {
    let mut error_rate = 0;
    for t in nearby_tickets {
        for v in t {
            if rules.iter().all(|(min, max)| v < min || v > max) {
                error_rate += v
            }
        }
    }
    error_rate
}

#[aoc(day16, part1)]
fn part1(input: &(Vec<(u32, u32)>, Vec<u32>, Vec<Vec<u32>>)) -> u32 {
    find_error_rate(&input.0, &input.2)
}

#[aoc(day16, part2)]
fn part2(input: &(Vec<(u32, u32)>, Vec<u32>, Vec<Vec<u32>>)) -> u64 {
    let t_valid: Vec<Vec<u32>> = input
        .2
        .clone()
        .into_iter()
        .filter(|x| is_valid(&input.0, x))
        .collect();
    let mut ids = vec![];
    for r in input.0.chunks(2) {
        // println!("{:?}", r);
        let mut possible_rules = vec![];
        for id in 0..input.1.len() {
            if t_valid.iter().all(|t| {
                (r[0].0 <= t[id] && t[id] <= r[0].1) || (r[1].0 <= t[id] && t[id] <= r[1].1)
            }) {
                possible_rules.push(id);
            }
        }
        ids.push(possible_rules)
    }
    // println!("{:?}", ids);
    let mut final_rules = [0; 20];
    while let Some(i) = ids.iter().position(|p| p.len() == 1) {
        let v = ids[i][0];
        final_rules[i] = v;
        for s in &mut ids {
            s.retain(|x| x != &v);
        }
    }

    let res: Vec<u64> = final_rules[..6]
        .iter()
        .map(|x| input.1[*x] as u64)
        .collect();

    // println!("{:?}", res);
    res.iter().product()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(part1(&input_generator(input)), 71);
    }

    #[test]
    fn test_part2() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        // assert_eq!(part2(&input_generator(input)), 1716)
    }
}
