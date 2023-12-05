use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
struct Data {
    seeds: Vec<Item>,
    starting_type: ItemType,
    maps: HashMap<ItemType, ItemMap>,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Item(usize);

#[derive(Clone)]
#[derive(Debug)]
struct ItemMap {
    result_type: ItemType,
    ranges: Vec<MapRange>,
}

impl ItemMap {
    fn map_value(&self, item: &Item) -> Item {
        for range in &self.ranges {
            let result = range.try_map_value(item);

            if let Some(result) = result {
                return result;
            }
        }

        *item
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct MapRange {
    source_start: usize,
    dest_start: usize,
    length: usize,
}

impl MapRange {
    fn try_map_value(&self, item: &Item) -> Option<Item> {
        if (self.source_start..self.source_start + self.length).contains(&item.0) {
            Some(Item(item.0 - self.source_start + self.dest_start))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
struct RangeType {
    start: usize,
    length: usize,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct ItemType(String);


#[aoc_generator(day5)]
fn input_generator(input: &str) -> Result<Data> {
    let (input, result) = parse_almanac(input).map_err(|err| err.to_owned())?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

use nom::IResult;
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::sequence::separated_pair;

fn parse_almanac(input: &str) -> IResult<&str, Data> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, vec_of_maps) = separated_list1(tag("\n\n"), parse_map)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    let maps: HashMap<_, _> = vec_of_maps.into_iter().collect();

    Ok((input, Data {
        seeds,
        starting_type: ItemType("seed".to_owned()),
        maps,
    }))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Item>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), parse_item)(input)?;

    Ok((input, seeds))
}

fn parse_map(input: &str) -> IResult<&str, (ItemType, ItemMap)> {
    let (input, source) = parse_item_type(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, dest) = parse_item_type(input)?;
    let (input, _) = tag(" map:\n")(input)?;

    let (input, ranges) = separated_list1(tag("\n"), parse_range)(input)?;

    Ok((input, (source, ItemMap {
        result_type: dest,
        ranges,
    })))
}

fn parse_range(input: &str) -> IResult<&str, MapRange> {
    let (input, dest_start) = parse_usize(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, source_start) = parse_usize(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, length) = parse_usize(input)?;

    Ok((input, MapRange {
        source_start,
        dest_start,
        length,
    }))
}

fn parse_item_type(input: &str) -> IResult<&str, ItemType> {
    map(take_while1(|c| c != '-' && c != ' '), |s: &str| ItemType(s.to_owned()))(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    map(parse_usize, |n| Item(n))(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[aoc(day5, part1)]
fn solve_part1(input: &Data) -> usize {
    let start_type = &input.starting_type;
    let end_type = &ItemType("location".to_owned());

    let minimum_location = input.seeds.iter().map(|seed| {
        find_mapped_value(input, start_type, end_type, *seed).0
    }).min().unwrap();

    minimum_location
}

fn find_mapped_value(data: &Data, start_type: &ItemType, end_type: &ItemType, value: Item) -> Item {
    let mut current_type = start_type;
    let mut current_value = value;

    while current_type != end_type {
        let map = &data.maps[current_type];
        let next_value = map.map_value(&current_value);
        let next_type = &map.result_type;

        current_type = next_type;
        current_value = next_value;
    }

    current_value
}

#[aoc(day5, part2)]
fn solve_part2(input: &Data) -> usize {
    let pairs: Vec<_> = input.seeds
        .chunks(2)
        .map(|[start, length]| RangeType { start, length })
        .collect();
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        dbg!{&input};
        let result = super::solve_part1(&input);

        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        //assert_eq!(result, None);
        assert!(false);
    }
}
