use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
type Data = (u64, u64);


#[aoc_generator(day6)]
fn input_generator(input: &str) -> Result<(Vec<Data>, Data)> {
    let mut iter = input.lines().filter(|l| !l.is_empty());

    let times = iter.next().unwrap().split_ascii_whitespace().skip(1);
    let distances = iter.next().unwrap().split_ascii_whitespace().skip(1);

    let pairs: Vec<_> = times.zip(distances).map(|(time, distance)| {
        let time: u64 = time.parse()?;
        let distance: u64 = distance.parse()?;

        Ok((time, distance))
    })
    .collect::<Result<Vec<_>>>()?;

    let mut iter = input.lines().filter(|l| !l.is_empty());

    let time = iter.next().unwrap().split_ascii_whitespace().skip(1).fold(String::new(), |mut l, r| { l += r; l }).parse()?;
    let distance = iter.next().unwrap().split_ascii_whitespace().skip(1).fold(String::new(), |mut l, r| { l += r; l }).parse()?;

    Ok((pairs, (time, distance)))
}

#[aoc(day6, part1)]
fn solve_part1(input: &(Vec<Data>, Data)) -> u32 {
    // (t - n) * n - d > 0
    // -n^2 + tn - d > 0

    let mut ways = 1;

    for &(time, distance) in &input.0 {
        ways *= ways_to_win_the_race(time, distance);
    }

    ways
}

fn ways_to_win_the_race(time: u64, distance: u64) -> u32 {
    let time: f64 = time as f64;
    let distance: f64 = distance as f64;

    let a = -1.0;
    let b = time;
    let c = -distance;

    let positive_zero = (-b - (b*b - 4.0 * a * c).sqrt()) / 2.0 / a;
    let negative_zero = (-b + (b*b - 4.0 * a * c).sqrt()) / 2.0 / a;

    let lowest_solution: u32 = negative_zero.floor() as u32 + 1;
    let highest_solution: u32 = positive_zero.ceil() as u32 - 1;

    if highest_solution >= lowest_solution {
        highest_solution - lowest_solution + 1
    } else {
        0
    }
}

#[aoc(day6, part2)]
fn solve_part2(input: &(Vec<Data>, Data)) -> u32 {
    ways_to_win_the_race(input.1.0, input.1.1)
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
Time:      7  15   30
Distance:  9  40  200
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 71503);
    }
}
