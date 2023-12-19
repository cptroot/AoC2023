use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = Vec<Data>;
type InputRef = [Data];
type Data = usize;


#[aoc_generator(dayX)]
fn input_generator(input: &str) -> Result<Input> {
    unimplemented!();
    let (input, result) = parse_input(input)
        .map_err(|err| err.map(|err| anyhow!(nom::error::convert_error(input, err))))?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

type IResult<I, T> = nom::IResult<I, T, nom::error::VerboseError<I>>;
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::sequence::separated_pair;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = separated_list1(tag("\n"), parse_game)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[aoc(dayX, part1)]
fn solve_part1(input: &InputRef) -> usize {
    unimplemented!()
}

#[aoc(dayX, part2)]
fn solve_part2(input: &InputRef) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        //assert_eq!(result, None);
        assert!(false);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        //assert_eq!(result, None);
        assert!(false);
    }
}
