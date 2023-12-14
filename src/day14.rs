use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use ndarray::s;
use ndarray::Array2;
use ndarray::ArrayViewMut1;
use ndarray::ShapeBuilder;

use std::collections::VecDeque;
#[cfg(test)]
use std::io::stdout;

type Input = Data;
type InputRef = Data;
type Data = Dish;
type Dish = Array2<Space>;

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
enum Space {
    Rounded,
    Cube,
    Empty
}


#[aoc_generator(day14)]
fn input_generator(input: &str) -> Result<Input> {
    let (input, result) = parse_input(input).map_err(|err| err.to_owned())?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

use nom::IResult;
use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::branch::alt;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = parse_dish(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_dish(input: &str) -> IResult<&str, Dish> {
    let (input, row_data) = separated_list1(tag("\n"),
        many1(parse_space)
    )(input)?;

    let cols = row_data[0].len();
    let rows = row_data.len();

    let data: Vec<_> = row_data.into_iter().flatten().collect();
    let shape = (rows, cols).strides((cols, 1));

    Ok((input, Array2::from_shape_vec(shape, data).unwrap()))
}

fn parse_space(input: &str) -> IResult<&str, Space> {
    let (input, c) = alt((tag("#"), tag("O"), tag(".")))(input)?;

    use Space::*;
    let space = match c {
        "#" => Cube,
        "O" => Rounded,
        "." => Empty,
        _ => unreachable!(),
    };

    Ok((input, space))
}

#[aoc(day14, part1)]
fn solve_part1(input: &InputRef) -> usize {
    let mut dish = input.clone();

    tilt_north(&mut dish);

    north_load(&dish)
}

fn tilt_north(dish: &mut Dish) {
    for column in dish.columns_mut() {
        tilt_slice(column);
    }
}

fn tilt_west(dish: &mut Dish) {
    for row in dish.rows_mut() {
        tilt_slice(row);
    }
}

fn tilt_south(dish: &mut Dish) {
    for mut column in dish.columns_mut() {
        tilt_slice(column.slice_mut(s![..;-1]));
    }
}

fn tilt_east(dish: &mut Dish) {
    for mut row in dish.rows_mut() {
        tilt_slice(row.slice_mut(s![..;-1]));
    }
}

fn tilt_slice(mut slice: ArrayViewMut1<Space>) {
    let mut first_open_space = 0;

    for i in 0..slice.len() {
        use Space::*;
        match slice[i] {
            Cube => first_open_space = i + 1,
            Rounded => {
                slice[i] = Empty;
                slice[first_open_space] = Rounded;
                first_open_space += 1;
            },
            Empty => { },
        }
    }
}

fn north_load(dish: &Dish) -> usize {
    let mut total = 0;

    let shape = dish.shape();
    let rows = shape[0];
    for (i, row) in dish.rows().into_iter().enumerate() {
        let dist = rows - i;

        let load = row.iter().filter(|&&s| s == Space::Rounded).count();

        total += load * dist;
    }

    total
}

fn spin_cycle(dish: &mut Dish) {
    tilt_north(dish);
    tilt_west(dish);
    tilt_south(dish);
    tilt_east(dish);
}

#[aoc(day14, part2)]
fn solve_part2(input: &InputRef) -> usize {
    let mut dish = input.clone();

    let mut previous_dish_queue: VecDeque<Dish> = VecDeque::new();
    for cycles in 0..1_000_000_000 {
        let cycles = cycles + 1;
        if previous_dish_queue.len() == 100 {
            previous_dish_queue.rotate_right(1);
            previous_dish_queue[0].assign(&dish);
        } else {
            previous_dish_queue.push_front(dish.clone());
        }

        spin_cycle(&mut dish);

        for (i, previous_dish) in previous_dish_queue.iter().enumerate() {
            if previous_dish == dish {
                let cycle_length = i + 1;

                let cycle_offset = (1_000_000_000 - cycles) % cycle_length;

                #[cfg(test)]
                {
                    dbg!(cycles, cycle_length, cycle_offset);

                    for dish in previous_dish_queue.iter().take(cycle_length).rev() {
                        pretty_print(dish);
                        use std::io::Write;
                        writeln!(stdout(), "{}", north_load(&dish)).unwrap();
                    }

                    pretty_print(&dish);
                    use std::io::Write;
                    writeln!(stdout(), "{}", north_load(&dish)).unwrap();

                    dbg!{north_load(&dish)};
                }

                return north_load(&previous_dish_queue[(i + cycle_length - cycle_offset) % cycle_length]);
            }
        }
    }

    north_load(&dish)
}

#[cfg(test)]
fn pretty_print(dish: &Dish) {
    let mut stdout = stdout().lock();

    for row in dish.rows() {
        use std::io::Write;
        for c in row.iter() {
            use Space::*;
            let c = match c {
                Rounded => 'O',
                Cube => '#',
                Empty => '.',
            };
            write!(stdout, "{}", c).unwrap();
        }
        writeln!(stdout, "").unwrap();
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        dbg!{&input};
        let result = super::solve_part1(&input);

        assert_eq!(result, 136);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 64);
    }
}
