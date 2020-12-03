use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileType {
    NONE,
    TREE,
}
#[derive(Debug)]
pub struct Tile {
    t_type: TileType,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        let t_type = match c {
            '.' => TileType::NONE,
            '#' => TileType::TREE,
            _ => todo!(),
        };
        Tile { t_type }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> HashMap<(usize, usize), Tile> {
    let mut res = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            res.insert((x, y), c.into());
        }
    }
    res
}

fn get_number_of_tree(
    map: &HashMap<(usize, usize), Tile>,
    increase_x: usize,
    increase_y: usize,
) -> u32 {
    let (mut x, mut y) = (increase_x, increase_y);
    let mut count_tree = 0;
    let max_x = map.keys().map(|v| v.0).max().unwrap();
    let max_y = map.keys().map(|v| v.1).max().unwrap();
    while y <= max_y {
        if x > max_x {
            x = x - max_x - 1;
        }
        let t = map.get(&(x, y)).unwrap().t_type;
        // println!("{} {} T: {:?}", x, y, t);
        if t == TileType::TREE {
            count_tree += 1;
        }
        x += increase_x;
        y += increase_y;
    }
    count_tree
}

#[aoc(day3, part1)]
pub fn part1(input: &HashMap<(usize, usize), Tile>) -> u32 {
    get_number_of_tree(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &HashMap<(usize, usize), Tile>) -> u64 {
    let mut res = vec![];
    res.push(get_number_of_tree(input, 1, 1));
    res.push(get_number_of_tree(input, 3, 1));
    res.push(get_number_of_tree(input, 5, 1));
    res.push(get_number_of_tree(input, 7, 1));
    res.push(get_number_of_tree(input, 1, 2));
    println!("R: {:?}", res);
    res.iter().fold(1, |acc, x| acc * *x as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(get_number_of_tree(&input_generator(input), 3, 1), 7);
    }

    #[test]
    fn test_part2() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(part2(&input_generator(input)), 336);
    }
}
