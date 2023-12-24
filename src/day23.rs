use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use ndarray::Array2;
use ndarray::ShapeBuilder;

use petgraph::stable_graph::NodeIndex;
use petgraph::visit::EdgeRef;

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
enum Data {
    Empty,
    Wall,
    Slope(Direction),
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

#[aoc_generator(day23)]
fn input_generator(input: &str) -> Result<Array2<Data>> {
    let mut row_data = Vec::new();

    for (_, line) in input.lines().filter(|line| !line.is_empty()).enumerate()  {
        let mut row = Vec::new();
        for (_, c) in line.char_indices() {
            let cell = match c {
                '.' => Ok(Data::Empty),
                '#' => Ok(Data::Wall),
                '^' => Ok(Data::Slope(Direction::Up)),
                '>' => Ok(Data::Slope(Direction::Right)),
                'v' => Ok(Data::Slope(Direction::Down)),
                '<' => Ok(Data::Slope(Direction::Left)),
                _ => Err(anyhow!("Invalid character")),
            }?;

            row.push(cell);
        }

        row_data.push(row);
    }

    let cols = row_data[0].len();
    let rows = row_data.len();

    let data: Vec<_> = row_data.into_iter().flatten().collect();
    let shape = (rows, cols).strides((cols, 1));

    Ok(Array2::from_shape_vec(shape, data).unwrap())
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

    fn neighbors<'a>(&'a self, shape: &'a [usize]) -> impl Iterator<Item = Pos> + 'a  {
        DIRECTIONS.into_iter()
            .filter_map(move |direction| {
                self.try_move(direction, (shape[1], shape[0])).map(|neighbor| neighbor)
            })
    }
}

impl std::ops::Index<Pos> for Array2<Data> {
    type Output = <Self as std::ops::Index<ndarray::Ix2>>::Output;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[(index.j as usize, index.i as usize)]
    }
}

#[aoc(day23, part1)]
fn solve_part1(input: &Array2<Data>) -> usize {
    let shape = input.shape();
    let rows = shape[0];
    let cols = shape[1];
    let start = Pos { i: 1, j: 0 };
    let end = Pos { i: cols - 2, j: rows - 1 };

    let mut nodes = HashMap::new();

    let mut graph = petgraph::Graph::new();

    for ((j, i), value) in input.indexed_iter() {
        let current_pos = Pos { i, j };
        if value != &Data::Wall {
            let node = graph.add_node(());
            nodes.insert(current_pos, node);
        }
    }

    for ((j, i), value) in input.indexed_iter() {
        let current_pos = Pos { i, j };
        match value {
            Data::Empty => {
                let this_node = nodes[&current_pos];
                for neighbor in current_pos.neighbors(shape) {
                    if input[neighbor] != Data::Wall {
                        graph.add_edge(this_node, nodes[&neighbor], 1);
                    }
                }
            },
            &Data::Slope(direction) => {
                if let Some(next) = current_pos.try_move(direction, (cols, rows)) {
                    let this_node = nodes[&current_pos];
                    graph.add_edge(this_node, nodes[&next], 1);
                }
            },
            Data::Wall => { },
        }
    }

    dfs_max(&graph, nodes[&start], nodes[&end])
}

fn dfs_max(graph: &petgraph::Graph<(), usize>, start: NodeIndex, end: NodeIndex) -> usize {
    let mut visited = HashSet::new();
    dfs_max_inner(graph, &mut visited, start, end)
}
fn dfs_max_inner(graph: &petgraph::Graph<(), usize>, visited: &mut HashSet<NodeIndex>, current: NodeIndex, end: NodeIndex) -> usize {
    if end == current { return 0; }
    visited.insert(current);

    let mut maximum_cost = 0;
    for edge in graph.edges(current) {
        let neighbor = edge.target();
        if visited.contains(&neighbor) { continue; }

        let cost = dfs_max_inner(graph, visited, neighbor, end) + edge.weight();
        maximum_cost = std::cmp::max(cost, maximum_cost);
    }
    
    visited.remove(&current);

    maximum_cost
}

#[aoc(day23, part2)]
fn solve_part2(input: &Array2<Data>) -> usize {
    let shape = input.shape();
    let rows = shape[0];
    let cols = shape[1];
    let start = Pos { i: 1, j: 0 };
    let end = Pos { i: cols - 2, j: rows - 1 };

    let mut nodes = HashMap::new();
    let mut graph = petgraph::Graph::new();

    let node = graph.add_node(());
    nodes.insert(start, node);
    let node = graph.add_node(());
    nodes.insert(end, node);

    for ((j, i), value) in input.indexed_iter() {
        let current_pos = Pos { i, j };
        if value != &Data::Wall {
            let valid_neighbor_count = current_pos.neighbors(shape)
                .filter(|&neighbor| input[neighbor] != Data::Wall)
                .count();
            if valid_neighbor_count > 2 {
                let node = graph.add_node(());
                nodes.insert(current_pos, node);
            }
        }
    }

    for (&current_pos, &node) in &nodes {
        for neighbor in current_pos.neighbors(shape) {
            if input[neighbor] == Data::Wall { continue; }
            let (end_node, cost) = traverse_path(input, current_pos, neighbor, &nodes);
            if !graph.contains_edge(node, end_node) {
                graph.add_edge(node, end_node, cost);
            }
        }
    }

    dfs_max(&graph, nodes[&start], nodes[&end])
}

fn traverse_path(input: &Array2<Data>, mut prev_pos: Pos, mut current_pos: Pos, nodes: &HashMap<Pos, NodeIndex>) -> (NodeIndex, usize) {
    let shape = input.shape();

    let mut cost = 1;
    while !nodes.contains_key(&current_pos) {
        let next_pos = current_pos.neighbors(shape)
            .filter(|&neighbor| neighbor != prev_pos)
            .filter(|&neighbor| input[neighbor] != Data::Wall)
            .next().unwrap();

        cost += 1;
        (prev_pos, current_pos) = (current_pos, next_pos);
    }

    (nodes[&current_pos], cost)
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 94);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 154);
    }
}
