#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn sum_exist(value: u64, data: &[u64]) -> bool {
    // println!("{} - {:?}", value, data);
    for (i, v) in data.iter().enumerate() {
        for j in i + 1..data.len() {
            if v + data[j] == value {
                return true;
            }
        }
    }
    false
}

struct Xmas {
    data: Vec<u64>,
}

impl Xmas {
    fn run(&self, preamble_size: usize) -> u64 {
        let mut i = preamble_size;
        while i < self.data.len() {
            if !sum_exist(self.data[i], &self.data[i - preamble_size..i]) {
                return self.data[i];
            }
            i += 1;
        }
        0
    }
    fn find_contigous_sum(&self, value: u64) -> Vec<u64> {
        for (i, v) in self.data.iter().enumerate() {
            let mut sum = *v;
            for j in i + 1..self.data.len() {
                // println!("{} - {}", sum, self.data[j]);
                sum += self.data[j];
                if sum == value {
                    return self.data[i..j].to_vec();
                }
                if sum > value {
                    break;
                }
            }
        }
        vec![]
    }
}

#[aoc(day9, part1)]
fn part1(input: &[u64]) -> u64 {
    let x = Xmas {
        data: input.to_vec(),
    };
    x.run(25)
}

#[aoc(day9, part2)]
fn part2(input: &[u64]) -> u64 {
    let x = Xmas {
        data: input.to_vec(),
    };
    let res = x.find_contigous_sum(26796446);
    res.iter().min().unwrap() + res.iter().max().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let x = Xmas {
            data: input.to_vec(),
        };
        assert_eq!(x.run(5), 127);
    }
    #[test]
    fn test_part2() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let x = Xmas {
            data: input.to_vec(),
        };
        let res = x.find_contigous_sum(127);
        let r = res.iter().min().unwrap() + res.iter().max().unwrap();
        assert_eq!(r, 62);
    }
}
