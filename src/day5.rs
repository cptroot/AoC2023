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

    fn map_ranges(&self, ranges: Vec<RangeType>) -> Vec<RangeType> {
        let mut remaining_ranges = ranges;

        let starting_length: usize = remaining_ranges.iter().map(|r| r.length).sum();

        let mut output_ranges = Vec::new();

        for map_range in &self.ranges {
            remaining_ranges = remaining_ranges.into_iter().flat_map(|range| {
                let (rem, out) = map_range.try_map_range(range);
                output_ranges.extend(out);
                rem
            }).collect();

            let remaining_length: usize = remaining_ranges.iter().map(|r| r.length).sum();
            let output_length: usize = output_ranges.iter().map(|r| r.length).sum();
            assert_eq!(starting_length, remaining_length + output_length);
        }

        output_ranges.extend(remaining_ranges.into_iter());

        output_ranges
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

    fn source_range(&self) -> RangeType {
        RangeType {
            start: self.source_start,
            length: self.length,
        }
    }

    fn try_map_range(&self, range: RangeType) -> (impl Iterator<Item = RangeType>, impl Iterator<Item = RangeType>) {
        let contains_start = self.source_range().to_range().contains(&range.start);
        let contains_end = self.source_range().to_range().contains(&(range.end() - 1));

        match (contains_start, contains_end) {
            (true, true) => {
                // This range fully contains the source values, map the whole range
                (vec![].into_iter(), vec![RangeType {
                    start: range.start - self.source_start + self.dest_start,
                    length: range.length,
                }].into_iter())
            },
            (true, false) => {
                // This range overlaps at the start
                let first_untransformed = self.source_start + self.length;
                // exclusive
                let last_untransformed = range.end();
                (
                    vec![RangeType {
                            start: first_untransformed,
                            length: last_untransformed - first_untransformed,
                    }].into_iter(),
                    vec![RangeType {
                            start: range.start - self.source_start + self.dest_start,
                            length: first_untransformed - range.start,
                    }].into_iter()
                )
            },
            (false, true) => {
                // This range overlaps at the end
                let first_transformed = self.source_start;
                // exclusive
                let last_transformed = range.start + range.length;
                (
                    vec![RangeType {
                            start: range.start,
                            length: first_transformed - range.start,
                    }].into_iter(),
                    vec![RangeType {
                            start: first_transformed - self.source_start + self.dest_start,
                            length: last_transformed - first_transformed,
                    }].into_iter(),
                )
            },
            (false, false) => {
                // either we're inside the start range or we're not
                if self.source_start > range.start && self.source_range().end() < range.end() {
                    // Contained by the input slice :<, two untranslated halves
                    let first_transformed = self.source_start;
                    // exclusive
                    let last_transformed = self.source_range().end();

                    (
                        vec![
                            RangeType {
                                start: range.start,
                                length: first_transformed - range.start,
                            },
                            RangeType {
                                start: last_transformed,
                                length: range.end() - last_transformed,
                            }
                        ].into_iter(),
                        vec![RangeType {
                                start: first_transformed - self.source_start + self.dest_start,
                                length: last_transformed - first_transformed,
                        }].into_iter()
                    )
                } else {
                    // No overlap
                    (
                        vec![range].into_iter(),
                        vec![].into_iter()
                    )
                }

            },
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct RangeType {
    start: usize,
    length: usize,
}

impl RangeType {
    fn to_range(&self) -> std::ops::Range<usize> {
        self.start..self.end()
    }
    fn end(&self) -> usize {
        self.start + self.length
    }
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
    let ranges: Vec<_> = input.seeds
        .chunks(2)
        .map(|arr| RangeType { start: arr[0].0, length: arr[1].0 })
        .collect();

    let start_type = &input.starting_type;
    let end_type = &ItemType("location".to_owned());

    let final_ranges = find_mapped_ranges(input, start_type, end_type, ranges);

    let minimum_location = final_ranges.iter()
        .map(|range| range.start)
        .min()
        .unwrap();

    minimum_location
}

fn find_mapped_ranges(data: &Data, start_type: &ItemType, end_type: &ItemType, ranges: Vec<RangeType>) -> Vec<RangeType> {
    let mut current_type = start_type;
    let mut current_ranges = ranges;

    while current_type != end_type {
        dbg!{current_type};
        let map = &data.maps[current_type];
        let next_ranges = map.map_ranges(current_ranges);
        let next_type = &map.result_type;

        current_type = next_type;
        current_ranges = next_ranges;
    }

    current_ranges
}

#[cfg(test)]
mod test {
    use super::RangeType;
    use super::MapRange;

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

        assert_eq!(result, 46);
    }

    fn run_test(map_range: &MapRange, range: RangeType) -> (Vec<RangeType>, Vec<RangeType>) {
        let (a, b) = map_range.try_map_range(range);

        (a.collect(), b.collect())
    }
    
    #[test]
    fn test_map_range() {
        let map_range = MapRange {
            source_start: 20,
            dest_start: 40,
            length: 20,
        };

        let range = RangeType {
            start: 10,
            length: 0,
        };
        let (remaining, output) = run_test(&map_range, range);
        assert_eq!(remaining, vec![range]);
        assert_eq!(output, vec![]);

        let range = RangeType {
            start: 40,
            length: 10,
        };
        let (remaining, output) = run_test(&map_range, range);
        assert_eq!(remaining, vec![range]);
        assert_eq!(output, vec![]);

        let range = RangeType {
            start: 10,
            length: 40,
        };
        let (remaining, output) = run_test(&map_range, range);
        assert_eq!(remaining, vec![RangeType { start: 10, length: 10}, RangeType { start: 40, length: 10 }]);
        assert_eq!(output, vec![RangeType { start: 40, length: 20}]);

        let range = RangeType {
            start: 20,
            length: 10,
        };
        let (remaining, output) = run_test(&map_range, range);
        assert_eq!(remaining, vec![]);
        assert_eq!(output, vec![RangeType { start: 40, length: 10}]);
    }
}
