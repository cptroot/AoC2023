use std::collections::{HashMap, HashSet};
use std::collections::BTreeMap;
use std::collections::VecDeque;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = (Vec<ModuleName>, Vec<Module>);
type InputRef = Input;

#[derive(Debug, Clone)]
struct ModuleName(String);

#[derive(Debug, Clone)]
struct Module {
    name: ModuleIndex,
    module_type: ModuleType,
    connections: Vec<ModuleIndex>,
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
struct ModuleIndex(usize);

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}


#[aoc_generator(day20)]
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
use nom::multi::separated_list1;
use nom::bytes::complete::tag;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, str_modules) = separated_list1(tag("\n"), parse_module)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    let mut module_names: Vec<_> = str_modules.iter()
        .map(|module| ModuleName(module.name.to_owned()))
        .collect();

    let mut module_name_map: HashMap<&str, ModuleIndex> = str_modules.iter().enumerate()
        .map(|(i, module)| (module.name, ModuleIndex(i)))
        .collect();

    for module in &str_modules {
        for connection in &module.connections {
            if !module_name_map.contains_key(connection) {
                module_names.push(ModuleName(connection.to_string()));
                module_name_map.insert(connection, ModuleIndex(module_names.len() - 1));
            }
        }
    }

    let modules = str_modules.into_iter()
        .map(|module| {
            Module {
                name: module_name_map[module.name],
                module_type: module.module_type,
                connections: module.connections.into_iter().map(|s| module_name_map[s]).collect(),
            }
        })
        .collect();

    Ok((input, (module_names, modules)))
}

struct StrModule<'input> {
    name: &'input str,
    module_type: ModuleType,
    connections: Vec<&'input str>,
}

fn parse_module(input: &str) -> IResult<&str, StrModule<'_>> {
    let (input, module_type) = parse_module_type(input)?;
    let (input, name) = parse_name(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, connections) = separated_list1(tag(", "), parse_name)(input)?;

    Ok((input, StrModule {
        name,
        module_type,
        connections,
    }))
}

