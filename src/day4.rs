use std::collections::HashSet;
use std::collections::VecDeque;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Data = Card;

#[derive(Debug, Clone)]
struct Card {
    winners: Vec<usize>,
    haves: Vec<usize>,
}


#[aoc_generator(day4)]
fn input_generator(input: &str) -> Result<Vec<Data>> {
    let (input, result) = parse_input(input).map_err(|err| err.to_owned())?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

use nom::IResult;
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::multi::many1;
use nom::bytes::complete::tag;

fn parse_input(input: &str) -> IResult<&str, Vec<Data>> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = separated_list1(tag("\n"), parse_card)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, _id) = parse_u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = many1(tag(" "))(input)?;

    let (input, winners) = separated_list1(many1(tag(" ")), parse_number)(input)?;
    let (input, _) = tag(" |")(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, haves) = separated_list1(many1(tag(" ")), parse_number)(input)?;

    Ok((input, Card {
        winners,
        haves,
    }))
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Data]) -> usize {
    let mut total = 0;

    for card in input {
        let haves: HashSet<_> = card.haves.iter().map(|&n| n).collect();

        let mut card_value = None;

        for winning in &card.winners {
            if haves.contains(&winning) {
                card_value = Some(match card_value {
                    None => 1,
                    Some(card_value) => card_value * 2,
                });
            }
        }

        if let Some(card_value) = card_value {
            total += card_value;
        }
    }

    total
}

fn count_matches(card: &Card) -> usize {
    let haves: HashSet<_> = card.haves.iter().map(|&n| n).collect();

    card.winners.iter().filter(|w| haves.contains(w)).count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Data]) -> usize {
    let mut total = 0;

    let mut extra_copies = VecDeque::new();

    for card in input {
        let copies_of_this_card = extra_copies.pop_front().unwrap_or(0) + 1;

        total += copies_of_this_card;

        let matches = count_matches(card);

        if extra_copies.len() < matches {
            extra_copies.resize(matches, 0);
        }

        for i in 0..matches {
            extra_copies[i] += copies_of_this_card;
        }
    }

    total
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        dbg!{&input};
        let result = super::solve_part1(&input);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 30);
    }
}
