// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u32 {
    let mut tmp_i = input.lines();
    let time: u32 = tmp_i.next().unwrap().parse().unwrap();
    let boats: Vec<u32> = tmp_i
        .next()
        .unwrap()
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    boats.iter().map(|x| x - time % x).min();
    let mut found = (time.clone(), 0);
    for b in boats {
        let diff = b - time % b;
        if diff < found.0 {
            found = (diff, b)
        }
    }
    found.0 * found.1
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let mut tmp_i = input.lines();
    tmp_i.next().unwrap();
    let boats: Vec<(usize, i64)> = tmp_i
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .map(|(i, x)| (i, x.parse::<i64>().unwrap()))
        .collect();
    let mods: Vec<i64> = boats.iter().map(|(_, x)| *x).collect();
    let res: Vec<i64> = boats.iter().map(|(i, x)| x - *i as i64).collect();
    chinese_remainder(&res, &mods).unwrap()
    // let biggest = boats.iter().max_by_key(|x| x.1).unwrap();
    // println!("biggest {:?}", biggest);
    // let mut time : u64 = biggest.1;
    // loop {
    // println!("{}", time);
    // let mut found = true;
    // for b in &boats {
    // println!("{} {:?} big {} = {}", time, b, biggest.0 as u64, (b.1 - (time - biggest.0 as u64) % b.1) % b.1 );
    // if (b.1 - (time - biggest.0 as u64) % b.1) % b.1 != b.0 as u64 {
    // found = false;
    // break;
    // }
    // }
    // if found {
    // return time - biggest.0 as u64
    // } else {
    // time += biggest.1;
    // }
    // }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "939
7,13,x,x,59,x,31,19";
        assert_eq!(part1(input), 295)
    }

    #[test]
    fn test_part2() {
        let input = "939
7,13,x,x,59,x,31,19";
        assert_eq!(part2(input), 1068781);
        let input = "939
17,x,13,19";
        assert_eq!(part2(input), 3417);
        let input = "939
67,7,59,61";
        assert_eq!(part2(input), 754018)
    }
}