fn parse_module_type(input: &str) -> IResult<&str, ModuleType> {
    let (input, c) = opt(one_of("%&"))(input)?;

    let module_type = match c {
        Some('%') => ModuleType::FlipFlop,
        Some('&') => ModuleType::Conjunction,
        None => ModuleType::Broadcast,
        _ => unreachable!(),
    };

    Ok((input, module_type))
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| ('a'..='z').contains(&c))(input)
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
enum ModuleState {
    FlipFlop { on: bool },
    Conjunction {
        connection_states: BTreeMap<ModuleIndex, Pulse>,
    },
    Broadcast,
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
enum Pulse {
    Low,
    High,
}

#[aoc(day20, part1)]
fn solve_part1(input: &InputRef) -> usize {
    let mut module_states: Vec<_> = input.1.iter()
        .map(|module| {
            use ModuleType::*;
            match module.module_type {
                Broadcast => ModuleState::Broadcast,
                FlipFlop => ModuleState::FlipFlop { on: false },
                Conjunction => {
                    let connection_states: BTreeMap<_, _> = input.1.iter().enumerate()
                        .filter(|(_, other_module)| other_module.connections.contains(&module.name))
                        .map(|(i, _)| (ModuleIndex(i), Pulse::Low))
                        .collect();

                    ModuleState::Conjunction { connection_states, }
                },
            }
        })
        .collect();

    let button_index = ModuleIndex(input.0.len());
    let broadcast_index = input.0.iter().enumerate()
        .filter(|(_, name)| name.0 == "broadcaster")
        .map(|(i, _)| ModuleIndex(i))
        .next()
        .unwrap();


    let mut pulse_counts = HashMap::new();
    pulse_counts.insert(Pulse::Low, 0);
    pulse_counts.insert(Pulse::High, 0);

    for _ in 0..1000 {
        let mut pulses = VecDeque::new();

        pulses.push_back((button_index, Pulse::Low, broadcast_index));

        while let Some((source, pulse, dest)) = pulses.pop_front() {
            /*
            let source_name = if source.0 == input.0.len() {
                "button"
            } else {
                &input.0[source.0].0
            };
            dbg!{source_name, pulse, &input.0[dest.0].0};
            */
            *pulse_counts.get_mut(&pulse).unwrap() += 1;
            if input.1.len() <= dest.0 { continue; }
            let dest_module = &input.1[dest.0];

            use ModuleType::*;
            let pulse_to_send = match dest_module.module_type {
                Broadcast => Some(pulse),
                FlipFlop => {
                    if pulse == Pulse::Low {
                        if let ModuleState::FlipFlop { on } = &mut module_states[dest.0] {
                            *on = !*on;

                            if *on {
                                Some(Pulse::High)
                            } else {
                                Some(Pulse::Low)
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                Conjunction => {
                    if let ModuleState::Conjunction { connection_states } = &mut module_states[dest.0] {
                        *connection_states.get_mut(&source).unwrap() = pulse;
                        let all_high = connection_states.iter().all(|(_, &pulse_state)| pulse_state == Pulse::High);
                        if all_high {
                            Some(Pulse::Low)
                        } else {
                            Some(Pulse::High)
                        }
                    } else {
                        None
                    }
                },
            };

            if let Some(pulse_to_send) = pulse_to_send {
                for &connection in &dest_module.connections {
                    pulses.push_back((dest_module.name, pulse_to_send, connection));
                }
            }
        }
    }

    //dbg!{&pulse_counts};
    pulse_counts[&Pulse::Low] * pulse_counts[&Pulse::High]
}

#[aoc(day20, part2)]
fn solve_part2(input: &InputRef) -> usize {
    //render_graph(input).unwrap();
    let mut module_states: Vec<_> = input.1.iter()
        .map(|module| {
            use ModuleType::*;
            match module.module_type {
                Broadcast => ModuleState::Broadcast,
                FlipFlop => ModuleState::FlipFlop { on: false },
                Conjunction => {
                    let connection_states: BTreeMap<_, _> = input.1.iter().enumerate()
                        .filter(|(_, other_module)| other_module.connections.contains(&module.name))
                        .map(|(i, _)| (ModuleIndex(i), Pulse::Low))
                        .collect();

                    ModuleState::Conjunction { connection_states, }
                },
            }
        })
        .collect();

    let button_index = ModuleIndex(input.0.len());
    let broadcast_index = input.0.iter().enumerate()
        .filter(|(_, name)| name.0 == "broadcaster")
        .map(|(i, _)| ModuleIndex(i))
        .next()
        .unwrap();
    let rx_index = input.0.iter().enumerate()
        .filter(|(_, name)| name.0 == "rx")
        .map(|(i, _)| ModuleIndex(i))
        .next()
        .unwrap();

    let rx_predecessor = get_predecessors(input, rx_index)[0];
    let rx_pred_preds = get_predecessors(input, rx_predecessor);
    let keys: HashSet<_> = rx_pred_preds.into_iter()
        .map(|predecessor| get_predecessors(input, predecessor)[0])
        .collect();

    let mut key_cycle_lengths = HashMap::new();

    dbg!{&keys};

    for i in 1.. {
        let mut pulses = VecDeque::new();

        pulses.push_back((button_index, Pulse::Low, broadcast_index));

        while let Some((source, pulse, dest)) = pulses.pop_front() {
            if pulse == Pulse::Low && dest == rx_index {
                return i;
            }

            if dest.0 >= input.1.len() { continue; }
            let dest_module = &input.1[dest.0];

            use ModuleType::*;
            let pulse_to_send = match dest_module.module_type {
                Broadcast => Some(pulse),
                FlipFlop => {
                    if pulse == Pulse::Low {
                        if let ModuleState::FlipFlop { on } = &mut module_states[dest.0] {
                            *on = !*on;

                            if *on {
                                Some(Pulse::High)
                            } else {
                                Some(Pulse::Low)
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                Conjunction => {
                    if let ModuleState::Conjunction { connection_states } = &mut module_states[dest.0] {
                        *connection_states.get_mut(&source).unwrap() = pulse;
                        let all_high = connection_states.iter().all(|(_, &pulse_state)| pulse_state == Pulse::High);
                        if all_high {
                            if keys.contains(&dest) {
                                println!("{}: {:?}", i, dest);
                                key_cycle_lengths.insert(dest, i);
                                if key_cycle_lengths.len() == keys.len() {
                                    use num::Integer;
                                    
                                    return key_cycle_lengths.iter()
                                        .map(|(_, length)| *length)
                                        .reduce(|a, b| a.lcm(&b)).unwrap();
                                }
                            }
                            Some(Pulse::Low)
                        } else {
                            Some(Pulse::High)
                        }
                    } else {
                        None
                    }
                },
            };

            if let Some(pulse_to_send) = pulse_to_send {
                for &connection in &dest_module.connections {
                    pulses.push_back((dest_module.name, pulse_to_send, connection));
                }
            }
        }
    }

    unreachable!()
}

fn get_predecessors(input: &InputRef, index: ModuleIndex) -> Vec<ModuleIndex> {
    input.1.iter().enumerate()
        .filter(|(_, other_module)| other_module.connections.contains(&index))
        .map(|(_, other_module)| other_module.name)
        .collect()
}

#[allow(unused)]
fn render_graph(input: &InputRef) -> Result<()> {
    let file = std::fs::File::create("graph.gz")?;
    let mut writer = std::io::BufWriter::new(file);
    use std::io::Write;

    writeln!(writer, "digraph day20 {{")?;
    for module in &input.1 {
        let source_label = &input.0[module.name.0].0;
        for connection in &module.connections {
            if connection.0 < input.1.len() {
                let dest = &input.1[connection.0];
                let dest_label = &input.0[dest.name.0].0;
                if dest.module_type == ModuleType::Conjunction {
                    writeln!(writer, "  {} -> {} [color = orange]", source_label, dest_label)?;
                } else {
                    writeln!(writer, "  {} -> {}", source_label, dest_label)?;
                }
            } else {
                let dest_label = &input.0[connection.0].0;
                writeln!(writer, "  {} -> {}", source_label, dest_label)?;
            }
        }
    }
    writeln!(writer, "}}")?;
    Ok(())
}

#[cfg(test)]
mod test {
    const TEST_INPUT_REPEAT: &'static str =
r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;

    const TEST_INPUT_INTERESTING: &'static str =
r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT_REPEAT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 32000000);

        let input = super::input_generator(TEST_INPUT_INTERESTING).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 11687500);
    }
}
