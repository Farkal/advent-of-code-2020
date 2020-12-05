fn get_id(input: &str) -> u32 {
    let (s_row, s_col) = input.split_at(7);
    let (mut min_row, mut max_row, mut min_col, mut max_col) = (0, 127, 0, 7);
    for c in s_row.chars() {
        let nb_row = max_row - min_row + 1;
        match c {
            'F' => max_row -= (nb_row / 2) as u32,
            'B' => min_row += (nb_row / 2) as u32,
            _ => todo!(),
        }
    }
    for c in s_col.chars() {
        let nb_col = max_col - min_col + 1;
        match c {
            'L' => max_col -= (nb_col / 2) as u32,
            'R' => min_col += (nb_col / 2) as u32,
            _ => todo!(),
        }
    }
    // println!("CC {} - {}", min_col, max_col);

    min_row * 8 + min_col
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    input.lines().map(|x| get_id(x)).max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    let mut ids: Vec<u32> = input.lines().map(|x| get_id(x)).collect();
    ids.sort();
    if let Some((first, elements)) = ids.split_first() {
        let mut val = first.clone();
        for i in elements {
            val += 1;
            if i != &val {
                return val;
            }
        }
    }
    0
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_id("FBFBBFFRLR"), 357);
        assert_eq!(get_id("BFFFBBFRRR"), 567);
        assert_eq!(get_id("FFFBBBFRRR"), 119);
        assert_eq!(get_id("BBFFBBFRLL"), 820);
    }
}
