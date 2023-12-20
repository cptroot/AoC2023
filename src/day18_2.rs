use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::once;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = Vec<Data>;
type InputRef = [Data];
type Data = Dig;

struct Dig {
    direction: Direction,
    length: u64,
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

#[aoc_generator(day18, part2)]
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
    let (input, _) = parse_direction(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, _) = parse_usize(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, (length, direction)) = parse_color(input)?;

    Ok((input, Dig {
        direction,
        length,
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

fn parse_color(input: &str) -> IResult<&str, (u64, Direction)> {
    let (input, _) = tag("(#")(input)?;
    let (input, vals) = tuple((parse_hex_number(5), parse_hex_number(1)))(input)?;
    let (input, _) = tag(")")(input)?;

    let length = vals.0;
    use Direction::*;
    let direction = match vals.1 {
        0 => Right,
        1 => Down,
        2 => Left,
        3 => Up,
        _ => unreachable!(),
    };

    Ok((input, (length, direction)))
}

fn parse_hex_number(width: usize) -> impl Fn(&str) -> IResult<&str, u64> {
    move |input: &str| {
        let (input, num_str) = take_while_m_n(width, width, |c: char| c.is_digit(16))(input)?;

        Ok((input, u64::from_str_radix(num_str, 16).unwrap()))
    }
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
        self.shift_multiple(direction, 1)
    }
    fn shift_multiple(&self, direction: Direction, length: i32) -> Pos {
        let mut next_pos = *self;
        use Direction::*;
        match direction {
            Up => next_pos.j -= length,
            Right => next_pos.i += length,
            Down => next_pos.j += length,
            Left => next_pos.i -= length,
        }

        next_pos
    }

    fn neighbors(&self) -> impl Iterator<Item = Pos> + '_ {
        DIRECTIONS.iter().map(move |&d| self.shift(d))
    }
}

#[aoc(day18, part2)]
fn solve_part2(input: &InputRef) -> i64 {
    let winding_number = winding_number(input);

    let right_hand = winding_number > 0;

    let mut current_pos = Pos { i: 0, j: 0 };

    let mut path = Vec::new();

    for dig in input {
        current_pos = current_pos.shift_multiple(dig.direction, dig.length.try_into().unwrap());
        path.push((current_pos, dig.direction));
    }

    assert!(right_hand);

    let mut boundary_is = HashSet::new();
    let mut boundary_js = HashSet::new();

    let boundary_positions: Vec<_> = path.windows(2).chain(once([*path.last().unwrap(), *path.first().unwrap()].as_slice()))
        .map(|pair| {
            let boundary_pos = pair[0].0;
            let i = boundary_pos.i;
            let j = boundary_pos.j;

            boundary_is.insert(i);
            boundary_js.insert(j);

            use Direction::*;
            let (i_diff, j_diff, direction) = match (pair[0].1, pair[1].1) {
                (Up, Right) => (-1, -1, Up),
                (Up, Left) => (-1, 1, Up),
                (Right, Down) => (1, -1, Right),
                (Right, Up) => (-1, -1, Right),
                (Down, Left) => (1, 1, Down),
                (Down, Right) => (1, -1, Down),
                (Left, Up) => (-1, 1, Left),
                (Left, Down) => (1, 1, Left),
                _ => unreachable!(),
            };

            (Pos { i: i + i_diff, j: j + j_diff }, direction)
        })
        .collect();

    boundary_is.extend(boundary_positions.iter().map(|(p, _d)| p.i));
    boundary_js.extend(boundary_positions.iter().map(|(p, _d)| p.j));

    let boundary_is: Vec<_> = {
        let mut tmp: Vec<_> = boundary_is.into_iter().collect();
        tmp.sort();
        tmp
    };
    let boundary_js: Vec<_> = {
        let mut tmp: Vec<_> = boundary_js.into_iter().collect();
        tmp.sort();
        tmp
    };

    //dbg!{&boundary_is, &boundary_js};

    let boundary_i_map: HashMap<_, _> = boundary_is.iter().enumerate().map(|(index, i)| (i, index)).collect();
    let boundary_j_map: HashMap<_, _> = boundary_js.iter().enumerate().map(|(index, j)| (j, index)).collect();

    let boundary_positions: Vec<_> = boundary_positions.into_iter().map(|(p, d)| (Pos { i: boundary_i_map[&p.i].try_into().unwrap(), j: boundary_j_map[&p.j].try_into().unwrap() }, d)).collect();
    let boundary_vec: Vec<_> = boundary_positions.windows(2).chain(once([*boundary_positions.last().unwrap(), *boundary_positions.first().unwrap()].as_slice()))
        .flat_map(|pair| {
            //dbg!{pair[0].0, pair[1].0, pair[1].1};
            (0..).map(|n| {
                (pair[0].0.shift_multiple(pair[1].1, n), pair[1].1)
            })
            .take_while(|&(p, _d)| p != pair[1].0)
        })
        .collect();
    let boundary: HashSet<_> = boundary_vec.iter().map(|&(p, _d)| p).collect();

    let mut interior: HashSet<_> = HashSet::new();

    interior.insert(Pos{i: boundary_vec.last().unwrap().0.i + 1, j: boundary_vec.last().unwrap().0.i + 1});


    /*
    let num_rows = boundary_js.len();
    let num_cols = boundary_is.len();
    let mut grid = vec![vec!['.'; num_cols]; num_rows];

    for (p, d) in &boundary_vec {
        grid[p.j as usize][p.i as usize] = '#';
    }
    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    */



    let mut frontier: Vec<_> = interior.iter().cloned().collect();

    while let Some(p) = frontier.pop() {
        for n in p.neighbors() {
            if !interior.contains(&n) && !boundary.contains(&n) {
                frontier.push(n);
                interior.insert(n);
            }
        }
    }

    let mut total = 0;

    for p in interior {
        //dbg!{p};
        let start_i = boundary_is[p.i as usize];
        let start_j = boundary_js[p.j as usize];
        let end_i = boundary_is[p.i as usize + 1];
        let end_j = boundary_js[p.j as usize + 1];
        //dbg!{end_i - start_i, end_j - start_j};

        total += (end_i - start_i) as i64 * (end_j - start_j) as i64;
    }

    total
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
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 952408144115);
    }
}
