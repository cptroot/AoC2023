use std::collections::VecDeque;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use ndarray::Array2;
use ndarray::ShapeBuilder;

type Input = Data;
type InputRef = Data;
type Data = Array2<Cell>;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterType),
}

impl Cell {
    fn next_moves(&self, direction: Direction) -> impl Iterator<Item = Direction> {
        use Cell::*;
        use MirrorType::*;
        use SplitterType::*;
        use Direction::*;
        match *self {
            Empty => {
                match direction {
                    Up => [Up].iter(),
                    Right => [Right].iter(),
                    Down => [Down].iter(),
                    Left => [Left].iter(),
                }
            },
            Mirror(UpRight) => {
                match direction {
                    Up => [Right].iter(),
                    Right => [Up].iter(),
                    Down => [Left].iter(),
                    Left => [Down].iter(),
                }
            },
            Mirror(DownRight) => {
                match direction {
                    Up => [Left].iter(),
                    Left => [Up].iter(),
                    Down => [Right].iter(),
                    Right => [Down].iter(),
                }
            },
            Splitter(Vertical) => {
                match direction {
                    Up => [Up].iter(),
                    Down => [Down].iter(),
                    Right | Left => [Up, Down].iter(),
                }
            },
            Splitter(Horizontal) => {
                match direction {
                    Right => [Right].iter(),
                    Left => [Left].iter(),
                    Up | Down => [Right, Left].iter(),
                }
            },
        }.cloned()
    }
}

impl std::ops::Index<Pos> for Array2<Cell> {
    type Output = <Self as std::ops::Index<ndarray::Ix2>>::Output;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[(index.j as usize, index.i as usize)]
    }
}

#[derive(Debug, Clone, Copy)]
enum MirrorType {
    UpRight,
    DownRight,
}

#[derive(Debug, Clone, Copy)]
enum SplitterType {
    Vertical,
    Horizontal,
}


#[aoc_generator(day16)]
fn input_generator(input: &str) -> Result<Input> {
    let (input, result) = parse_input(input)
        .map_err(|err| err.map(|err| anyhow!(nom::error::convert_error(input, err))))?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

type IResult<I, T> = nom::IResult<I, T, nom::error::VerboseError<I>>;
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::multi::many1;
use nom::bytes::complete::tag;

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = parse_grid(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_grid(input: &str) -> IResult<&str, Data> {
    let (input, row_data) = separated_list1(tag("\n"),
        many1(parse_space)
    )(input)?;

    let cols = row_data[0].len();
    let rows = row_data.len();

    let data: Vec<_> = row_data.into_iter().flatten().collect();
    let shape = (rows, cols).strides((cols, 1));

    Ok((input, Array2::from_shape_vec(shape, data).unwrap()))
}

fn parse_space(input: &str) -> IResult<&str, Cell> {
    let (input, c) = one_of("./\\|-")(input)?;

    use Cell::*;
    let cell = match c {
        '.' => Empty,
        '/' => Mirror(MirrorType::UpRight),
        '\\' => Mirror(MirrorType::DownRight),
        '|' => Splitter(SplitterType::Vertical),
        '-' => Splitter(SplitterType::Horizontal),
        _ => unreachable!(),
    };

    Ok((input, cell))
}

#[derive(Debug, Clone, Copy)]
struct Energized {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

impl std::ops::Index<Pos> for Array2<Energized> {
    type Output = <Self as std::ops::Index<ndarray::Ix2>>::Output;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[(index.i as usize, index.j as usize)]
    }
}

impl std::ops::IndexMut<Pos> for Array2<Energized> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[(index.j as usize, index.i as usize)]
    }
}

impl Energized {
    fn is_energized(&self) -> bool {
        self.up || self.down ||
            self.right || self.left
    }
    #[cfg(test)]
    fn count_beams(&self) -> u32 {
        [self.up, self.down, self.right, self.left].iter().cloned().filter(|b| *b).count() as u32
    }

