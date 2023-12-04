use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;

use regex::Regex;

type Data = String;


#[aoc_generator(day1)]
fn input_generator(input: &str) -> Result<Vec<Data>> {
    Ok(input.lines().filter(|l| !l.is_empty()).map(|l| l.to_owned()).collect())
}

#[aoc(day1, part1)]
fn solve_part1(input: &[Data]) -> u32 {
    let mut total = 0;
    for line in input {
        let first_char = line.chars().filter(|c| c.is_digit(10)).next();
        let last_char = line.chars().rev().filter(|c| c.is_digit(10)).next();

        let first_num = first_char.unwrap().to_digit(10).unwrap();
        let last_num = last_char.unwrap().to_digit(10).unwrap();

        let val = first_num * 10 + last_num;
        total += val;
    }

    total
}

#[aoc(day1, part2)]
fn solve_part2(input: &[Data]) -> u32 {
    let regex_patterns = [r"([1-9])", "(one)", "(two)", "(three)", "(four)", "(five)", "(six)", "(seven)", "(eight)", "(nine)"];
    let regexes = regex_patterns.map(|pat| Regex::new(pat).unwrap());
    let mut total = 0;

    for line in input {
        let mut first_pair = None;
        let mut last_pair = None;
        for re in &regexes {
            let mut captures = re.captures_iter(line);
            let first_capture = captures.next();
            let last_capture = captures.last();

            let first_match = first_capture.map(|capture| capture.get(0).unwrap());
            let last_match = match last_capture {
                Some(capture) => Some(capture.get(0).unwrap()),
                None => first_match,
            };

            if let Some(first_match) = first_match {
                let first_is_less = match first_pair {
                    None => true,
                    Some((start, _)) => start > first_match.start(),
                };
                if first_is_less {
                    first_pair = Some((first_match.start(), str_to_num(first_match.as_str())));
                }
            }
            if let Some(last_match) = last_match {
                let last_is_more = match last_pair {
                    None => true,
                    Some((end, _)) => end < last_match.end(),
                };
                if last_is_more {
                    last_pair = Some((last_match.end(), str_to_num(last_match.as_str())));
                }
            }
        }

        let val = first_pair.unwrap().1 * 10 + last_pair.unwrap().1;
        total += val;
    }
    total
}

fn str_to_num(num_str: &str) -> u32 {
    match num_str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("unexpected input: {}", num_str),
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT1: &'static str =
r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    const TEST_INPUT2: &'static str =
r#"
two1nine
eightwothree
eightwo
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
treb7uchet
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT1).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT2).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 281 + 77 + 82);
    }
}
