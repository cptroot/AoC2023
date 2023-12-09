use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;

type Data = Sequence;

type Sequence = Vec<i64>;


#[aoc_generator(day9)]
fn input_generator(input: &str) -> Result<Vec<Data>> {
    let result: Vec<Vec<i64>> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line
            .split_ascii_whitespace()
            .map(|n_str| Ok(n_str.parse::<i64>()?))
            .collect::<Result<Sequence>>())
        .collect::<Result<Vec<Data>>>()?;

    Ok(result)
}

#[aoc(day9, part1)]
fn solve_part1(input: &[Data]) -> i64 {
    input.iter().map(|sequence| predict_next(sequence)).sum()
}

fn predict_next(sequence: &[i64]) -> i64 {
    let difference_sequence: Vec<_> = sequence.windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    let next_difference = if difference_sequence.iter().filter(|&&n| n != 0).next().is_none() {
        0
    } else {
        predict_next(&difference_sequence)
    };

    sequence.last().unwrap() + next_difference
}

#[aoc(day9, part2)]
fn solve_part2(input: &[Data]) -> i64 {
    input.iter().map(|sequence| predict_prev(sequence)).sum()
}

fn predict_prev(sequence: &[i64]) -> i64 {
    let difference_sequence: Vec<_> = sequence.windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    let prev_difference = if difference_sequence.iter().filter(|&&n| n != 0).next().is_none() {
        0
    } else {
        predict_prev(&difference_sequence)
    };

    sequence.first().unwrap() - prev_difference
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 114);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 2);
    }
}