    fn add_direction(&mut self, d: Direction) -> bool {
        use Direction::*;
        let field = match d {
            Up => &mut self.up,
            Right => &mut self.right,
            Down => &mut self.down,
            Left => &mut self.left,
        };

        let was_energized = *field;

        *field = true;

        !was_energized
    }
}

#[cfg(test)]
fn pretty_print(state: &Array2<Energized>) {
    for row in state.rows() {
        for s in row.iter() {
            let c = match s {
                Energized { up: false, down: false, right: false, left: false } => '.',
                Energized { up: true, down: false, right: false, left: false } => '^',
                Energized { up: false, down: false, right: true, left: false } => '>',
                Energized { up: false, down: true, right: false, left: false } => 'v',
                Energized { up: false, down: false, right: false, left: true } => '<',
                _ => char::from_digit(s.count_beams(), 10).unwrap(),
            };

            print!("{}", c);
        }
        println!();
    }
}

impl Default for Energized {
    fn default() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    i: i32,
    j: i32,
}

impl Pos {
    fn try_move(&self, direction: Direction, shape: &[usize]) -> Option<Pos> {
        let mut next_pos = *self;
        use Direction::*;
        match direction {
            Up => next_pos.j -= 1,
            Right => next_pos.i += 1,
            Down => next_pos.j += 1,
            Left => next_pos.i -= 1,
        }

        if next_pos.i >= 0 && next_pos.j >= 0 &&
                next_pos.i < shape[0] as i32 && next_pos.j < shape[1] as i32 {
            Some(next_pos)
        } else {
            None
        }
    }
}

#[aoc(day16, part1)]
fn solve_part1(input: &InputRef) -> usize {
    let mut state = Array2::from_elem(input.raw_dim(), Energized::default());

    let mut frontier = VecDeque::new();

    let start_pos = Pos { i: -1, j: 0 };
    let start_direction = Direction::Right;

    frontier.push_back((start_pos, start_direction));

    while let Some((pos, direction)) = frontier.pop_front() {
        let next_pos = pos.try_move(direction, input.shape());
        if let Some(next_pos) = next_pos {
            let next_cell = input[next_pos];
            for new_direction in next_cell.next_moves(direction) {
                if state[next_pos].add_direction(new_direction) {
                    frontier.push_back((next_pos, new_direction));
                }
            }
        }
    }

    #[cfg(test)]
    pretty_print(&state);

    state.iter().filter(|s| s.is_energized()).count()
}

fn count_energized(input: &InputRef, start_pos: Pos, start_direction: Direction) -> usize {
    let mut state = Array2::from_elem(input.raw_dim(), Energized::default());

    let mut frontier = VecDeque::new();

    frontier.push_back((start_pos, start_direction));

    while let Some((pos, direction)) = frontier.pop_front() {
        let next_pos = pos.try_move(direction, input.shape());
        if let Some(next_pos) = next_pos {
            let next_cell = input[next_pos];
            for new_direction in next_cell.next_moves(direction) {
                if state[next_pos].add_direction(new_direction) {
                    frontier.push_back((next_pos, new_direction));
                }
            }
        }
    }

    state.iter().filter(|s| s.is_energized()).count()
}

#[aoc(day16, part2)]
fn solve_part2(input: &InputRef) -> usize {
    let shape = input.shape();
    let rows = shape[0] as i32;
    let cols = shape[1] as i32;

    use Direction::*;
    let start_iter = (0..rows).map(|j| (Pos { i: -1, j, }, Right))
        .chain((0..cols).map(|i| (Pos { i, j: -1, }, Down)))
        .chain((0..rows).map(|j| (Pos { i: cols, j, }, Left)))
        .chain((0..cols).map(|i| (Pos { i, j: rows, }, Up)));

    start_iter.map(|(p, d)| count_energized(input, p, d))
        .max().unwrap()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 46);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 51);
    }
}
