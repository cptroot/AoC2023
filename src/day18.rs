use std::collections::HashSet;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = Vec<Data>;
type InputRef = [Data];
type Data = Dig;

struct Dig {
    direction: Direction,
    length: usize,
    #[allow(dead_code)]
    color: Color,
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    fn hand(self, right_hand: bool) -> Direction {
        use Direction::*;
        match (self, right_hand) {
            (Up, true) |
            (Down, false) => Right,
            (Right, true) |
            (Left, false) => Down,
            (Down, true) |
            (Up, false) => Left,
            (Left, true) |
            (Right, false) => Up,
        }
    }
}

#[allow(dead_code)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}


#[aoc_generator(day18, part1)]
fn input_generator(input: &str) -> Result<Input> {
    let (input, result) = parse_input(input)
        .map_err(|err| err.map(|err| anyhow!(nom::error::convert_error(input, err))))?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

type IResult<I, T> = nom::IResult<I, T, nom::error::VerboseError<I>>;
use nom::bytes::complete::take_while1;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::sequence::tuple;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = separated_list1(tag("\n"), parse_dig)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_dig(input: &str) -> IResult<&str, Dig> {
    let (input, direction) = parse_direction(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, length) = parse_usize(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_color(input)?;

    Ok((input, Dig {
        direction,
        length,
        color,
    }))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction_c) = one_of("URDL")(input)?;

    use Direction::*;
    let direction = match direction_c {
        'U' => Up,
        'R' => Right,
        'D' => Down,
        'L' => Left,
        _ => unreachable!(),
    };

    Ok((input, direction))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("(#")(input)?;
    let (input, color_vals) = tuple((parse_hex_number, parse_hex_number, parse_hex_number))(input)?;
    let (input, _) = tag(")")(input)?;

    let color = Color {
        r: color_vals.0,
        g: color_vals.1,
        b: color_vals.2,
    };

    Ok((input, color))
}

fn parse_hex_number(input: &str) -> IResult<&str, u8> {
    let (input, num_str) = take_while_m_n(2, 2, |c: char| c.is_digit(16))(input)?;

    Ok((input, u8::from_str_radix(num_str, 16).unwrap()))
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct Pos {
    i: i32,
    j: i32,
}

impl Pos {
    fn shift(&self, direction: Direction) -> Pos {
        let mut next_pos = *self;
        use Direction::*;
        match direction {
            Up => next_pos.j -= 1,
            Right => next_pos.i += 1,
            Down => next_pos.j += 1,
            Left => next_pos.i -= 1,
        }

        next_pos
    }

    fn neighbors(&self) -> impl Iterator<Item = Pos> + '_ {
        DIRECTIONS.iter().map(move |&d| self.shift(d))
    }
}

#[aoc(day18, part1)]
fn solve_part1(input: &InputRef) -> usize {
    let winding_number = winding_number(input);

    let right_hand = winding_number > 0;

    let mut current_pos = Pos { i: 0, j: 0 };
    let mut last_direction = None;

    let mut boundary = HashSet::new();
    let mut interior = HashSet::new();
    boundary.insert(current_pos);

    for dig in input {
        if let Some(last_direction) = last_direction {
            use Direction::*;
            let add_corner = match (last_direction, dig.direction, right_hand) {
                (Up, Left, true) |
                (Up, Right, false) |
                (Right, Up, true) |
                (Right, Down, false) |
                (Down, Right, true) |
                (Down, Left, false) |
                (Left, Down, true) |
                (Left, Up, false) => true,
                _ => false,
            };

            if add_corner {
                let corner = current_pos.shift(last_direction).shift(last_direction.hand(right_hand));
                interior.insert(corner);
            }
        }

        for _ in 0..dig.length {
            current_pos = current_pos.shift(dig.direction);
            boundary.insert(current_pos);

            let hand = current_pos.shift(dig.direction.hand(right_hand));
            interior.insert(hand);
        }

        last_direction = Some(dig.direction);
    }

    let mut interior: HashSet<_> = interior.difference(&boundary).cloned().collect();

    let mut frontier: Vec<_> = interior.iter().cloned().collect();

    while let Some(p) = frontier.pop() {
        for n in p.neighbors() {
            if !interior.contains(&n) && !boundary.contains(&n) {
                frontier.push(n);
                interior.insert(n);
            }
        }
    }

    interior.len() + boundary.len()
}

fn winding_number(input: &InputRef) -> i32 {
    use Direction::*;
    input.windows(2).map(|point| {
        match (point[0].direction, point[1].direction) {
            (Up, Right) => 1,
            (Up, Up) => 0,
            (Up, Left) => -1,
            (Right, Down) => 1,
            (Right, Right) => 0,
            (Right, Up) => -1,
            (Down, Left) => 1,
            (Down, Down) => 0,
            (Down, Right) => -1,
            (Left, Up) => 1,
            (Left, Left) => 0,
            (Left, Down) => -1,
            _ => unreachable!(),
        }
    }).sum()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 62);
    }
}
