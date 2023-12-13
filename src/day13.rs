use std::iter::zip;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use ndarray::Array2;
use ndarray::ShapeBuilder;

type Input = Vec<Data>;
type InputRef = [Data];
type Data = Pattern;
type Pattern = Array2<Space>;

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
enum Space {
    Ash,
    Rock,
}


#[aoc_generator(day13)]
fn input_generator(input: &str) -> Result<Input> {
    let (input, result) = parse_input(input).map_err(|err| err.to_owned())?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

use nom::IResult;
use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::branch::alt;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = separated_list1(tag("\n\n"), parse_pattern)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let (input, row_data) = separated_list1(tag("\n"),
        many1(parse_space)
    )(input)?;

    let cols = row_data[0].len();
    let rows = row_data.len();

    let data: Vec<_> = row_data.into_iter().flatten().collect();
    let shape = (rows, cols).strides((cols, 1));

    Ok((input, Array2::from_shape_vec(shape, data).unwrap()))
}

fn parse_space(input: &str) -> IResult<&str, Space> {
    let (input, c) = alt((tag("#"), tag(".")))(input)?;

    use Space::*;
    let space = match c {
        "#" => Rock,
        "." => Ash,
        _ => unreachable!(),
    };

    Ok((input, space))
}

#[aoc(day13, part1)]
fn solve_part1(input: &InputRef) -> usize {
    let mut total = 0;

    'outer:
    for arr in input {
        // Vertical reflection
        let shape = arr.shape();
        let cols = shape[1];
        for i in 1..cols {
            if flips_vertically(arr, i) {
                total += i;
                continue 'outer;
            }
        }

        let rows = shape[0];
        for j in 1..rows {
            if flips_horizontally(arr, j) {
                total += j * 100;
                continue 'outer;
            }
        }

        unreachable!();
    }

    total
}

fn flips_vertically(arr: &Array2<Space>, i: usize) -> bool {
    let shape = arr.shape();
    let cols = shape[1];
    for (left_column, right_column) in (0..=i - 1).rev().zip(i..cols) {
        //dbg!{i, left_column, right_column};
        let left_column = arr.column(left_column);
        let right_column = arr.column(right_column);
        //dbg!{left_column, right_column};

        if left_column != right_column {
            return false;
        }
    }

    true
}

fn flips_horizontally(arr: &Array2<Space>, j: usize) -> bool {
    let shape = arr.shape();
    let rows = shape[0];

    for (up_row, down_row) in (0..=j - 1).rev().zip(j..rows) {
        let up_row = arr.row(up_row);
        let down_row = arr.row(down_row);

        if up_row != down_row {
            return false;
        }
    }

    true
}

#[aoc(day13, part2)]
fn solve_part2(input: &InputRef) -> usize {
    let mut total = 0;

    'outer:
    for arr in input {
        // Vertical reflection
        let shape = arr.shape();
        let cols = shape[1];
        for i in 1..cols {
            if flips_vertically_with_smudge(arr, i) {
                total += i;
                continue 'outer;
            }
        }

        let rows = shape[0];
        for j in 1..rows {
            if flips_horizontally_with_smudge(arr, j) {
                total += j * 100;
                continue 'outer;
            }
        }

        unreachable!();
    }

    total
}

fn flips_vertically_with_smudge(arr: &Array2<Space>, i: usize) -> bool {
    let shape = arr.shape();
    let cols = shape[1];

    let mut smudges = 0;

    for (left_column, right_column) in (0..=i - 1).rev().zip(i..cols) {
        //dbg!{i, left_column, right_column};
        let left_column = arr.column(left_column);
        let right_column = arr.column(right_column);
        //dbg!{left_column, right_column};

        smudges += zip(left_column.iter(), right_column.iter()).filter(|(l, r)| l != r).count();
        if smudges > 1 {
            return false;
        }
    }

    smudges == 1
}

fn flips_horizontally_with_smudge(arr: &Array2<Space>, j: usize) -> bool {
    let shape = arr.shape();
    let rows = shape[0];

    let mut smudges = 0;

    for (up_row, down_row) in (0..=j - 1).rev().zip(j..rows) {
        let up_row = arr.row(up_row);
        let down_row = arr.row(down_row);

        smudges += zip(up_row.iter(), down_row.iter()).filter(|(u, d)| u != d).count();

        if smudges > 1 {
            return false;
        }
    }

    smudges == 1
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        dbg!{&input};
        let result = super::solve_part1(&input);

        assert_eq!(result, 405);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 400);
    }
}
