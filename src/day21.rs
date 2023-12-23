use std::collections::HashSet;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use ndarray::Array2;
use ndarray::ShapeBuilder;

type Input = (Pos, Array2<Data>);
type InputRef = (Pos, Array2<Data>);
type Data = bool;

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

    fn neighbors<'a>(&'a self, shape: &'a [usize]) -> impl Iterator<Item = Pos> + 'a  {
        DIRECTIONS.into_iter()
            .filter_map(move |direction| {
                self.try_move(direction, (shape[1], shape[0])).map(|neighbor| neighbor)
            })
    }
}

impl std::ops::Index<Pos> for Array2<bool> {
    type Output = <Self as std::ops::Index<ndarray::Ix2>>::Output;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[(index.i, index.j)]
    }
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Result<Input> {
    let mut row_data = Vec::new();

    let mut start = None;

    for (j, line) in input.lines().filter(|line| !line.is_empty()).enumerate()  {
        let mut row = Vec::new();
        for (i, c) in line.char_indices() {
            let cell = match c {
                '.' => Ok(false),
                'S' => Ok(false),
                '#' => Ok(true),
                _ => Err(anyhow!("Invalid character")),
            }?;

            if c == 'S' {
                start = Some(Pos { i, j });
            }

            row.push(cell);
        }

        row_data.push(row);
    }

    let cols = row_data[0].len();
    let rows = row_data.len();

    let data: Vec<_> = row_data.into_iter().flatten().collect();
    let shape = (rows, cols).strides((cols, 1));

    Ok((start.unwrap(), Array2::from_shape_vec(shape, data).unwrap()))
}

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

#[aoc(day21, part1)]
fn solve_part1(input: &InputRef) -> usize {
    solve_part1_inner(input.0, &input.1, 64)
}
fn solve_part1_inner(start: Pos, array: &Array2<bool>, steps: usize) -> usize {
    let mut current_positions = HashSet::new();
    let mut next_positions = HashSet::new();
    next_positions.insert(start);

    let shape = array.shape();
    for _ in 0..steps {
        std::mem::swap(&mut current_positions, &mut next_positions);

        for &pos in &current_positions {
            for neighbor in pos.neighbors(shape) {
                if !array[neighbor] {
                    next_positions.insert(neighbor);
                }
            }
        }
    }

    next_positions.len()
}
fn solve_part1_inner_iterated(start: Pos, array: &Array2<bool>, maximum_steps: usize) -> (usize, usize) {
    let mut current_positions = HashSet::new();
    let mut next_positions = HashSet::new();
    let mut next_next_positions = HashSet::new();
    next_positions.insert(start);

    let shape = array.shape();
    if maximum_steps % 2 == 1 {
        std::mem::swap(&mut current_positions, &mut next_positions);

        for &pos in &current_positions {
            for neighbor in pos.neighbors(shape) {
                if !array[neighbor] {
                    next_positions.insert(neighbor);
                }
            }
        }
    }
    std::mem::swap(&mut next_positions, &mut next_next_positions);
    current_positions.clear();
    for i in 0..maximum_steps / 2 {
        std::mem::swap(&mut current_positions, &mut next_next_positions);

        for &pos in &current_positions {
            for neighbor in pos.neighbors(shape) {
                if !array[neighbor] {
                    next_positions.insert(neighbor);
                }
            }
        }

        for &pos in &next_positions {
            for neighbor in pos.neighbors(shape) {
                if !array[neighbor] {
                    next_next_positions.insert(neighbor);
                }
            }
        }

        if current_positions == next_next_positions {
            let is_odd = maximum_steps % 2 == 1;
            return (i * 2 + 2 + is_odd as usize, current_positions.len());
        }
    }

    (maximum_steps, next_next_positions.len())
}

#[aoc(day21, part2)]
fn solve_part2(input: &InputRef) -> usize {
    solve_part2_inner(input, 26501365)
}
fn solve_part2_inner(input: &InputRef, maximum_steps: usize) -> usize {
    let side_length = {
        let shape = input.1.shape();
        assert_eq!(shape[0], shape[1]);
        shape[0]
    };
    let start = input.0;
    assert_eq!(start.i, side_length / 2);
    assert_eq!(start.j, side_length / 2);
    let array = &input.1;
    // Split it into cases
    // Center
    let center = solve_part1_inner_iterated(input.0, &input.1, maximum_steps).1;

    // Compass Points
    // Up
    let up = if maximum_steps >= start.j + 1 {
        let initial_steps = maximum_steps - input.0.j - 1;
        let start = Pos { i: start.i, j: side_length - 1 };

        solve_cardinal(array, start, initial_steps, side_length)
    } else { 0 };
    // Down
    let down = if maximum_steps >= start.j + 1 {
        let initial_steps = maximum_steps - input.0.j - 1;
        let start = Pos { i: start.i, j: 0 };

        solve_cardinal(array, start, initial_steps, side_length)
    } else { 0 };
    // Left
    let left = if maximum_steps >= start.j + 1 {
        let initial_steps = maximum_steps - input.0.j - 1;
        let start = Pos { i: side_length - 1, j: start.j, };

        solve_cardinal(array, start, initial_steps, side_length)
    } else { 0 };
    // Right
    let right = if maximum_steps >= start.j + 1 {
        let initial_steps = maximum_steps - input.0.j - 1;
        let start = Pos { i: 0, j: start.j, };

        solve_cardinal(array, start, initial_steps, side_length)
    } else { 0 };

    // Diagonals
    // Up Left
    let up_left = if maximum_steps >= start.j + start.i + 2 {
        let initial_steps = maximum_steps - input.0.j - input.0.i - 2;
        let start = Pos { i: side_length - 1, j: side_length - 1 };

        solve_diagonal(array, start, initial_steps, side_length)
    } else { 0 };
    // Up Right
    let up_right = if maximum_steps >= start.j + start.i + 2 {
        let initial_steps = maximum_steps - input.0.j - input.0.i - 2;
        let start = Pos { i: 0, j: side_length - 1 };

        solve_diagonal(array, start, initial_steps, side_length)
    } else { 0 };
    // Down Right
    let down_right = if maximum_steps >= start.j + start.i + 2 {
        let initial_steps = maximum_steps - input.0.j - input.0.i - 2;
        let start = Pos { i: 0, j: 0 };

        solve_diagonal(array, start, initial_steps, side_length)
    } else { 0 };
    // Down Left
    let down_left = if maximum_steps >= start.j + start.i + 2 {
        let initial_steps = maximum_steps - input.0.j - input.0.i - 2;
        let start = Pos { i: side_length - 1, j: 0 };

        solve_diagonal(array, start, initial_steps, side_length)
    } else { 0 };

    let reachable_points =
        dbg!{center} +
        dbg!{up} +
        dbg!{down} +
        dbg!{left} +
        dbg!{right} +
        dbg!{up_left} +
        dbg!{up_right} +
        dbg!{down_right} +
        dbg!{down_left};
    reachable_points
}

