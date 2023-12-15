use std::collections::HashMap;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Data = String;


#[aoc_generator(day15, part1)]
fn input_generator(input: &str) -> Result<Vec<Data>> {
    let mut result = Vec::new();

    for line in input.lines() {
        if line.is_empty() { continue; }

        result.extend(line.split(',').map(|s| s.to_owned()));
    }

    Ok(result)
}

#[aoc(day15, part1)]
fn solve_part1(input: &[Data]) -> usize {
    input.iter().map(|s| s.as_str()).map(hash_algorithm).sum()
}

fn hash_algorithm(s: &str) -> usize {
    let mut current: usize = 0;

    for c in s.chars() {
        current += c as usize;
        current *= 17;
        current %= 256;
    }

    current
}

#[derive(Debug, Clone)]
struct Step {
    label: String,
    instruction: Instruction,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Dash,
    Equals {
        focal_length: usize
    }
}

#[aoc_generator(day15, part2)]
fn input_generator_parsed(input: &str) -> Result<Vec<Step>> {
    let (input, result) = parse_input(input).map_err(|err| err.map(|err| anyhow!(nom::error::convert_error(input, err))))?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

use nom::bytes::complete::take_while1;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;

type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

fn parse_input(input: &str) -> IResult<&str, Vec<Step>> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = separated_list1(tag(","), parse_step)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, label_str) = is_not("-=")(input)?;
    let (input, operation_str) = take(1u8)(input)?;

    let label = label_str.to_owned();
    let mut input = input;
    use Instruction::*;
    let instruction = match operation_str {
        "-" => Dash,
        "=" => {
            let (next_input, focal_length) = parse_usize(input)?;
            input = next_input;
            Equals {
                focal_length,
            }
        },
        _ => unreachable!(),
    };

    Ok((input, Step {
        label,
        instruction,
    }))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

struct LabeledLens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[aoc(day15, part2)]
fn solve_part2<'a>(input: &'a[Step]) -> usize {
    let mut boxes: HashMap<usize, Vec<LabeledLens<'a>>> = HashMap::new();

    for step in input {
        let box_number = hash_algorithm(&step.label);
        let step_box = boxes.entry(box_number).or_insert_with(|| Vec::new());
        use Instruction::*;
        match step.instruction {
            Dash => {
                for i in 0..step_box.len() {
                    if step_box[i].label == step.label {
                        step_box.remove(i);
                        break;
                    }
                }
            },
            Equals { focal_length } => {
                let mut found = false;
                for i in 0..step_box.len() {
                    if step_box[i].label == step.label {
                        step_box[i].focal_length = focal_length;
                        found = true;
                        break;
                    }
                }
                if !found {
                    step_box.push(LabeledLens {
                        label: &step.label,
                        focal_length,
                    });
                }
            },
        }
    }

    let mut total = 0;
    for box_number in 0..256 {
        let step_box = boxes.get(&box_number).map(|v| v.as_slice()).unwrap_or(&[]);
        
        let focusing_power: usize = step_box.iter()
            .enumerate()
            .map(|(i, lens)| (i + 1, lens))
            .map(|(slot_number, lens)| (box_number + 1) * slot_number * lens.focal_length)
            .sum();

        total += focusing_power;
    }

    total
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator_parsed(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 145);
    }
}
