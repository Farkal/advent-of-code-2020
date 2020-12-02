#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split('\n').map(|x| x.to_string()).collect()
}

fn password_check(input: &str) -> bool {
    let a = input.split(' ').collect::<Vec<&str>>();
    let min_max = a[0]
        .split('-')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let (min, max) = (min_max[0], min_max[1]);
    let letter = a[1].chars().next().unwrap();
    let password = a[2];
    let count = password.matches(letter).count();
    return count >= min as usize && count <= max as usize;
}

fn password_check2(input: &str) -> bool {
    let a = input.split(' ').collect::<Vec<&str>>();
    let pos = a[0]
        .split('-')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let (first, second) = (pos[0], pos[1]);
    let letter = a[1].chars().next().unwrap();
    let mut password = a[2].chars();
    let mut total = 0;
    let first_l = password.nth((first - 1) as usize);
    // println!("{:?}", first_l);
    if first_l == Some(letter) {
        total += 1;
    }
    let second_l = password.nth((second - first - 1) as usize);
    // println!("{:?}", second_l);
    if second_l == Some(letter) {
        total += 1;
    }
    return total == 1;
}

#[aoc(day2, part1)]
pub fn part1(input: &[String]) -> usize {
    input.iter().filter(|x| password_check(x)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[String]) -> usize {
    input.iter().filter(|x| password_check2(x)).count()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let l = [
            "1-3 a: abcde".into(),
            "1-3 b: cdefg".into(),
            "2-9 c: ccccccccc".into(),
        ];
        assert_eq!(part1(&l), 2);
    }
    #[test]
    fn test_part2() {
        let l = [
            "1-3 a: abcde".into(),
            "1-3 b: cdefg".into(),
            "2-9 c: ccccccccc".into(),
        ];
        assert_eq!(part2(&l), 1);
    }
}
