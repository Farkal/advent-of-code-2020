use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Cardinal {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Cardinal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "N" => Ok(Cardinal::North),
            "S" => Ok(Cardinal::South),
            "E" => Ok(Cardinal::East),
            "W" => Ok(Cardinal::West),
            "L" => Ok(Cardinal::Left),
            "R" => Ok(Cardinal::Right),
            "F" => Ok(Cardinal::Forward),
            _ => unreachable!(),
        }
    }
}

const DIRECTIONS: [Cardinal; 4] = [
    Cardinal::North,
    Cardinal::East,
    Cardinal::South,
    Cardinal::West,
];

struct Ship {
    x: i32,
    y: i32,
    facing_index: u32,
    waypoint: (i32, i32),
}

impl Ship {
    fn new() -> Self {
        Ship {
            x: 0,
            y: 0,
            facing_index: 1,
            waypoint: (10, 1),
        }
    }

    fn execute(&mut self, inst: &(Cardinal, i32)) {
        let (d, num) = inst;
        match d {
            Cardinal::North => self.y -= num,
            Cardinal::South => self.y += num,
            Cardinal::West => self.x -= num,
            Cardinal::East => self.x += num,
            Cardinal::Left => {
                self.facing_index = (DIRECTIONS.len() as i64
                    + (self.facing_index as i64 - (num / 90) as i64))
                    as u32
                    % DIRECTIONS.len() as u32
            }
            Cardinal::Right => {
                self.facing_index =
                    (self.facing_index + (num / 90) as u32) % DIRECTIONS.len() as u32
            }
            Cardinal::Forward => {
                let cd = DIRECTIONS[self.facing_index as usize];
                self.execute(&(cd, *num));
            }
        }
    }

    fn rotate_waypoint(&mut self, num: i32) {
        match (num / 90) % DIRECTIONS.len() as i32 {
            0 => {}
            1 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            2 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
            3 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
            _ => unreachable!(),
        }
    }

    fn execute2(&mut self, inst: &(Cardinal, i32)) {
        let (d, num) = inst;
        match d {
            Cardinal::North => self.waypoint.1 += num,
            Cardinal::South => self.waypoint.1 -= num,
            Cardinal::East => self.waypoint.0 += num,
            Cardinal::West => self.waypoint.0 -= num,
            Cardinal::Left => {
                self.facing_index = (DIRECTIONS.len() as i64
                    + (self.facing_index as i64 - (num / 90) as i64))
                    as u32
                    % DIRECTIONS.len() as u32;
                self.rotate_waypoint(360 - num);
            }
            Cardinal::Right => {
                self.facing_index =
                    (self.facing_index + (num / 90) as u32) % DIRECTIONS.len() as u32;
                self.rotate_waypoint(*num)
            }
            Cardinal::Forward => {
                self.x += num * self.waypoint.0;
                self.y += num * self.waypoint.1;
            }
        }
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<(Cardinal, i32)> {
    input
        .lines()
        .map(|x| {
            let (d, num) = x.split_at(1);
            let num: i32 = num.parse().unwrap();
            (d.parse().unwrap(), num)
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[(Cardinal, i32)]) -> i32 {
    let mut s = Ship::new();
    for i in input {
        // println!("{:?}", i);
        s.execute(i);

        // println!("{:?} {} {}", s.x, s.y, s.facing_index);
    }
    s.x.abs() + s.y.abs()
}

#[aoc(day12, part2)]
fn part2(input: &[(Cardinal, i32)]) -> i32 {
    let mut s = Ship::new();
    for i in input {
        s.execute2(i)
    }
    s.x.abs() + s.y.abs()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(part1(&input_generator(input)), 25)
    }

    #[test]
    fn test_part2() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(part2(&input_generator(input)), 286)
    }
}
