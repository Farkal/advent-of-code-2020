use regex::Regex;
use std::collections::HashMap;

fn check_password(password: &str, advanced: bool) -> bool {
    // println!("P: {}", password);
    let mut checking = HashMap::new();
    checking.insert("byr", false);
    checking.insert("iyr", false);
    checking.insert("eyr", false);
    checking.insert("hgt", false);
    checking.insert("hcl", false);
    checking.insert("ecl", false);
    checking.insert("pid", false);
    let hcl_regex = Regex::new("^#[a-f0-9]{6}$").unwrap();
    let ecl_regex = Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();

    for field in password.replace("\n", " ").split(" ") {
        let mut s = field.split(":");
        let key = s.next().unwrap();
        if let Some(v) = checking.get_mut(key) {
            if advanced {
                let value = s.next().unwrap();

                *v = match key {
                    "byr" => {
                        let n: u32 = value.parse().unwrap();
                        n >= 1920 && n <= 2002
                    }
                    "iyr" => {
                        let n: u32 = value.parse().unwrap();
                        n >= 2010 && n <= 2020
                    }
                    "eyr" => {
                        let n: u32 = value.parse().unwrap();
                        n >= 2020 && n <= 2030
                    }
                    "hgt" => {
                        let (n, u) = value.split_at(value.len() - 2);
                        let n: u32 = n.parse().unwrap_or(0);
                        match u {
                            "cm" => n >= 150 && n <= 193,
                            "in" => n >= 59 && n <= 76,
                            _ => false,
                        }
                    }
                    "hcl" => hcl_regex.is_match(value),
                    "ecl" => ecl_regex.is_match(value),
                    "pid" => pid_regex.is_match(value),
                    _ => false,
                }
            } else {
                *v = true
            }
        }
    }
    // println!("{:?}", checking);
    checking.values().all(|x| *x)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|x| check_password(x, false))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|x| check_password(x, true))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let invalid = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let valid = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(part2(invalid), 0);
        assert_eq!(part2(valid), 4);
    }
}
