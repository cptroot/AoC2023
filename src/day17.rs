use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use ndarray::Array2;
use ndarray::Array3;
use ndarray::ShapeBuilder;

type Input = Data;
type InputRef = Data;
type Data = Array2<u32>;


#[aoc_generator(day17)]
pub (crate) fn input_generator(input: &str) -> Result<Input> {
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
        many1(parse_digit)
    )(input)?;

    let cols = row_data[0].len();
    let rows = row_data.len();

    let data: Vec<_> = row_data.into_iter().flatten().collect();
    let shape = (rows, cols).strides((cols, 1));

    Ok((input, Array2::from_shape_vec(shape, data).unwrap()))
}

fn parse_digit(input: &str) -> IResult<&str, u32> {
    let (input, number_string) = one_of("0123456789")(input)?;
    let number = number_string.to_digit(10).unwrap();

    Ok((input, number))
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn invert(self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn z_index(direction: Direction, consecutive_steps: usize) -> usize {
    use Direction::*;
    let set = match direction {
        Up => 0,
        Right => 1,
        Down => 2,
        Left => 3,
    };

    set * 3 + (consecutive_steps - 1)
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
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

    fn neighbors<'a>(&'a self, direction: Direction, shape: &'a [usize]) -> impl Iterator<Item = (Pos, Direction)> + 'a  {
        DIRECTIONS.iter()
            .cloned()
            .filter(move |&d| d != direction.invert())
            .filter_map(|d| self.try_move(d, shape).map(|p| (p, d)))
    }
}

impl std::ops::Index<Pos> for Array2<u32> {
    type Output = <Self as std::ops::Index<ndarray::Ix2>>::Output;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[(index.j as usize, index.i as usize)]
    }
}

impl std::ops::Index<Pos3> for Array3<Option<u32>> {
    type Output = <Self as std::ops::Index<ndarray::Ix3>>::Output;

    fn index(&self, index: Pos3) -> &Self::Output {
        &self[(index.pos.j as usize, index.pos.i as usize, index.z)]
    }
}

impl std::ops::IndexMut<Pos3> for Array3<Option<u32>> {
    fn index_mut(&mut self, index: Pos3) -> &mut Self::Output {
        &mut self[(index.pos.j as usize, index.pos.i as usize, index.z)]
    }
}

impl std::ops::Index<Pos3> for Array3<Option<(u32, Pos3)>> {
    type Output = <Self as std::ops::Index<ndarray::Ix3>>::Output;

    fn index(&self, index: Pos3) -> &Self::Output {
        &self[(index.pos.j as usize, index.pos.i as usize, index.z)]
    }
}

impl std::ops::IndexMut<Pos3> for Array3<Option<(u32, Pos3)>> {
    fn index_mut(&mut self, index: Pos3) -> &mut Self::Output {
        &mut self[(index.pos.j as usize, index.pos.i as usize, index.z)]
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos3 {
    pos: Pos,
    z: usize,
}

#[aoc(day17, part1)]
fn solve_part1(input: &InputRef) -> u32 {
    let mut state: Array3<Option<(u32, Pos3)>> = Array3::from_elem([input.shape()[0], input.shape()[1], 12], None);

    let start_pos = Pos { i: 0, j: 0 };
    let shape = input.shape();
    let rows = shape[0];
    let cols = shape[1];
    let end_pos = Pos { i: cols as i32 - 1, j: rows as i32 - 1 };

    let mut frontier = Vec::new();
    // cur_pos, dir, consecutive, g, f
    frontier.push((start_pos, Direction::Left, 1, 0, h(start_pos, shape)));
    let index = Pos3 { pos: start_pos, z: z_index(Direction::Left, 1) };
    state[index] = Some((h(start_pos, shape), index));

    while !frontier.is_empty() {
        frontier.sort_by(|a, b| a.4.cmp(&b.4));
        let last = frontier.len() - 1;
        frontier.swap(0, last);
        let (pos, direction, consecutive_steps, g, _f) = frontier.pop().unwrap();

        //dbg!{pos, direction, consecutive_steps, g};
        for neighbor in pos.neighbors(direction, shape) {
            let new_steps = if neighbor.1 == direction {
                consecutive_steps + 1
            } else {
                1
            };
            if new_steps >= 4 { continue; }

            let cost = input[neighbor.0];

            let new_g = g + cost;
            let new_f = new_g + h(neighbor.0, shape);

            if neighbor.0 == end_pos {
                let end_pos = Pos3 { pos: neighbor.0, z: z_index(neighbor.1, new_steps)};
                state[end_pos] = Some((new_f, Pos3 { pos, z: z_index(direction, consecutive_steps) }));
                pretty_print(input, &state, end_pos);
                return new_g;
            }

            let index = Pos3 { pos: neighbor.0, z: z_index(neighbor.1, new_steps)};
            let previous_f: Option<(u32, Pos3)> = state[index];

            if previous_f.is_none() || previous_f.unwrap().0 > new_f {
                state[index] = Some((new_f, Pos3 { pos, z: z_index(direction, consecutive_steps) }));
                frontier.push((neighbor.0, neighbor.1, new_steps, new_g, new_f));
            }
        }
    }
    
    unreachable!()
}

fn h(pos: Pos, shape: &[usize]) -> u32 {
    ((shape[0] + shape[1]) as i32 - (pos.j + pos.i)) as u32
}

fn pretty_print(input: &Array2<u32>, state: &Array3<Option<(u32, Pos3)>>, end_pos: Pos3) {
    let mut grid = Array2::from_elem(input.raw_dim(), '.');
    let shape = input.shape();
    let rows = shape[0];
    let cols = shape[1];
    let mut curr_pos = end_pos;
    let mut next_c = None;
    while curr_pos.pos != (Pos { i: 0, j: 0 }) {
        if let Some(c) = next_c {
            grid[(curr_pos.pos.j as usize, curr_pos.pos.i as usize)] = c;
        }
        next_c = Some(match curr_pos.z {
            0 | 1 | 2 => '^',
            3 | 4 | 5 => '>',
            6 | 7 | 8 => 'v',
            9 | 10 | 11 => '<',
            _ => unreachable!(),
        });
        //dbg!{curr_pos};
        curr_pos = state[curr_pos].unwrap().1;
    }
    for row in 0..rows {
        for col in 0..cols {
            let c: char = grid[(row, col)];

            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 102);
    }
}
