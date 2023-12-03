use std::collections::HashMap;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;

#[derive(Debug, Clone)]
struct Data {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[derive(Debug, Clone)]
struct Symbol {
    position: Pos,
    #[allow(unused)]
    value: char,
}

#[derive(Debug, Clone)]
struct Number {
    position: Pos,
    length: i32,
    value: usize,
}

#[derive(Debug, Clone)]
struct Pos {
    x: i32,
    y: i32,
}


#[aoc_generator(day3)]
fn input_generator(input: &str) -> Result<Data> {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        if line.is_empty() { continue; }


        let mut remaining_line = line.as_bytes();

        while !remaining_line.is_empty() {
            let c = remaining_line[0];
            let position = Pos {
                x: (line.len() - remaining_line.len()) as i32,
                y: y as i32,
            };

            if is_digit(c) {
                let (input, length, value) = parse_usize(remaining_line);
                let new_number = Number {
                    position,
                    length,
                    value,
                };
                numbers.push(new_number);

                remaining_line = input;
            } else {
                if c != b'.' {
                    let new_symbol = Symbol {
                        position,
                        value: c as char,
                    };
                    symbols.push(new_symbol);
                }

                remaining_line = &remaining_line[1..];
            }
        }
    }


    Ok(Data {
        symbols,
        numbers,
    })
}

fn is_digit(c: u8) -> bool {
    let c = c as char;
    c.is_digit(10)
}

fn parse_usize(input: &[u8]) -> (&[u8], i32, usize) {
    let mut i = 1;

    while i < input.len() {
        if !is_digit(input[i]) { break; }
        i += 1;
    }

    let num_str = &input[0..i];
    let num_str = std::str::from_utf8(num_str).unwrap();
    let length = i as i32;

    (&input[i..], length, num_str.parse().unwrap())
}

#[aoc(day3, part1)]
fn solve_part1(input: &Data) -> usize {
    let mut total = 0;

    for number in &input.numbers {
        if is_any_symbol_adjacent(number, &input.symbols) {
            total += number.value;
        }
    }

    total
}

fn is_any_symbol_adjacent(number: &Number, symbols: &[Symbol]) -> bool {
    for symbol in symbols {
        if is_this_symbol_adjacent(number, symbol) { return true; }
    }

    false
}

fn is_this_symbol_adjacent(number: &Number, symbol: &Symbol) -> bool {
    (number.position.x - 1 ..= number.position.x + number.length).contains(&symbol.position.x)
        && number.position.y.abs_diff(symbol.position.y) <= 1
}

#[derive(Debug, Clone, Copy)]
enum SymbolSlot {
    NoneFound,
    OneFound(usize),
    TwoFound(usize),
}

#[aoc(day3, part2)]
fn solve_part2(input: &Data) -> usize {
    let mut symbol_map = HashMap::new();

    use SymbolSlot::*;

    for (i, _symbol) in input.symbols.iter().enumerate() {
        symbol_map.insert(i, NoneFound);
    }

    for number in &input.numbers {
        let symbol_index = find_adjacent_symbol(number, &input.symbols);

        if let Some(symbol_index) = symbol_index {
            let entry = symbol_map.get_mut(&symbol_index).unwrap();
            match *entry {
                NoneFound => *entry = OneFound(number.value),
                OneFound(value) => *entry = TwoFound(value * number.value),
                TwoFound(_) => unreachable!(),
            }
        }
    }

    symbol_map.iter()
        .map(|(_, v)| {
            match v {
                &TwoFound(value) => value,
                _ => 0,
            }
        })
        .sum()
}

fn find_adjacent_symbol(number: &Number, symbols: &[Symbol]) -> Option<usize> {
    for (i, symbol) in symbols.iter().enumerate() {
        if is_this_symbol_adjacent(number, symbol) { return Some(i); }
    }

    None
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 467835);
    }
}
