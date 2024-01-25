use itertools::Itertools;
use rayon::prelude::*;

use super::{
    intcode::{self, Intcode},
    Error, Result,
};

pub const INPUT_FILE: &str = "inputs/day02/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<i64> {
    let mut program = parse(input)?;
    program[1] = 12;
    program[2] = 2;
    let result = run(program)?;
    Ok(result)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    let original_program = parse(input)?;

    let (a, b, _) = (0..=99)
        .cartesian_product(0..=99)
        .par_bridge()
        .flat_map(|(a, b)| {
            let mut program = original_program.clone();
            program[1] = a;
            program[2] = b;
            run(program).map(|result| (a, b, result))
        })
        .find_any(|&(_, _, result)| result == 19690720)
        .ok_or(Error::search("values not found"))?;

    Ok(100 * a + b)
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i64>().map_err(Error::from))
        .collect()
}

fn run(program: impl AsRef<[i64]>) -> intcode::Result<i64> {
    let mut machine = Intcode::new(program);
    machine.run()?;
    Ok(machine.get_memory().read(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day02/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, 3500)]
    fn test_part1(#[case] which: usize, #[case] expected: i64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let program = parse(&input)?;
        let result = run(program)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
