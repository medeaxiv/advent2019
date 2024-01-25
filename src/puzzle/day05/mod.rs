use super::{intcode::Intcode, Error, Result};

pub const INPUT_FILE: &str = "inputs/day05/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<i64> {
    let program = parse(input)?;
    let outputs = Intcode::run_program_with_inputs(program, [1])?;
    Ok(outputs[outputs.len() - 1])
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    let program = parse(input)?;
    let outputs = Intcode::run_program_with_inputs(program, [5])?;
    Ok(outputs[outputs.len() - 1])
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()
}
