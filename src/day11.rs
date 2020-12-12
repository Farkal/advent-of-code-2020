use std::collections::HashMap;

#[derive(PartialEq, Clone)]
enum CellType {
    Floor,
    Free,
    Occupied,
}
type Grid = HashMap<(i32, i32), CellType>;

fn check_adjacent_1(input: &Grid, i: i32, j: i32) -> u32 {
    let mut nb_occupied_adjacent = 0;
    for k in -1i32..=1 {
        for l in -1i32..=1 {
            if (k != 0 || l != 0)
                && input.get(&(i + k, j + l)).unwrap_or(&CellType::Floor) == &CellType::Occupied
            {
                nb_occupied_adjacent += 1
            }
        }
    }
    nb_occupied_adjacent
}
fn check_adjacent_2(input: &Grid, i: i32, j: i32, max: (usize, usize)) -> u32 {
    let mut nb_occupied_adjacent = 0;
    // println!("i {} j {}", i, j);
    for k in -1i32..=1 {
        for l in -1i32..=1 {
            // println!("k {} l {}", k, l);
            if k != 0 || l != 0 {
                let (mut tmp_i, mut tmp_j) = (i + k, j + l);
                while tmp_i >= 0 && tmp_i <= max.0 as i32 && tmp_j >= 0 && tmp_j <= max.1 as i32 {
                    // println!("{} - {} - {:?}", tmp_i, tmp_j, max);
                    match input.get(&(tmp_i, tmp_j)).unwrap_or(&CellType::Floor) {
                        CellType::Free => break,
                        CellType::Occupied => {
                            nb_occupied_adjacent += 1;
                            break;
                        }
                        _ => {}
                    }
                    tmp_i += k;
                    tmp_j += l;
                }
            }
        }
    }
    nb_occupied_adjacent
}

fn display(input: &Grid, max: (usize, usize)) {
    for y in 0..=max.1 as i32 {
        for x in 0..=max.0 as i32 {
            let c = match input.get(&(x, y)).unwrap_or(&CellType::Floor) {
                CellType::Floor => ".",
                CellType::Free => "L",
                CellType::Occupied => "#",
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn run_round(
    input: &Grid,
    part1: bool,
    max: (usize, usize),
) -> (u32, HashMap<(i32, i32), CellType>) {
    let mut nb_change = 0;
    let mut res = input.clone();
    for ((i, j), c) in input {
        let (nb_occupied_adjacent, toleration) = if part1 {
            (check_adjacent_1(input, *i, *j), 4)
        } else {
            (check_adjacent_2(input, *i, *j, max), 5)
        };
        // println!("ADJ {}", nb_occupied_adjacent);
        match c {
            CellType::Occupied if nb_occupied_adjacent >= toleration => {
                *res.get_mut(&(*i, *j)).unwrap() = CellType::Free;
                nb_change += 1
            }
            CellType::Free if nb_occupied_adjacent == 0 => {
                *res.get_mut(&(*i, *j)).unwrap() = CellType::Occupied;
                nb_change += 1
            }
            _ => continue,
        }
    }
    // display(&res, max);
    // println!("----- CHANGES {}", nb_change);
    (nb_change, res)
}

fn run_until_no_changes(input: &str, part1: bool) -> usize {
    let mut res = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let ct = match c {
                'L' => CellType::Free,
                '#' => CellType::Occupied,
                _ => continue,
            };
            res.insert((x as i32, y as i32), ct);
            max_x = x;
            max_y = y;
        }
    }
    // display(&res, (max_x, max_y));
    loop {
        let (c, r) = run_round(&res, part1, (max_x, max_y));
        // println!("{}", c);
        if c == 0 {
            break;
        }
        res = r;
    }
    res.iter()
        .filter(|(_, v)| *v == &CellType::Occupied)
        .count()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    run_until_no_changes(input, true)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    run_until_no_changes(input, false)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part1(input), 37);
    }

    #[test]
    fn test_part2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part2(input), 26);
    }
}
