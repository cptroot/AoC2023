use std::collections::HashMap;
use std::ops::RangeInclusive;

use std::cmp::{max, min};

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = (HashMap<WorkflowName, Workflow>, Vec<Part>);
type InputRef = Input;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct WorkflowName(String);
#[derive(Debug, Clone)]
struct Workflow {
    name: WorkflowName,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
enum Rule {
    Comparison(ComparisonRule),
    Jump(JumpTarget),
}

#[derive(Debug, Clone)]
struct ComparisonRule {
    category: Category,
    comp: ComparisonDirection,
    value: u32,
    jump_target: JumpTarget,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
enum JumpTarget {
    Reject,
    Accept,
    Jump(WorkflowName),
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum ComparisonDirection {
    Less,
    Greater,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn total_rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}


#[aoc_generator(day19)]
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
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::sequence::tuple;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, workflow_vec) = separated_list1(tag("\n"), parse_workflow)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, parts) = separated_list1(tag("\n"), parse_part)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    let workflows = workflow_vec.into_iter().map(|w| (w.name.clone(), w)).collect();

    Ok((input, (workflows, parts)))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = parse_name(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = separated_list1(tag(","), parse_rule)(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, Workflow { name, rules }))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((
        map(parse_comparison_rule, |comp_rule| Rule::Comparison(comp_rule)),
        map(parse_jump_target, |jump_target| Rule::Jump(jump_target)),
    ))(input)
}

fn parse_comparison_rule(input: &str) -> IResult<&str, ComparisonRule> {
    let (input, category_c) = one_of("xmas")(input)?;
    let (input, comp_c) = one_of("<>")(input)?;
    let (input, value) = parse_u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, jump_target) = parse_jump_target(input)?;

    let category = match category_c {
        'x' => Category::X,
        'm' => Category::M,
        'a' => Category::A,
        's' => Category::S,
        _ => unreachable!(),
    };
    let comp = match comp_c {
        '<' => ComparisonDirection::Less,
        '>' => ComparisonDirection::Greater,
        _ => unreachable!(),
    };

    Ok((input, ComparisonRule {
        category,
        comp,
        value,
        jump_target,
    }))
}

fn parse_jump_target(input: &str) -> IResult<&str, JumpTarget> {
    alt((
        map(one_of("AR"), |c| match c {
            'A' => JumpTarget::Accept,
            'R' => JumpTarget::Reject,
            _ => unreachable!(),
        }),
        map(parse_name, |name| JumpTarget::Jump(name)),
    ))(input)
}

