use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use std::cmp::max;

type Data = Game;

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    moves: Vec<Move>,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    r: u32,
    g: u32,
    b: u32,
}


#[aoc_generator(day2)]
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
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::sequence::separated_pair;

fn parse_input(input: &str) -> IResult<&str, Vec<Data>> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = separated_list1(tag("\n"), parse_game)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = parse_u32(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, moves) = separated_list1(tag("; "), parse_move)(input)?;

    Ok((input, Game {
        id,
        moves,
    }))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, numbers) = separated_list1(tag(", "), separated_pair(parse_u32, tag(" "), parse_color))(input)?;

    let mut single_move = Move {
        r: 0, g: 0, b: 0
    };

    use Color::*;
    for (num, color) in numbers {
        match color {
            Red => single_move.r = num,
            Green => single_move.g = num,
            Blue => single_move.b = num,
        }
    }

    Ok((input, single_move))
}

enum Color {
    Red,
    Green,
    Blue,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((tag("red"), tag("blue"), tag("green")))(input)?;

    use Color::*;

    let color = match color {
        "red" => Red,
        "green" => Green,
        "blue" => Blue,
        _ => panic!("Impossible input"),
    };

    Ok((input, color))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Data]) -> u32 {
    let mut total = 0;

    let allowed_cubes = Move {
        r: 12,
        g: 13,
        b: 14,
    };

    for game in input {
        if is_game_possible(&allowed_cubes, game) {
            total += game.id;
        }
    }

    total
}

fn is_game_possible(bag: &Move, game: &Game) -> bool {
    for single_move in &game.moves {
        let impossible =
            bag.r < single_move.r ||
            bag.g < single_move.g ||
            bag.b < single_move.b;
        
        if impossible {
            return false;
        }
    }

    true
}

#[aoc(day2, part2)]
fn solve_part2(input: &[Data]) -> u32 {
    let mut total = 0;
    for game in input {
        let minimum_bag = minimum_bag(game);

        total += minimum_bag.r * minimum_bag.g * minimum_bag.b;
    }

    total
}

fn minimum_bag(game: &Game) -> Move {
    let mut bag = Move {
        r: 0,
        g: 0,
        b: 0,
    };

    for single_move in &game.moves {
        bag.r = max(bag.r, single_move.r);
        bag.g = max(bag.g, single_move.g);
        bag.b = max(bag.b, single_move.b);
    }

    bag
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 2286);
    }
}
