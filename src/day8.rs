use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use num::Integer;

type Data = (Instructions, Labels, Network);

type Instructions = Vec<Direction>;

type Labels = Vec<String>;

type Network = HashMap<Node, NodePair>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct NodePair {
    left: Node,
    right: Node,
}

impl NodePair {
    fn select_path(&self, direction: Direction) -> Node {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
struct Node {
    label: NodeId,
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
struct NodeId(usize);


#[aoc_generator(day8)]
fn input_generator(input: &str) -> Result<Data> {
    let (input, result) = parse_input(input).map_err(|err| err.to_owned())?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

use nom::IResult;
use nom::bytes::complete::take;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, directions) = parse_directions(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, (labels, network)) = parse_network(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, (directions, labels, network)))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = nom::bytes::complete::is_a("LR")(input)?;

    let directions = directions.chars()
        .map(|c| {
            match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            }
        })
        .collect();

    Ok((input, directions))
}

fn parse_network(input: &str) -> IResult<&str, (Labels, Network)> {
    let (input, str_network) = parse_str_network(input)?;

    let mut label_strs = HashSet::new();
    for line in &str_network {
        label_strs.insert(line.0);
        label_strs.insert(line.1.0);
        label_strs.insert(line.1.1);
    }

    let mut labels = Vec::with_capacity(label_strs.len());
    let mut label_str_to_node = HashMap::with_capacity(label_strs.len());

    for label in label_strs {
        let index = labels.len();
        let node = Node {
            label: NodeId(index),
        };
        labels.push(label.to_owned());
        label_str_to_node.insert(label, node);
    }

    let mut network = HashMap::new();

    for line in str_network {
        let label_node = label_str_to_node[line.0];

        let node_pair = NodePair {
            left: label_str_to_node[line.1.0],
            right: label_str_to_node[line.1.1],
        };

        network.insert(label_node, node_pair);
    }

    Ok((input, (labels, network)))
}

fn parse_str_network(input: &str) -> IResult<&str, Vec<(&str, (&str, &str))>> {
    separated_list1(tag("\n"), parse_single_node)(input)
}

fn parse_single_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, label) = parse_label(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = parse_label(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = parse_label(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (label, (left, right))))
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    take(3usize)(input)
}


#[aoc(day8, part1)]
fn solve_part1(input: &Data) -> usize {
    let goal_node = input.1.iter()
        .enumerate()
        .filter(|&(_, label)| label == "ZZZ")
        .map(|(i, _)| Node {
            label: NodeId(i),
        })
        .next()
        .unwrap();

    let mut current_node = input.1.iter()
        .enumerate()
        .filter(|&(_, label)| label == "AAA")
        .map(|(i, _)| Node {
            label: NodeId(i),
        })
        .next()
        .unwrap();

    let mut steps = 0;

    while current_node != goal_node {
        let index = steps % input.0.len();
        let direction = input.0[index];

        let next_nodes = &input.2[&current_node];

        current_node = next_nodes.select_path(direction);
        steps += 1;
    }

    steps
}

#[derive(Debug, Clone)]
struct CycleInfo {
    /// This was originally going to be for calculating the more complicated solve
    ///  solution, but it turns out that all of the cycle starts and solve times cancel each
    ///  other out and you're left with a multiple of cycle length.
    #[allow(unused)]
    solve_times: HashMap<Node, usize>,
    cycle_start_time: usize,
    cycle_end_time: usize,
}

#[aoc(day8, part2)]
fn solve_part2(input: &Data) -> usize {
    let starts: Vec<_> = input.1.iter()
        .enumerate()
        .filter(|&(_, label)| label.ends_with("A"))
        .map(|(i, _)| Node {
            label: NodeId(i),
        })
        .collect();
    let goals: HashSet<_> = input.1.iter()
        .enumerate()
        .filter(|&(_, label)| label.ends_with("Z"))
        .map(|(i, _)| Node {
            label: NodeId(i),
        })
        .collect();

    let info: Vec<_> = starts.iter()
        .map(|&start_node| {
            let mut current_node = start_node;
            let mut steps = 0;

            let mut solve_times = HashMap::new();

            let mut visited = HashMap::new();

            loop {
                let index = steps % input.0.len();
                if visited.contains_key(&(current_node, index)) {
                    break;
                }
                visited.insert((current_node, index), steps);
                let direction = input.0[index];

                let next_nodes = &input.2[&current_node];

                current_node = next_nodes.select_path(direction);
                steps += 1;

                if goals.contains(&current_node) {
                    solve_times.insert(current_node, steps);
                }
            }

            CycleInfo {
                solve_times,
                cycle_start_time: visited[&(current_node, steps % input.0.len())],
                cycle_end_time: steps,
            }
        })
        .collect();

    info.iter()
        .map(|info| info.cycle_end_time - info.cycle_start_time)
        .fold(1, |a, b| a.lcm(&b))
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    const TEST_INPUT2: &'static str =
r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
    const TEST_INPUT3: &'static str =
r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 6);

        let input = super::input_generator(TEST_INPUT2).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT3).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 6);
    }
}
