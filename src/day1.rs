#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn get_2020_sum(input: &[u32]) -> u32 {
    for (i, vi) in input.iter().enumerate() {
        for (j, vj) in input.iter().enumerate() {
            if i != j {
                if vi + vj == 2020 {
                    return vi * vj;
                }
            }
        }
    }
    0
}

fn get_2020_sum2(input: &[u32]) -> u32 {
    for (i, vi) in input.iter().enumerate() {
        for (j, vj) in input.iter().enumerate() {
            for (k, vk) in input.iter().enumerate() {
                if i != j && j != k && k != i {
                    if vi + vj + vk == 2020 {
                        return vi * vj * vk;
                    }
                }
            }
        }
    }
    0
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    get_2020_sum(input)
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    get_2020_sum2(input)
}

#[cfg(test)]
pub mod tests {
    use super::{get_2020_sum, get_2020_sum2};

    #[test]
    fn test_part1() {
        let l = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(get_2020_sum(&l), 514579);
    }

    #[test]
    fn test_part2() {
        let l = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(get_2020_sum2(&l), 241861950);
    }
}
