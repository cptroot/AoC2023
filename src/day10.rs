use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use ndarray::Array2;
use ndarray::ShapeBuilder;

use std::collections::HashMap;
use std::collections::HashSet;

type Data = (Pos, ndarray::Array2<Adjacencies>);

#[derive(Debug, Clone, Copy)]
struct Adjacencies {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

impl From<[bool; 4]> for Adjacencies {
    fn from(values: [bool; 4]) -> Self {
        Adjacencies {
            up: values[0],
            right: values[1],
            down: values[2],
            left: values[3],
        }
    }
}

impl Adjacencies {
    fn other_direction(&self, direction: Direction) -> Option<Direction> {
        let mut directions = Vec::new();
        use Direction::*;
        if self.up { directions.push(Up); }
        if self.down { directions.push(Down); }
        if self.right { directions.push(Right); }
        if self.left { directions.push(Left); }

        if directions.contains(&direction.invert()) {
            directions.into_iter().filter(|&d| d != direction.invert()).next()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct Pos {
    i: usize,
    j: usize,
}

impl Pos {
    fn try_move(&self, direction: Direction, bounds: (usize, usize)) -> Option<Self> {
        use Direction::*;
        match direction {
            Up => {
                if self.j > 0 {
                    Some(Pos {
                        i: self.i,
                        j: self.j - 1,
                    })
                } else {
                    None
                }
            },
            Right => {
                if self.i < bounds.0 - 1 {
                    Some(Pos {
                        i: self.i + 1,
                        j: self.j,
                    })
                } else {
                    None
                }
            },
            Down => {
                if self.j < bounds.1 - 1 {
                    Some(Pos {
                        i: self.i,
                        j: self.j + 1,
                    })
                } else {
                    None
                }
            },
            Left => {
                if self.i > 0 {
                    Some(Pos {
                        i: self.i - 1,
                        j: self.j,
                    })
                } else {
                    None
                }
            },
        }
    }
}

impl std::ops::Index<Pos> for Array2<Adjacencies> {
    type Output = <Self as std::ops::Index<ndarray::Ix2>>::Output;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[(index.i, index.j)]
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Result<Data> {
    let character_map = {
        let mut tmp: HashMap<char, Adjacencies> = HashMap::new();
        tmp.insert('|', [true, false, true, false].into());
        tmp.insert('-', [false, true, false, true].into());
        tmp.insert('L', [true, true, false, false].into());
        tmp.insert('J', [true, false, false, true].into());
        tmp.insert('7', [false, false, true, true].into());
        tmp.insert('F', [false, true, true, false].into());
        tmp.insert('.', [false, false, false, false].into());
        tmp.insert('S', [false, false, false, false].into());

        tmp
    };

    let mut rows = Vec::new();
    let mut rowlength = None;

    let mut start = None;

    for (j, line) in input.lines().filter(|line| !line.is_empty()).enumerate()  {
        let mut row = Vec::new();
        for (i, c) in line.char_indices() {
            if c == 'S' {
                start = Some(Pos {
                    i,
                    j,
                });
            }
            row.push(character_map[&c]);
        }

        if let None = rowlength { rowlength = Some(row.len()); }
        rows.extend(row);
    }

    let rowlength = rowlength.unwrap();
    let shape = (rowlength, rows.len() / rowlength).strides((1, rowlength));

    Ok((start.unwrap(), Array2::from_shape_vec(shape, rows).unwrap()))
}

#[aoc(day10, part1)]
fn solve_part1(input: &Data) -> usize {
    let start = input.0;

    let array = &input.1;
    let bounds = (array.shape()[0], array.shape()[1]);

    let neighbors = get_neighbors(start, array);

    for (mut incoming_direction, neighbor) in neighbors {
        let mut current_position = neighbor;
        let mut length = 1;

        while current_position != start {
            let cell = array[current_position];
            let next_direction = cell.other_direction(incoming_direction);
            
            if let Some(next_direction) = next_direction {
                let next_position = current_position.try_move(next_direction, bounds);
                if let Some(next_position) = next_position {
                    current_position = next_position;
                    incoming_direction = next_direction;
                } else {
                    break;
                }
            } else {
                break;
            }

            length += 1;
        }

        if current_position == start {
            return length / 2;
        }
    }

    0
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
    fn invert(self) -> Self {
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

fn get_neighbors(location: Pos, array: &Array2<Adjacencies>) -> impl Iterator<Item = (Direction, Pos)> {
    let bounds = (array.shape()[0], array.shape()[1]);
    DIRECTIONS.into_iter()
        .filter_map(move |direction| {
            location.try_move(direction, bounds).map(|neighbor| (direction, neighbor))
        })
}

#[aoc(day10, part2)]
fn solve_part2(input: &Data) -> usize {
    let mut path = solve_maze(input);

    let boundary: HashSet<_> = path.iter().map(|(_, pos, _)| pos).cloned().collect();

    let winding_number = calculate_winding_number(&path);
    dbg!{winding_number};
    if winding_number != 4 {
        path.reverse();
        path.iter_mut().for_each(|point| {
            *point = (point.2.invert(), point.1, point.0.invert());
        });
    }

    let mut interior = HashSet::new();

    use Direction::*;
    let bounds = (input.1.shape()[0], input.1.shape()[1]);
    for point in &path {
        let directions = match (point.0, point.2) {
            (Up, Up) => [Right].iter(),
            (Right, Right) => [Down].iter(),
            (Down, Down) => [Left].iter(),
            (Left, Left) => [Up].iter(),
            (Up, Left) => [Up, Right].iter(),
            (Right, Up) => [Right, Down].iter(),
            (Down, Right) => [Down, Left].iter(),
            (Left, Down) => [Left, Up].iter(),
            _ => [].iter(),
        };
        for &d in directions {
            let potential_interior_pos = point.1.try_move(d, bounds);

            if let Some(pos) = potential_interior_pos {
                if !boundary.contains(&pos) {
                    interior.insert(pos);
                }
            }
        }
    }

    let mut frontier: Vec<_> = interior.iter().cloned().collect();

    while !frontier.is_empty() {
        let pos = frontier.pop().unwrap();

        for (_, neighbor) in get_neighbors(pos, &input.1) {
            if !interior.contains(&neighbor) && !boundary.contains(&neighbor) {
                frontier.push(neighbor);
                interior.insert(neighbor);
            }
        }
    }

    interior.len()
}

fn calculate_winding_number(path: &[(Direction, Pos, Direction)]) -> i32 {
    use Direction::*;
    path.iter().map(|point| {
        match (point.0, point.2) {
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

fn solve_maze(input: &Data) -> Vec<(Direction, Pos, Direction)> {
    let start = input.0;

    let array = &input.1;
    let bounds = (array.shape()[0], array.shape()[1]);

    let neighbors = get_neighbors(start, array);

    for (incoming_direction, neighbor) in neighbors {
        let mut current_position = neighbor;
        let mut current_direction = incoming_direction;

        let mut history = Vec::new();

        while current_position != start {
            let cell = array[current_position];
            let next_direction = cell.other_direction(current_direction);
            
            if let Some(next_direction) = next_direction {
                let next_position = current_position.try_move(next_direction, bounds);
                if let Some(next_position) = next_position {
                    history.push((current_direction, current_position, next_direction));
                    current_position = next_position;
                    current_direction = next_direction;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        history.push((current_direction, current_position, incoming_direction));

        if current_position == start {
            return history;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT_CIRCLE: &'static str =
r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

    const TEST_INPUT_COMPLICATED: &'static str =
r#"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"#;

    const TEST_INPUT_1_TILE: &'static str =
r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

    const TEST_INPUT_CORNERS: &'static str =
r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT_CIRCLE).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 4);

        let input = super::input_generator(TEST_INPUT_COMPLICATED).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT_1_TILE).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 4);

        let input = super::input_generator(TEST_INPUT_CORNERS).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 8);
    }
}
