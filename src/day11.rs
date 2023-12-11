use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

use ndarray::Array2;
use ndarray::ShapeBuilder;

type Data = (Vec<(usize, usize)>, Array2<bool>);


#[aoc_generator(day11)]
fn input_generator(input: &str) -> Result<Data> {
    let mut rows = Vec::new();
    let mut rowlength = None;

    let mut galaxies = Vec::new();

    for (j, line) in input.lines().filter(|line| !line.is_empty()).enumerate()  {
        let mut row = Vec::new();
        for (i, c) in line.char_indices() {
            let cell = match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(anyhow!("Invalid character")),
            }?;

            if c == '#' {
                galaxies.push((i, j));
            }

            row.push(cell);
        }

        if let None = rowlength { rowlength = Some(row.len()); }
        rows.extend(row);
    }

    let rowlength = rowlength.unwrap();
    let shape = (rowlength, rows.len() / rowlength).strides((1, rowlength));

    Ok((galaxies, Array2::from_shape_vec(shape, rows).unwrap()))
}

#[aoc(day11, part1)]
fn solve_part1(input: &Data) -> usize {
    solve_multiplied_expansion(input, 2)
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    b.0.abs_diff(a.0) + b.1.abs_diff(a.1)
}

#[aoc(day11, part2)]
fn solve_part2(input: &Data) -> usize {
    solve_multiplied_expansion(input, 1_000_000)
}

fn solve_multiplied_expansion((galaxies, array): &Data, multiply: usize) -> usize {
    let skiprows: Vec<_> = array.columns().into_iter()
        .enumerate()
        .filter(|(_, col)| {
            col.fold(true, |a, &b| a && !b)
        })
        .map(|(i, _)| i)
        .collect();
    let skipcols: Vec<_> = array.rows().into_iter()
        .enumerate()
        .filter(|(_, row)| {
            row.fold(true, |a, &b| a && !b)
        })
        .map(|(i, _)| i)
        .collect();

    let modified_galaxies: Vec<_> = galaxies.iter()
        .map(|(i, j)| {
            let skipped_columns = skipcols.iter()
                .filter(|&n| n < i)
                .count();
            let skipped_rows = skiprows.iter()
                .filter(|&n| n < j)
                .count();

            let i = i + (multiply - 1) * skipped_columns;
            let j = j + (multiply - 1) * skipped_rows;

            (i, j)
        })
        .collect();

    let mut distance = 0;

    for (index, a) in modified_galaxies.iter().enumerate() {
        for b in modified_galaxies[index + 1..].iter() {
            distance += manhattan_distance(a, b);
        }
    }

    distance
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 374);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_multiplied_expansion(&input, 10);

        assert_eq!(result, 1030);

        let result = super::solve_multiplied_expansion(&input, 100);

        assert_eq!(result, 8410);
    }
}
