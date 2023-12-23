use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = Vec<Data>;
type InputRef = [Data];
type Data = (Pos3, Pos3);

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct Pos3 {
    x: i64,
    y: i64,
    z: i64,
}


#[aoc_generator(day22)]
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
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::sequence::separated_pair;
use nom::sequence::tuple;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = separated_list1(tag("\n"), parse_block)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_block(input: &str) -> IResult<&str, Data> {
    let (input, p1) = parse_pos3(input)?;
    let (input, _) = tag("~")(input)?;
    let (input, p2) = parse_pos3(input)?;

    Ok((input, (p1, p2)))
}

fn parse_pos3(input: &str) -> IResult<&str, Pos3> {
    let (input, (x, _, y, _, z)) = tuple((
        parse_i64,
        tag(","),
        parse_i64,
        tag(","),
        parse_i64,
    ))(input)?;

    Ok((input, Pos3 {
        x,
        y,
        z,
    }))
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[aoc(day22, part1)]
fn solve_part1(input: &InputRef) -> usize {
    let mut heights: Vec<_> = input.iter().enumerate()
        .map(|(i, (p1, p2))| {
            (std::cmp::min(p1.z, p2.z), i)
        })
        .collect();
    heights.sort_by_key(|p| p.0);

    let mut placed_blocks = HashMap::new();
    let mut supporting_blocks = HashSet::new();

    for &(mut height, i) in &heights {
        let mut supporting = HashSet::new();
        while height > 0 && supporting.is_empty() {
            height -= 1;
            if height == 0 { break; }
            for (x, y) in footprint(&input[i]) {
                let pos = Pos3 {
                    x,
                    y,
                    z: height,
                };
                if placed_blocks.contains_key(&pos) {
                    supporting.insert(placed_blocks[&pos]);
                }
            }
        }

        height += 1;

        if supporting.len() == 1 {
            supporting_blocks.extend(supporting.drain());
        }

        let top_height = input[i].0.z.abs_diff(input[i].1.z) as i64 + height;

        for (x, y) in footprint(&input[i]) {
            let pos = Pos3 {
                x,
                y,
                z: top_height,
            };
            placed_blocks.insert(pos, i);
        }
    }

    (0..input.len()).filter(|i| !supporting_blocks.contains(i)).count()
}

fn footprint((p1, p2): &(Pos3, Pos3)) -> impl Iterator<Item = (i64, i64)> {
    let x1 = std::cmp::min(p1.x, p2.x);
    let x2 = std::cmp::max(p1.x, p2.x);
    let y1 = std::cmp::min(p1.y, p2.y);
    let y2 = std::cmp::max(p1.y, p2.y);
    (x1..=x2).flat_map(move |x| (y1..=y2).map(move |y| (x, y)))
}

#[aoc(day22, part2)]
fn solve_part2(input: &InputRef) -> usize {
    let mut heights: Vec<_> = input.iter().enumerate()
        .map(|(i, (p1, p2))| {
            (std::cmp::min(p1.z, p2.z), i)
        })
        .collect();
    heights.sort_by_key(|p| p.0);

    let mut placed_blocks = HashMap::new();
    let mut supported_blocks: Vec<Vec<usize>> = vec![Vec::new(); input.len()];
    let mut supporting_blocks = vec![Vec::new(); input.len()];

    for &(mut height, i) in &heights {
        let mut supporting: HashSet<usize> = HashSet::new();
        while height > 0 && supporting.is_empty() {
            height -= 1;
            if height == 0 { break; }
            for (x, y) in footprint(&input[i]) {
                let pos = Pos3 {
                    x,
                    y,
                    z: height,
                };
                if placed_blocks.contains_key(&pos) {
                    supporting.insert(placed_blocks[&pos]);
                }
            }
        }

        height += 1;

        for supporting in supporting.drain() {
            supported_blocks[supporting].push(i);
            supporting_blocks[i].push(supporting);
        }

        let top_height = input[i].0.z.abs_diff(input[i].1.z) as i64 + height;

        for (x, y) in footprint(&input[i]) {
            let pos = Pos3 {
                x,
                y,
                z: top_height,
            };
            placed_blocks.insert(pos, i);
        }
    }

    let mut total_fallen = 0;

    for i in 0..input.len() {
        let mut chain = HashSet::new();
        chain.insert(i);

        let mut candidates = HashSet::new();

        candidates.extend(supported_blocks[i].iter().cloned());

        for &(_, j) in heights.iter() {
            if !candidates.contains(&j) { continue; }
            let chains = supporting_blocks[j].iter()
                .filter(|supporting| !chain.contains(supporting))
                .next().is_none();

            if chains {
                chain.insert(j);
                candidates.extend(supported_blocks[j].iter().cloned());
            }
        }

        total_fallen += chain.len() - 1;
    }

    total_fallen
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 7);
    }
}