fn parse_name(input: &str) -> IResult<&str, WorkflowName> {
    map(
        take_while1(|c: char| ('a'..='z').contains(&c)),
        |name: &str| WorkflowName(name.to_owned())
    )(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{")(input)?;
    let (input, (x, _, m, _, a, _, s)) = tuple((
        parse_val("x"), tag(","),
        parse_val("m"), tag(","),
        parse_val("a"), tag(","),
        parse_val("s")
    ))(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, Part {
        x,
        m,
        a,
        s,
    }))
}

fn parse_val(category: &str) -> impl Fn(&str) -> IResult<&str, u32> + '_ {
    move |input: &str| {
        let (input, _) = tag(category)(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, value) = parse_u32(input)?;

        Ok((input, value))
    }
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[aoc(day19, part1)]
fn solve_part1(input: &InputRef) -> u32 {
    let start = WorkflowName("in".to_owned());
    let mut total = 0;
    for part in &input.1 {
        let mut current_workflow_name = &start;

        let accepted = 'outer: loop {
            let current_workflow = &input.0[current_workflow_name];

            for rule in &current_workflow.rules {
                let jump_target = match rule {
                    &Rule::Jump(ref jump_target) => Some(jump_target),
                    &Rule::Comparison(ref comp) => {
                        let current_value = match comp.category {
                            Category::X => part.x,
                            Category::M => part.m,
                            Category::A => part.a,
                            Category::S => part.s,
                        };

                        let passed = match comp.comp {
                            ComparisonDirection::Less => current_value < comp.value,
                            ComparisonDirection::Greater => current_value > comp.value,
                        };

                        if passed {
                            Some(&comp.jump_target)
                        } else {
                            None
                        }
                    },
                };

                match jump_target {
                    Some(JumpTarget::Accept) => { break 'outer true; },
                    Some(JumpTarget::Reject) => { break 'outer false; },
                    Some(JumpTarget::Jump(jump_target)) => {
                        current_workflow_name = jump_target;
                        continue 'outer;
                    },
                    None => { },
                }
            }
        };

        if accepted {
            total += part.total_rating();
        }
    }
    total
}

#[derive(Debug, Clone)]
struct Quad {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

impl Quad {
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }
}

fn find_quad(input: &InputRef, workflow: &Workflow, index: usize) -> Option<Quad> {
    let quad = Quad {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    let end = WorkflowName("in".to_owned());

    find_quad_recursive(input, &end, workflow, index, true, quad)
}

fn find_quad_recursive(input: &InputRef, stop_name: &WorkflowName, current_workflow: &Workflow, current_index: usize, jumped: bool, mut current_quad: Quad) -> Option<Quad> {
    match &current_workflow.rules[current_index] {
        &Rule::Jump(JumpTarget::Accept) => { },
        &Rule::Jump(_) => {
            if !jumped {
                return dbg!{None};
            }
        },
        &Rule::Comparison(ref rule) => {
            let value = match rule.category {
                Category::X => &mut current_quad.x,
                Category::M => &mut current_quad.m,
                Category::A => &mut current_quad.a,
                Category::S => &mut current_quad.s,
            };

            if jumped {
                match rule.comp {
                    ComparisonDirection::Less => {
                        *value = *value.start()..=min(*value.end(), rule.value - 1);
                    },
                    ComparisonDirection::Greater => {
                        *value = max(*value.start(), rule.value + 1)..=*value.end();
                    },
                }
            } else {
                match rule.comp {
                    ComparisonDirection::Greater => {
                        *value = *value.start()..=min(*value.end(), rule.value);
                    },
                    ComparisonDirection::Less => {
                        *value = max(*value.start(), rule.value)..=*value.end();
                    },
                }
            }
        },
    }

    if current_quad.is_empty() { return dbg!{None}; }

    if current_index == 0 {
        //dbg!{&current_workflow.name, &current_quad};
        if &current_workflow.name == stop_name {
            Some(current_quad)
        } else {
            let jump_target = JumpTarget::Jump(current_workflow.name.clone());
            let next_rule = find_matching_rules(input, &jump_target).next();
            if next_rule.is_none() { return dbg!{None}; }
            let (next_workflow, next_index) = next_rule.unwrap();
            //dbg!{&next_workflow.name};
            find_quad_recursive(input, stop_name, next_workflow, next_index, true, current_quad)
        }
    } else {
        find_quad_recursive(input, stop_name, current_workflow, current_index - 1, false, current_quad)
    }
}

fn find_matching_rules<'w>(input: &'w InputRef, jump_target: &'w JumpTarget) -> impl Iterator<Item = (&'w Workflow, usize)> + 'w {
    input.0.iter().flat_map(move |(_, workflow)| {
        workflow.rules.iter().enumerate().filter_map(move |(i, rule)| {
            let accepts = if let Rule::Jump(ref target) = rule {
                target == jump_target
            } else if let Rule::Comparison(rule) = rule {
                &rule.jump_target == jump_target
            } else {
                false
            };

            if accepts {
                Some((workflow, i))
            } else {
                None
            }
        })
    })
}

#[aoc(day19, part2)]
fn solve_part2(input: &InputRef) -> u64 {
    // Going to assume that this is a acyclic tree

    let mut quads = Vec::new();

    for (workflow, i) in find_matching_rules(input, &JumpTarget::Accept) {
        //dbg!{&workflow.name, i};
        let quad = find_quad(input, workflow, i);
        if let Some(quad) = quad {
            //dbg!{&workflow.name, i, &quad};
            quads.push(quad);
        }
    }

    //dbg!{&quads};

    println!("finding volume of {} quads", quads.len());

    quad_union_volume(&quads)
}
fn get_boundaries<F: Fn(&Quad) -> &RangeInclusive<u32>>(quads: &[Quad], key: F) -> Vec<u32> {
    let mut boundaries: Vec<_> = quads.iter().flat_map(|q| [*key(q).start(), *key(q).end() + 1]).collect();
    boundaries.sort();
    boundaries.dedup();
    boundaries
}

fn quad_union_volume(quads: &[Quad]) -> u64 {
    let x_boundaries = get_boundaries(quads, |q| &q.x);
    let m_boundaries = get_boundaries(quads, |q| &q.m);
    let a_boundaries = get_boundaries(quads, |q| &q.a);

    //dbg!{&x_boundaries, &m_boundaries, &a_boundaries};

    let x_map: HashMap<_, _> = x_boundaries.iter().enumerate().map(|(i, n)| (*n, i)).collect();
    let m_map: HashMap<_, _> = m_boundaries.iter().enumerate().map(|(i, n)| (*n, i)).collect();
    let a_map: HashMap<_, _> = a_boundaries.iter().enumerate().map(|(i, n)| (*n, i)).collect();

    let mut lines = HashMap::new();

    for quad in quads {
        let x_min = x_map[quad.x.start()];
        let x_max = x_map[&(quad.x.end() + 1)];
        for x_point in x_min..x_max {
            let m_min = m_map[quad.m.start()];
            let m_max = m_map[&(quad.m.end() + 1)];
            for m_point in m_min..m_max {
                let a_min = a_map[quad.a.start()];
                let a_max = a_map[&(quad.a.end() + 1)];
                for a_point in a_min..a_max {
                    let line = lines.entry((x_point, m_point, a_point)).or_insert(Vec::new());
                    line.push(&quad.s);
                }
            }
        }
    }

    lines.into_iter()
        .map(|((x_point, m_point, a_point), s_intervals)| {
            (x_boundaries[x_point + 1] - x_boundaries[x_point]) as u64 *
            (m_boundaries[m_point + 1] - m_boundaries[m_point]) as u64 *
            (a_boundaries[a_point + 1] - a_boundaries[a_point]) as u64 *
            length_of_union(s_intervals) as u64
        })
        .sum()
}

fn length_of_union(mut intervals: Vec<&RangeInclusive<u32>>) -> u32 {
    intervals.sort_by(|a, b| a.start().cmp(b.start()));
    let mut current_interval = None;

    let mut length = 0;

    for interval in intervals {
        if current_interval.is_none() {
            current_interval = Some(interval.clone());
        } else {
            if current_interval.as_ref().unwrap().start() <= interval.end() && interval.start() <= current_interval.as_ref().unwrap().end() {
                // overlaps, merge them
                current_interval = Some(
                    min(*current_interval.as_ref().unwrap().start(), *interval.start())..=
                    max(*current_interval.as_ref().unwrap().end(), *interval.end())
                )
            } else {
                let i = current_interval.unwrap();
                length += i.end() - i.start() + 1;
                current_interval = Some(interval.clone());
            }
        }
    }

    if let Some(current_interval) = current_interval {
        let i = current_interval;
        length += i.end() - i.start() + 1;
    }

    length
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 19114);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 167409079868000);
    }
}
