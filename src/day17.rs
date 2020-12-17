use std::collections::HashMap;
type Grid = HashMap<(i32, i32, i32), bool>;
type Grid4 = HashMap<(i32, i32, i32, i32), bool>;

#[aoc_generator(day17, part1)]
fn input_generator(input: &str) -> Grid {
    let mut grid: Grid = HashMap::new();
    for x in -20..=20 {
        for y in -20..=20 {
            for z in -10..=10 {
                grid.insert((x, y, z), false);
            }
        }
    }
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.insert((x as i32, y as i32, 0), c == '#');
        }
    }
    grid
}

#[aoc_generator(day17, part2)]
fn input_generator2(input: &str) -> Grid4 {
    let mut grid = HashMap::new();
    for x in -20..=20 {
        for y in -20..=20 {
            for z in -10..=10 {
                for w in -10..=10 {
                    grid.insert((x, y, z, w), false);
                }
            }
        }
    }
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.insert((x as i32, y as i32, 0, 0), c == '#');
        }
    }
    grid
}

fn get_nb_neighbor_active(grid: &Grid, pos: &(i32, i32, i32)) -> u32 {
    let mut count = 0;
    for x in pos.0 - 1..=pos.0 + 1 {
        for y in pos.1 - 1..=pos.1 + 1 {
            for z in pos.2 - 1..=pos.2 + 1 {
                if &(x, y, z) != pos && grid.get(&(x, y, z)).unwrap_or(&false) == &true {
                    count += 1;
                }
            }
        }
    }
    count
}

fn run_round(grid: &mut Grid) {
    let initial_grid = grid.clone();

    for (c_pos, c_active) in grid.iter_mut() {
        let nb_active = get_nb_neighbor_active(&initial_grid, c_pos);
        // print!("{:?} - {} - {}", c_pos, c_active, nb_active);
        if *c_active && (nb_active < 2 || nb_active > 3) {
            *c_active = false
        }
        if !*c_active && nb_active == 3 {
            *c_active = true
        }
        // println!(" -> {}", c_active);
    }
}
fn run_round2(grid: &mut Grid4) {
    let initial_grid = grid.clone();

    for (c_pos, c_active) in grid.iter_mut() {
        let nb_active = get_nb_neighbor_active2(&initial_grid, c_pos);
        // print!("{:?} - {} - {}", c_pos, c_active, nb_active);
        if *c_active && (nb_active < 2 || nb_active > 3) {
            *c_active = false
        }
        if !*c_active && nb_active == 3 {
            *c_active = true
        }
        // println!(" -> {}", c_active);
    }
}

fn get_nb_neighbor_active2(grid: &Grid4, pos: &(i32, i32, i32, i32)) -> u32 {
    let mut count = 0;
    for x in pos.0 - 1..=pos.0 + 1 {
        for y in pos.1 - 1..=pos.1 + 1 {
            for z in pos.2 - 1..=pos.2 + 1 {
                for w in pos.3 - 1..=pos.3 + 1 {
                    if &(x, y, z, w) != pos && grid.get(&(x, y, z, w)).unwrap_or(&false) == &true {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[aoc(day17, part1)]
fn part1(input: &Grid) -> usize {
    let mut input = input.clone();
    // println!("{:?}", input);
    for _ in 0..6 {
        run_round(&mut input)
    }
    input.iter().filter(|(_, a)| **a).count()
}
#[aoc(day17, part2)]
fn part2(input: &Grid4) -> usize {
    let mut input = input.clone();
    // println!("{:?}", input);
    for _ in 0..6 {
        run_round2(&mut input)
    }
    input.iter().filter(|(_, a)| **a).count()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#.
..#
###";
        assert_eq!(part1(&input_generator(input)), 112)
    }
    #[test]
    fn test_part2() {
        let input = ".#.
..#
###";
        assert_eq!(part2(&input_generator2(input)), 848)
    }
}
