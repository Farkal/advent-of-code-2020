// Parser definition

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1 as digit, space0 as space},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use std::str::FromStr;

// We parse any expr surrounded by parens, ignoring all whitespaces around those
fn parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), expr, tag(")")), space)(i)
}

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
fn factor(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        parens,
    ))(i)
}

fn expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = factor(i)?;

    fold_many0(
        pair(alt((char('+'), char('*'))), factor),
        init,
        |acc, (op, val): (char, i64)| {
            if op == '+' {
                acc + val
            } else {
                acc * val
            }
        },
    )(i)
}

fn parens2(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), expr2, tag(")")), space)(i)
}
fn factor2(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        parens2,
    ))(i)
}

fn term(i: &str) -> IResult<&str, i64> {
    let (i, init) = factor2(i)?;

    fold_many0(
        pair(char('+'), factor2),
        init,
        |acc, (_, val): (char, i64)| acc + val,
    )(i)
}

fn expr2(i: &str) -> IResult<&str, i64> {
    let (i, init) = term(i)?;

    fold_many0(pair(char('*'), term), init, |acc, (_, val): (char, i64)| {
        acc * val
    })(i)
}

#[aoc(day18, part1)]
fn part1(input: &str) -> i64 {
    let mut res = 0;
    for l in input.lines() {
        let (_, r) = expr(l).unwrap();
        res += r;
    }
    res
}
#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {
    let mut res = 0;
    for l in input.lines() {
        let (_, r) = expr2(l).unwrap();
        res += r;
    }
    res
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(expr("1 + 2 * 3 + 4 * 5 + 6"), Ok(("", 71)));
        assert_eq!(
            expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 13632))
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(expr2("1 + 2 * 3 + 4 * 5 + 6"), Ok(("", 231)));
        assert_eq!(
            expr2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 23340))
        );
    }
}
