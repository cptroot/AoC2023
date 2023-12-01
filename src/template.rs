use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Data = usize;


#[aoc_generator(dayX)]
fn input_generator(input: &str) -> Result<Vec<Data>> {
    unimplemented!()
}

#[aoc(dayX, part1)]
fn solve_part1(input: &[Data]) -> usize {
    unimplemented!()
}

#[aoc(dayX, part2)]
fn solve_part2(input: &[Data]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        //assert_eq!(result, None);
        assert!(false);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        //assert_eq!(result, None);
        assert!(false);
    }
}