fn solve_cardinal(array: &Array2<bool>, start: Pos, initial_steps: usize, side_length: usize) -> usize {
    let mut reachable_points = 0;
    let mut steps = initial_steps % side_length;

    dbg!{initial_steps, steps};

    let (stable_steps_start, maximum_reachable_start) = solve_part1_inner_iterated(start, array, initial_steps);
    let (stable_steps_off, maximum_reachable_off) = if initial_steps >= side_length {
        solve_part1_inner_iterated(start, array, initial_steps - side_length)
    } else {
        (0, 0)
    };

    let stable_steps = std::cmp::max(stable_steps_start, stable_steps_off);

    while steps < stable_steps {
        reachable_points += solve_part1_inner(start, array, steps);
        steps += side_length;
    }
    if steps <= initial_steps {
        assert_eq!((initial_steps - steps) % side_length, 0);
        let count = (initial_steps - steps) / side_length + 1;

        let start_count = (count + 1) / 2;
        let off_count = count / 2;

        reachable_points += start_count * maximum_reachable_start;
        reachable_points += off_count * maximum_reachable_off;
    }
    reachable_points
}
fn solve_diagonal(array: &Array2<bool>, start: Pos, initial_steps: usize, side_length: usize) -> usize {
    let mut reachable_points = 0;
    let mut steps = initial_steps % side_length;

    let (stable_steps_start, maximum_reachable_start) = solve_part1_inner_iterated(start, array, initial_steps);
    let (stable_steps_off, maximum_reachable_off) = if initial_steps >= side_length {
        solve_part1_inner_iterated(start, array, initial_steps - side_length)
    } else {
        (0, 0)
    };

    let stable_steps = std::cmp::max(stable_steps_start, stable_steps_off);

    let mut n = (initial_steps - steps) / side_length + 1;
    while steps <= stable_steps {
        reachable_points += n * solve_part1_inner(start, array, steps);
        steps += side_length;
        n -= 1;
    }

    assert_eq!(initial_steps.abs_diff(steps) % side_length, 0);

    let start_rows = (n + 1) / 2;
    let off_rows = n / 2;
    let start_count = if start_rows > 0 {
        start_rows + (start_rows - 1) * start_rows
    } else {
        0
    };
    let off_count = off_rows * (off_rows + 1);

    reachable_points += start_count * maximum_reachable_start;
    reachable_points += off_count * maximum_reachable_off;
    reachable_points
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;
    const TEST_SIMPLE: &'static str =
r#"
...
.S.
...
"#;
    const TEST_WALL: &'static str =
r#"
.........
.........
.........
.........
....S....
.......#.
.......#.
.....###.
.........
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1_inner(input.0, &input.1, 6);

        assert_eq!(result, 16);
    }

    #[test]
    fn test_part2_simple() {
        let input = super::input_generator(TEST_SIMPLE).unwrap();

        let result = super::solve_part2_inner(&input, 1);
        assert_eq!(result, 4);

        let result = super::solve_part2_inner(&input, 2);
        assert_eq!(result, 9);

        let result = super::solve_part2_inner(&input, 3);
        assert_eq!(result, 16);

        let result = super::solve_part2_inner(&input, 4);
        assert_eq!(result, 25);

        let result = super::solve_part2_inner(&input, 5);
        assert_eq!(result, 36);

        let result = super::solve_part2_inner(&input, 12);
        assert_eq!(result, 169);
    }

    #[test]
    fn test_part2_wall() {
        let input = super::input_generator(TEST_WALL).unwrap();

        dbg!{super::solve_part2_inner(&input, 12)};

        assert!(false);
    }

    
    /*
    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();

        let result = super::solve_part2_inner(&input, 6);
        assert_eq!(result, 16);

        let result = super::solve_part2_inner(&input, 10);
        assert_eq!(result, 50);

        let result = super::solve_part2_inner(&input, 50);
        assert_eq!(result, 1594);

        let result = super::solve_part2_inner(&input, 100);
        assert_eq!(result, 6536);

        let result = super::solve_part2_inner(&input, 500);
        assert_eq!(result, 167004);

        let result = super::solve_part2_inner(&input, 1000);
        assert_eq!(result, 668697);

        let result = super::solve_part2_inner(&input, 5000);
        assert_eq!(result, 16733044);
    }
    */
}
