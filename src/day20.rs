#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    border: Vec<(String, Option<usize>)>
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<Tile> {
    let mut res = vec![];
    for t in input.split("\n\n") {
        let mut  lines = t.lines();
        let id = lines.next().unwrap().split_whitespace().nth(1).unwrap().trim_end_matches(":").parse().unwrap();
        let border_top = lines.next().unwrap();
        let border_bottom =lines.last().unwrap();
        let border_left = t.lines().skip(1).map(|l| l.chars().next().unwrap()).fold(String::new(), |acc, v| acc + &v.to_string());
        let border_right = t.lines().skip(1).map(|l| l.chars().last().unwrap()).fold(String::new(), |acc, v| acc + &v.to_string());
        res.push(Tile {
            id,
            border: vec![(border_top.to_string(), None), (border_right, None), (border_bottom.to_string(), None), (border_left, None)]
        })
    }
    res
}

fn found_matching(tiles: &mut Vec<Tile>) {
    let o_tiles = tiles.clone();
    for t in tiles.iter_mut() {
        for b in t.border.iter_mut() {
            if b.1 == None {
                for t2 in o_tiles.iter() {
                    if t.id != t2.id {
                        for b2 in t2.border.iter() {
                            // println!("{:?} vs {:?}", b, b2);
                            if b.0 == b2.0 || b.0.chars().rev().collect::<String>() == b2.0 {
                                // println!(" MATCH {:?} vs {:?}", b, b2);
                                *b = (b.0.clone(), Some(t2.id))
                            }
                        }
                    }
                }
            }
        }
    }
}


#[aoc(day20, part1)]
fn part1(input: &Vec<Tile>) -> usize {
    let mut tiles = input.clone();
    found_matching(&mut tiles);
    // println!("{:#?}", tiles);
    tiles.iter().filter(|x| x.border.iter().filter(|b| b.1 == None).count() == 2).map(|x| x.id).product()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(part1(&input_generator(input)), 20899048083289)
    }
}


