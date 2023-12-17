use aoc_runner_derive::aoc;

use anyhow::Result;

use ndarray::Array2;
use ndarray::Array3;

type Input = Data;
type InputRef = Data;
type Data = Array2<u32>;

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

    fn to_index(self) -> usize {
        use Direction::*;
        match self {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

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
                next_pos.i < shape[1] as i32 && next_pos.j < shape[0] as i32 {
            Some(next_pos)
        } else {
            None
        }
    }

    fn neighbors<'a>(&'a self, direction: Direction, costs: &'a Array2<u32>) -> impl Iterator<Item = (Pos, Direction, u32)> + 'a  {
        let shape = costs.shape();
        DIRECTIONS.iter()
            .cloned()
            .filter(move |&d| d != direction.invert() && d != direction)
            .flat_map(move |d| {
                let mut curr_pos = Some(*self);
                let mut cost = 0;
                (0..=10).filter_map(move |i| {
                    let result = if i >= 4 {
                        curr_pos.map(|pos| (pos, cost))
                    } else {
                        None
                    };
                    curr_pos = curr_pos.and_then(|curr_pos| curr_pos.try_move(d, shape));
                    if let Some(curr_pos) = curr_pos {
                        cost += costs[curr_pos];
                    }
                    result
                })
                .map(move |(p, cost)| (p, d, cost))
            })
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
        &self[(index.pos.j as usize, index.pos.i as usize, index.direction.to_index())]
    }
}

impl std::ops::IndexMut<Pos3> for Array3<Option<u32>> {
    fn index_mut(&mut self, index: Pos3) -> &mut Self::Output {
        &mut self[(index.pos.j as usize, index.pos.i as usize, index.direction.to_index())]
    }
}

impl std::ops::Index<Pos3> for Array3<Option<(u32, Pos3)>> {
    type Output = <Self as std::ops::Index<ndarray::Ix3>>::Output;

    fn index(&self, index: Pos3) -> &Self::Output {
        &self[(index.pos.j as usize, index.pos.i as usize, index.direction.to_index())]
    }
}

impl std::ops::IndexMut<Pos3> for Array3<Option<(u32, Pos3)>> {
    fn index_mut(&mut self, index: Pos3) -> &mut Self::Output {
        &mut self[(index.pos.j as usize, index.pos.i as usize, index.direction.to_index())]
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos3 {
    pos: Pos,
    direction: Direction,
}

use crate::day17::input_generator;
#[aoc(day17, part2)]
fn solve_part2(input: &InputRef) -> u32 {
    let mut state: Array3<Option<(u32, Pos3)>> = Array3::from_elem([input.shape()[0], input.shape()[1], 12], None);

    let start_pos = Pos { i: 0, j: 0 };
    let shape = input.shape();
    let rows = shape[0];
    let cols = shape[1];
    let end_pos = Pos { i: cols as i32 - 1, j: rows as i32 - 1 };

    let mut frontier = Vec::new();
    // cur_pos, dir, consecutive, g, f
    frontier.push((start_pos, Direction::Left, 0, h(start_pos, shape)));
    let index = Pos3 { pos: start_pos, direction: Direction::Left };
    state[index] = Some((h(start_pos, shape), index));
    frontier.push((start_pos, Direction::Up, 0, h(start_pos, shape)));
    let index = Pos3 { pos: start_pos, direction: Direction::Up };
    state[index] = Some((h(start_pos, shape), index));

    while !frontier.is_empty() {
        frontier.sort_by(|a, b| a.3.cmp(&b.3));
        let last = frontier.len() - 1;
        frontier.swap(0, last);
        let (pos, direction, g, f) = frontier.pop().unwrap();
        let curr_pos3 = Pos3 { pos, direction, };

        if state[curr_pos3].unwrap().0 < f { continue; }

        if pos == end_pos {
            pretty_print(input, &state, curr_pos3);
            return g;
        }

        //dbg!{pos, direction, g};
        for neighbor in pos.neighbors(direction, input) {
            let cost = neighbor.2;
            //dbg!{pos, neighbor.0, cost};

            let new_g = g + cost;
            let new_f = new_g + h(neighbor.0, shape);

            let index = Pos3 { pos: neighbor.0, direction: neighbor.1 };
            let previous_f: Option<(u32, Pos3)> = state[index];

            if previous_f.is_none() || previous_f.unwrap().0 > new_f {
                state[index] = Some((new_f, curr_pos3));
                frontier.push((neighbor.0, neighbor.1, new_g, new_f));
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
        use Direction::*;
        next_c = Some(match curr_pos.direction {
            Up => '^',
            Right => '>',
            Down => 'v',
            Left => '<',
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
    const TEST_PATHOLOGICAL: &'static str =
r#"
111111111111
999999999991
999999999991
999999999991
999999999991
"#;

    #[test]
    fn test_part2_example() {
        let input = crate::day17::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 94);

        let input = crate::day17::input_generator(TEST_PATHOLOGICAL).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 71);
    }
}
