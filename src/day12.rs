use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::anyhow;

use std::collections::HashMap;

type Data = Record;

#[derive(Debug, Clone)]
struct Record {
    row: Vec<SpringCondition>,
    groups: Vec<usize>,
}


#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}


#[aoc_generator(day12)]
fn input_generator(input: &str) -> anyhow::Result<Vec<Data>> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (row_str, group_str) = line.split_once(' ').ok_or(anyhow!("Expected two parts"))?;
            let row = row_str.chars()
                .map(|c| {
                    use SpringCondition::*;
                    match c {
                        '.' => Ok(Operational),
                        '?' => Ok(Unknown),
                        '#' => Ok(Damaged),
                        _ => Err(anyhow!("Invalid character")),
                    }
                })
                .collect::<Result<_, _>>()?;

            let groups = group_str.split(',')
                .map(|num_str| num_str.parse::<usize>())
                .collect::<Result<_, _>>()?;

            Ok(Record {
                row,
                groups,
            })
        })
        .collect()

}

#[aoc(day12, part1)]
fn solve_part1(input: &[Data]) -> usize {
    input.iter()
        .map(|record| possible_arrangements(&record.row, &record.groups))
        .sum()
}

fn possible_arrangements(row: &[SpringCondition], groups: &[usize]) -> usize {
    if groups.is_empty() {
        if row.iter().all(|&condition| condition != SpringCondition::Damaged) {
            1
        } else {
            0
        }
    } else {
        if minimum_group_length(groups) > row.len() {
            0
        } else {
            let mut total = 0;
            if group_fits(groups[0], row) {
                if row.len() == groups[0] {
                    total += possible_arrangements(&[], &groups[1..]);
                } else {
                    total += possible_arrangements(&row[groups[0] + 1..], &groups[1..]);
                }
            }

            if row[0] != SpringCondition::Damaged {
                total += possible_arrangements(&row[1..], groups);
            }

            total
        }
    }
}

fn minimum_group_length(groups: &[usize]) -> usize {
    groups.iter().cloned().sum::<usize>() + groups.len() - 1
}

fn group_fits(n: usize, row: &[SpringCondition]) -> bool {
    if row.len() < n {
        false
    } else if row.len() == n {
        row.iter().take(n).all(|&condition| condition != SpringCondition::Operational)
    } else {
        row.iter().take(n).all(|&condition| condition != SpringCondition::Operational) &&
            row[n] != SpringCondition::Damaged
    }
}

#[aoc(day12, part2)]
fn solve_part2(input: &[Data]) -> usize {
    let mut total = 0;
    for (i, record) in
    input.iter()
        .map(|record| {
            let mut row = Vec::with_capacity(record.row.len() * 5);
            for i in 0..5 {
                if i != 0 { row.push(SpringCondition::Unknown); }
                row.extend(&record.row);
            }

            let mut groups = Vec::with_capacity(record.groups.len() * 5);
            for _ in 0..5 {
                groups.extend(&record.groups);
            }
            Record {
                row,
                groups,
            }
        }).enumerate() {
        //.map(|record| possible_arrangements(&record.row, &record.groups))

        let mut subproblems = HashMap::new();
        total += possible_arrangements_memoized(&record.row, &record.groups, &mut subproblems);
        if i % 10 == 0 { dbg!{total}; }
    }
        //.sum()
    total
}

fn possible_arrangements_memoized<'a>(row: &'a[SpringCondition], groups: &'a[usize], subproblems: & mut HashMap<(&'a [SpringCondition], &'a [usize]), usize>) -> usize {
    if subproblems.contains_key(&(row, groups)) {
        return subproblems[&(row, groups)];
    }
    let result = if groups.is_empty() {
        if row.iter().all(|&condition| condition != SpringCondition::Damaged) {
            1
        } else {
            0
        }
    } else {
        if minimum_group_length(groups) > row.len() {
            0
        } else {
            let mut total = 0;
            if group_fits(groups[0], row) {
                if row.len() == groups[0] {
                    total += possible_arrangements_memoized(&[], &groups[1..], subproblems);
                } else {
                    total += possible_arrangements_memoized(&row[groups[0] + 1..], &groups[1..], subproblems);
                }
            }

            if row[0] != SpringCondition::Damaged {
                total += possible_arrangements_memoized(&row[1..], groups, subproblems);
            }

            total
        }
    };
    subproblems.insert((row, groups), result);
    result
}


#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 525152);
    }
}
